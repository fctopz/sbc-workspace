use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};
entrypoint!(process_instruction_cebu);

fn process_instruction_cebu(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
        msg!("{},{},{:?}" , program_id, accounts.len(), instruction_data);
        Ok(())
}

// to build: cargo-build-sbf
// for update only - 20231205 @ 10:11pm