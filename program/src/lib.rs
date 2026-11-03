mod claim;
mod close;
mod compound;
mod deposit;
mod distribute;
mod init;
mod log;
mod withdraw;

use claim::*;
use close::*;
use compound::*;
use deposit::*;
use distribute::*;
use init::*;
use log::*;
use withdraw::*;

use blackgold_stake_api::instruction::*;
use solana_security_txt::security_txt;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&blackgold_stake_api::ID, program_id, data)?;

    match ix {
        BlackgoldStakeInstruction::Claim => process_claim(accounts, data)?,
        BlackgoldStakeInstruction::Close => process_close(accounts, data)?,
        BlackgoldStakeInstruction::Compound => process_compound(accounts, data)?,
        BlackgoldStakeInstruction::Deposit => process_deposit(accounts, data)?,
        BlackgoldStakeInstruction::Distribute => process_distribute(accounts, data)?,
        BlackgoldStakeInstruction::Init => process_init(accounts, data)?,
        BlackgoldStakeInstruction::Log => process_log(accounts, data)?,
        BlackgoldStakeInstruction::Withdraw => process_withdraw(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);

security_txt! {
    name: "BLKG Stake",
    project_url: "https://blackgold.supply",
    contacts: "email:orionlumesystems@gmail.com",
    policy: "https://github.com/orionlumesystems/blackgold-stake-api/blob/main/SECURITY.md",
    preferred_languages: "en",
    source_code: "https://github.com/orionlumesystems/blackgold-stake-api"
}
