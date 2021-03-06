use solana_move_loader_program::{
    account_state::pubkey_to_address,
    processor::{Executable, MoveLoaderInstruction},
};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use types::{account_config, transaction::TransactionArgument};

pub fn genesis(genesis_pubkey: &Pubkey, microlibras: u64) -> Instruction {
    let instruction_data = MoveLoaderInstruction::CreateGenesis(microlibras);
    let accounts = vec![AccountMeta::new(*genesis_pubkey, true)];
    Instruction::new(solana_sdk::move_loader::id(), &instruction_data, accounts)
}

pub fn mint(
    script_pubkey: &Pubkey,
    genesis_pubkey: &Pubkey,
    to_pubkey: &Pubkey,
    microlibras: u64,
) -> Instruction {
    let args = vec![
        TransactionArgument::Address(pubkey_to_address(to_pubkey)),
        TransactionArgument::U64(microlibras),
    ];

    let instruction_data = Executable::RunScript {
        sender_address: account_config::association_address(),
        function_name: "main".to_string(),
        args,
    };

    let accounts = vec![
        AccountMeta::new_readonly(*script_pubkey, false),
        AccountMeta::new(*genesis_pubkey, true),
        AccountMeta::new(*to_pubkey, false),
    ];

    Instruction::new(solana_sdk::move_loader::id(), &instruction_data, accounts)
}

pub fn transfer(
    script_pubkey: &Pubkey,
    genesis_pubkey: &Pubkey,
    from_pubkey: &Pubkey,
    to_pubkey: &Pubkey,
    microlibras: u64,
) -> Instruction {
    let args = vec![
        TransactionArgument::Address(pubkey_to_address(to_pubkey)),
        TransactionArgument::U64(microlibras),
    ];

    let instruction_data = Executable::RunScript {
        sender_address: pubkey_to_address(from_pubkey),
        function_name: "main".to_string(),
        args,
    };

    let accounts = vec![
        AccountMeta::new_readonly(*script_pubkey, false),
        AccountMeta::new_readonly(*genesis_pubkey, false),
        AccountMeta::new(*from_pubkey, true),
        AccountMeta::new(*to_pubkey, false),
    ];

    Instruction::new(solana_sdk::move_loader::id(), &instruction_data, accounts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pay() {
        let from_pubkey = Pubkey::new_rand();
        let to_pubkey = Pubkey::new_rand();
        let program_id = Pubkey::new_rand();
        let mint_id = Pubkey::new_rand();
        transfer(&program_id, &mint_id, &from_pubkey, &to_pubkey, 1);
    }

    #[test]
    fn test_mint() {
        let program_id = Pubkey::new_rand();
        let from_pubkey = Pubkey::new_rand();
        let to_pubkey = Pubkey::new_rand();

        mint(&program_id, &from_pubkey, &to_pubkey, 1);
    }
}
