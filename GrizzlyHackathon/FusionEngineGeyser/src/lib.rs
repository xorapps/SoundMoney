use borsh::BorshDeserialize;
use geyser_service_common::ServiceCommand;
use lazy_static::lazy_static;
use log::info;
use once_cell::sync::OnceCell;
use smol::channel::{unbounded, Sender};
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, ReplicaAccountInfoVersions, ReplicaBlockInfoVersions,
    ReplicaTransactionInfoVersions, Result as GeyserResult, SlotStatus,
};
use solana_sdk::pubkey::Pubkey;

mod account_filter;
pub use account_filter::*;

static VALIDATOR_CONFIG: OnceCell<Pubkey> = OnceCell::new();

lazy_static! {
    static ref SENDER: Sender<AccTx> = {
        let (sender, receiver) = unbounded::<AccTx>();

        smol::spawn(async move {
            while let Ok(value) = receiver.recv().await {
                match value {
                    AccTx::Tx { transaction, .. } => {
                        let message = transaction.message();

                        let fee_payer = message.fee_payer();
                        info!("FEE_PAYER: {:?}", fee_payer);
                        let instructions = message.decompile_instructions();

                        for instruction in instructions {
                            if instruction.program_id == &*VALIDATOR_CONFIG.get().unwrap() {
                                let command =
                                    ServiceCommand::try_from_slice(instruction.data).unwrap();

                                exec_command(
                                    &command,
                                    transaction.signature().to_string().as_str(),
                                );
                            }
                        }
                    }
                    _ => (),
                }
            }
        })
        .detach();

        sender
    };
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    let plugin = FusionEnginePlugin::new();
    let plugin: Box<dyn GeyserPlugin> = Box::new(plugin);
    Box::into_raw(plugin)
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct FusionEnginePlugin {
    libpath: String,
    validator_config: String,
}

impl FusionEnginePlugin {
    pub fn new() -> Self {
        FusionEnginePlugin::default()
    }
}

impl GeyserPlugin for FusionEnginePlugin {
    fn name(&self) -> &'static str {
        "FusionEnginePlugin"
    }

    fn on_load(&mut self, config_file: &str) -> GeyserResult<()> {
        use std::{fs::File, io::prelude::*};
        let validator_file_path = "../target/deploy/geyser_service_program-keypair.json";

        solana_logger::setup_with_default("info");
        info!(
            "Loading plugin {:?} from config_file {:?}",
            self.name(),
            config_file
        );

        // Get the Pubkey for the checking account for the Validator
        let mut validator_file = File::open(validator_file_path).unwrap();
        let mut validator_contents = String::new();
        validator_file
            .read_to_string(&mut validator_contents)
            .unwrap();
        let validator_bytes = serde_json::from_str::<Vec<u8>>(&validator_contents).unwrap();
        let validator_bytes: [u8; 64] = validator_bytes.try_into().unwrap();
        let validator_id_bytes: [u8; 32] = validator_bytes[32..].try_into().unwrap();
        let validator_checking_account = Pubkey::from(validator_id_bytes);

        info!("VALIDATOR_PUBKEY: {}", &validator_checking_account);

        VALIDATOR_CONFIG.set(validator_checking_account).unwrap();

        info!(
            "Loaded VALIDATOR_CONFIG {:?} from config_file {:?}",
            VALIDATOR_CONFIG.get(),
            validator_file_path
        );

        Ok(())
    }

    fn on_unload(&mut self) {}

    fn update_account(
        &mut self,
        account: ReplicaAccountInfoVersions,
        slot: u64,
        is_startup: bool,
    ) -> GeyserResult<()> {
        let outcome = AccTx::into_acc(slot, is_startup, &account);

        smol::block_on(async move {
            smol::spawn(async move { SENDER.send(outcome).await })
                .await
                .unwrap();
        });

        Ok(())
    }

    fn notify_transaction(
        &mut self,
        transaction: ReplicaTransactionInfoVersions,
        slot: u64,
    ) -> GeyserResult<()> {
        let outcome = AccTx::into_tx(slot, &transaction);

        smol::block_on(async move {
            smol::spawn(async move { SENDER.send(outcome).await }).detach();
        });

        Ok(())
    }

    fn notify_block_metadata(&mut self, _blockinfo: ReplicaBlockInfoVersions) -> GeyserResult<()> {
        Ok(())
    }

    fn update_slot_status(
        &mut self,
        _slot: u64,
        _parent: Option<u64>,
        _status: SlotStatus,
    ) -> GeyserResult<()> {
        Ok(())
    }

    fn notify_end_of_startup(&mut self) -> GeyserResult<()> {
        Ok(())
    }

    fn account_data_notifications_enabled(&self) -> bool {
        true
    }

    fn transaction_notifications_enabled(&self) -> bool {
        true
    }
}
