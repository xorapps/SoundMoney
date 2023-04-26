use geyser_service_common::{BillableItem, ServiceCommand};
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    ReplicaAccountInfoVersions, ReplicaTransactionInfoVersions,
};
use solana_sdk::{signature::Signature, transaction::SanitizedTransaction};
use solana_transaction_status::TransactionStatusMeta;

pub fn exec_command(command: &ServiceCommand, tx_signature: &str) {
    match command {
        ServiceCommand::TxEmail {
            validator: _,
            customer_name,
            email,
            subject,
            items,
            date,
            tx: _, // FIXME Remove this as is not used
        } => {
            email_builder(customer_name, email, subject, tx_signature, date, items);
        }
        ServiceCommand::ThinClientWebVersion {
            validator: _,
            customer_name,
            items,
            tx: _,
            date,
        } => html_builder(customer_name, tx_signature, date, items),
        _ => todo!(),
    }
}

pub fn html_builder(name: &str, tx: &str, date: &str, items: &[BillableItem]) {
    use borsh::BorshSerialize;
    use smol::io::BufWriter;
    use validator_document_service::TemplateBuilder;

    let mut builder = TemplateBuilder::new();
    builder
        .add_customer(name)
        .add_date(date)
        .add_tx(tx)
        .add_items(items.to_vec());

    let as_html_string = builder.to_template_1().unwrap();

    let tx_html = (tx.to_owned(), as_html_string);
    let stream_data = tx_html.try_to_vec().unwrap();

    smol::block_on(async {
        use smol::{io::AsyncWriteExt, net::TcpStream};
        let mut stream = TcpStream::connect("127.0.0.1:6365").await.unwrap();

        let mut buf_writer = BufWriter::with_capacity(4096, &mut stream);
        buf_writer.write_all(&stream_data).await.unwrap();

        buf_writer.flush().await.unwrap();

        log::info!("WRITTEN_HTML: {:?}", stream_data.len());
    })
}

pub fn email_builder(
    name: &str,
    email: &str,
    subject: &str,
    tx: &str,
    date: &str,
    items: &[BillableItem],
) {
    use borsh::BorshSerialize;
    use smol::io::BufWriter;
    use validator_document_service::TemplateBuilder;
    use xor_mailer_common::Envelope;

    let mut builder = TemplateBuilder::new();
    builder
        .add_customer(name)
        .add_date(date)
        .add_tx(tx)
        .add_items(items.to_vec());

    let mut envelope = Envelope::new();
    envelope
        .add_recipient((name, email))
        .add_subject(subject)
        .add_html_body(&builder.to_template_1().unwrap());

    let stream_data = envelope.try_to_vec().unwrap();

    smol::block_on(async {
        use smol::{io::AsyncWriteExt, net::TcpStream};
        let mut stream = TcpStream::connect("127.0.0.1:6363").await.unwrap();

        let mut buf_writer = BufWriter::with_capacity(4096, &mut stream);
        buf_writer.write_all(&stream_data).await.unwrap();

        buf_writer.flush().await.unwrap();

        log::info!("WRITTEN_MAIL_DATA: {:?}", stream_data.len());
    })
}

#[derive(Debug)]
pub enum AccTx {
    Acc {
        pubkey: Vec<u8>,
        lamports: u64,
        owner: Vec<u8>,
        executable: bool,
        rent_epoch: u64,
        data: Vec<u8>,
        write_version: u64,
        txn_signature: Option<Signature>,
        slot: u64,
        is_startup: bool,
    },
    Tx {
        slot: u64,
        signature: Signature,
        is_vote: bool,
        transaction: SanitizedTransaction,
        transaction_status_meta: TransactionStatusMeta,
        index: Option<usize>,
    },
}

impl Default for AccTx {
    fn default() -> Self {
        AccTx::Acc {
            pubkey: Vec::default(),
            lamports: u64::default(),
            owner: Vec::default(),
            executable: bool::default(),
            rent_epoch: u64::default(),
            data: Vec::default(),
            write_version: u64::default(),
            txn_signature: Option::default(),
            slot: u64::default(),
            is_startup: bool::default(),
        }
    }
}

impl AccTx {
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }

    pub fn into_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }

    pub fn is_account(&self) -> bool {
        match self {
            Self::Acc { pubkey, .. } => {
                let pubkey_bytes = bs58::decode("DHeGyhLA6Hr55sVRCQJjkUA7JhG3bAtyXUfCvxuPyhLn")
                    .into_vec()
                    .unwrap();

                if pubkey.as_slice() == pubkey_bytes.as_slice() {
                    true
                } else {
                    false
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn into_acc(slot: u64, is_startup: bool, value: &ReplicaAccountInfoVersions) -> Self {
        match value {
            ReplicaAccountInfoVersions::V0_0_1(inner_account) => Self::Acc {
                pubkey: inner_account.pubkey.to_owned(),
                lamports: inner_account.lamports,
                owner: inner_account.owner.to_owned(),
                executable: inner_account.executable,
                rent_epoch: inner_account.rent_epoch,
                data: inner_account.data.to_owned(),
                write_version: inner_account.write_version,
                txn_signature: Option::default(),
                slot,
                is_startup,
            },
            ReplicaAccountInfoVersions::V0_0_2(inner_account) => Self::Acc {
                pubkey: inner_account.pubkey.to_owned(),
                lamports: inner_account.lamports,
                owner: inner_account.owner.to_owned(),
                executable: inner_account.executable,
                rent_epoch: inner_account.rent_epoch,
                data: inner_account.data.to_owned(),
                write_version: inner_account.write_version,
                txn_signature: inner_account.txn_signature.cloned(),
                slot,
                is_startup,
            },
        }
    }

    pub fn into_tx(slot: u64, value: &ReplicaTransactionInfoVersions) -> Self {
        match value {
            ReplicaTransactionInfoVersions::V0_0_1(inner_tx) => Self::Tx {
                slot,
                signature: inner_tx.signature.to_owned(),
                is_vote: inner_tx.is_vote,
                transaction: inner_tx.transaction.to_owned(),
                transaction_status_meta: inner_tx.transaction_status_meta.to_owned(),
                index: Option::default(),
            },

            ReplicaTransactionInfoVersions::V0_0_2(inner_tx) => Self::Tx {
                slot,
                signature: inner_tx.signature.to_owned(),
                is_vote: inner_tx.is_vote,
                transaction: inner_tx.transaction.to_owned(),
                transaction_status_meta: inner_tx.transaction_status_meta.to_owned(),
                index: Some(inner_tx.index),
            },
        }
    }
}
