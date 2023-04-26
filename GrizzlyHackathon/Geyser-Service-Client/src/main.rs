use geyser_service_common::{BillableItem, ServiceCommand};
use serde::Deserialize;
use solana_rpc_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    signer::keypair::Keypair,
    system_program,
    transaction::Transaction,
};
use std::{fs::File, io::prelude::*};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut client_config_file = File::open("../Config/client_config.toml").unwrap();
    let mut client_config_contents = String::new();
    client_config_file
        .read_to_string(&mut client_config_contents)
        .unwrap();
    let client_config = toml::from_str::<ClientConfig>(&client_config_contents).unwrap();

    let mut keypair_file = File::open("../Config/user_keypair.json").unwrap();
    let mut keypair_contents = String::new();
    keypair_file.read_to_string(&mut keypair_contents).unwrap();
    let keypair_bytes = serde_json::from_str::<Vec<u8>>(&keypair_contents).unwrap();
    let keypair_bytes: [u8; 64] = keypair_bytes.try_into().unwrap();

    let user_keypair = Keypair::from_bytes(&keypair_bytes).unwrap();

    let public_key_bytes: [u8; 32] = keypair_bytes[32..].try_into().unwrap();
    let public_key = Pubkey::from(public_key_bytes);

    println!("USER PUBLIC KEY: {}", &public_key);

    // Get the programID
    let mut program_file =
        File::open("../target/deploy/geyser_service_program-keypair.json").unwrap();
    let mut program_contents = String::new();
    program_file.read_to_string(&mut program_contents).unwrap();
    let program_bytes = serde_json::from_str::<Vec<u8>>(&program_contents).unwrap();
    let program_bytes: [u8; 64] = program_bytes.try_into().unwrap();
    let program_id_bytes: [u8; 32] = program_bytes[32..].try_into().unwrap();
    let program_id = Pubkey::from(program_id_bytes);

    println!("PROGRAM ID: {}", &program_id);

    // Get the Pubkey for the checking account for the Validator
    let mut validator_file = File::open("../Config/validator_bank.json").unwrap();
    let mut validator_contents = String::new();
    validator_file
        .read_to_string(&mut validator_contents)
        .unwrap();
    let validator_bytes = serde_json::from_str::<Vec<u8>>(&validator_contents).unwrap();
    let validator_bytes: [u8; 64] = validator_bytes.try_into().unwrap();
    let validator_id_bytes: [u8; 32] = validator_bytes[32..].try_into().unwrap();
    let validator_checking_account = Pubkey::from(validator_id_bytes);

    println!("VALIDATOR_PUBKEY: {}", &validator_checking_account);

    let mut my_items = Vec::<BillableItem>::new();
    my_items.push(
        BillableItem::new()
            .add_cost("USDC-SPL 2")
            .add_img(format!("http://{}/cdn/images/espresso.jpg", client_config.ip).as_str())
            .add_name("Espresso"),
    );

    my_items.push(
        BillableItem::new()
            .add_cost("USDC-SPL 0.88")
            .add_img(format!("http://{}/cdn/images/doughnut.jpg", client_config.ip).as_str())
            .add_name("Doughnut"),
    );

    my_items.push(
        BillableItem::new()
            .add_cost("USDC-SPL 1.8")
            .add_img(format!("http://{}/cdn/images/mocha.jpg", client_config.ip).as_str())
            .add_name("Mocha"),
    );

    my_items.push(
        BillableItem::new()
            .add_cost("USDC-SPL 0.2")
            .add_img(format!("http://{}/cdn/images/croissant.jpg", client_config.ip).as_str())
            .add_name("Croissant"),
    );

    if args[1] == "mail" {
        send_mail(
            &client_config.email,
            my_items,
            program_id,
            public_key,
            validator_checking_account,
            &user_keypair,
        );
    } else if args[1] == "cdn" {
        launch_cdn(
            my_items,
            program_id,
            public_key,
            validator_checking_account,
            &user_keypair,
            &client_config.ip,
        );
    } else {
        println!("You need to pass the arguments `geyser-service-client cdn` or `geyser-service-client mail` to choose between seeing the demo as cdn or mail");

        std::process::exit(1);
    }
}

fn launch_cdn(
    my_items: Vec<BillableItem>,
    program_id: Pubkey,
    public_key: Pubkey,
    validator_checking_account: Pubkey,
    user_keypair: &Keypair,
    ip: &str,
) {
    let instruction_data = ServiceCommand::ThinClientWebVersion {
        validator: validator_checking_account,
        customer_name: "King Anatoly".to_owned(),
        date: "Tue 14 March 2023".to_owned(),
        tx: "FooBar".to_owned(),
        items: my_items,
    };

    let instruction = Instruction::new_with_borsh(
        program_id,
        &instruction_data,
        vec![
            AccountMeta::new(public_key, true),
            AccountMeta::new(validator_checking_account, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
    );

    let message = Message::new(&[instruction], Some(&public_key));

    let url = "http://localhost:8899".to_string();
    let client = RpcClient::new(url);

    let blockhash = client.get_latest_blockhash().unwrap();
    let tx = Transaction::new(&[user_keypair], message, blockhash);
    let tx_signature = client.send_and_confirm_transaction(&tx).unwrap();

    println!("TX Signature - {}", &tx_signature);
    let url_signature = format!("http://{}/tx/{}", ip, tx_signature);
    println!("Visit - {}", &url_signature);

    if webbrowser::open(&url_signature).is_ok() {
        println!("SUCCESSFULY OPENED THE CDN LINK ON YOUR BROWSER.")
    } else {
        println!(
            "Unable to launch the CDN link on your browser. \n Manually visit - {}",
            url_signature
        );
    }
}

fn send_mail(
    email: &str,
    my_items: Vec<BillableItem>,
    program_id: Pubkey,
    public_key: Pubkey,
    validator_checking_account: Pubkey,
    user_keypair: &Keypair,
) {
    let instruction_data = ServiceCommand::TxEmail {
        validator: validator_checking_account,
        customer_name: "King Anatoly".to_owned(),
        date: "Tue 14 March 2023".to_owned(),
        tx: "FooBar".to_owned(),
        items: my_items,
        email: email.to_owned(),
        subject: "Receipt for Breakfast".to_owned(),
    };

    let instruction = Instruction::new_with_borsh(
        program_id,
        &instruction_data,
        vec![
            AccountMeta::new(public_key, true),
            AccountMeta::new(validator_checking_account, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
    );

    let message = Message::new(&[instruction], Some(&public_key));

    let url = "http://localhost:8899".to_string();
    let client = RpcClient::new(url);

    let blockhash = client.get_latest_blockhash().unwrap();
    let tx = Transaction::new(&[user_keypair], message, blockhash);
    let tx_signature = client.send_and_confirm_transaction(&tx).unwrap();

    dbg!(&tx_signature);
}

#[derive(Debug, Deserialize)]
pub struct ClientConfig {
    pub ip: String,
    pub email: String,
}
