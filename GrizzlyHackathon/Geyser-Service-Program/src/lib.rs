use borsh::BorshDeserialize;
use geyser_service_common::ServiceCommand;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    pubkey::Pubkey,
    system_instruction, system_program,
};

mod command_lamports;
pub use command_lamports::*;

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("ProgramID: {:?}", &program_id);

    let account_info_iter = &mut accounts.iter();

    let payer = next_account_info(account_info_iter)?;
    let recipient = next_account_info(account_info_iter)?;
    // The system program is a required account to invoke a system
    // instruction, even though we don't use it directly.
    let system_account = next_account_info(account_info_iter)?;

    let command = ServiceCommand::try_from_slice(instruction_data)?;

    assert!(payer.is_writable);
    assert!(!recipient.is_signer);
    assert!(recipient.is_writable);
    assert!(system_program::check_id(system_account.key));

    let lamports = service_cost(&command);

    invoke(
        &system_instruction::transfer(payer.key, recipient.key, lamports),
        &[payer.clone(), recipient.clone(), system_account.clone()],
    )?;

    Ok(())
}
