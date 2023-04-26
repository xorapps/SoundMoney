use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::BillableItem;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, BorshDeserialize, BorshSerialize)]
pub enum ServiceCommand {
    DecentralizedNotification {
        recipient: Pubkey,
        validator: Pubkey,
        service_id: String,
        tx: String,
        date: String,
        items: Vec<BillableItem>,
    },
    ThinClientWebVersion {
        validator: Pubkey,
        customer_name: String,
        items: Vec<BillableItem>,
        tx: String,
        date: String,
    },
    TxEmail {
        validator: Pubkey,
        customer_name: String,
        email: String,
        subject: String,
        items: Vec<BillableItem>,
        tx: String,
        date: String,
    },
    NewsletterEmail {
        validator: Pubkey,
        from: String,
        reply_to: String,
        to: String,
        notification: String,
        service_id: String, //HTML String,
        tx: String,
        date: String,
        items: Vec<BillableItem>,
    },
}
