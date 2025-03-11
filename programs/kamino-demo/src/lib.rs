use anchor_lang::prelude::*;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program::invoke,
};
use solana_program::{pubkey, pubkey::Pubkey};

declare_id!("8jANiZd5HLQZea13Nz1oxWS6qEHKEA92s2WTzinY3vkA");

// Use the Kamino protocol program ID from your provided link.
pub const KAMINO_PROGRAM_ID: Pubkey = pubkey!("6LtLpnUFNByNXLyCoK9wA2MykKAmQNZKBdY8s47dehDc");

#[program]
pub mod kamino_demo {
    use super::*;

    //Function to trigger deposit instruction on Komino protocol program
    pub fn deposit(ctx: Context<Deposit>, token_max_a: u64, token_max_b: u64) -> Result<()> {
        let ix = Instruction {
            program_id: KAMINO_PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(ctx.accounts.user.key(), true),
                AccountMeta::new(ctx.accounts.strategy.key(), false),
                AccountMeta::new_readonly(ctx.accounts.global_config.key(), false),
                AccountMeta::new_readonly(ctx.accounts.pool.key(), false),
                AccountMeta::new_readonly(ctx.accounts.position.key(), false),
                AccountMeta::new_readonly(ctx.accounts.tick_array_lower.key(), false),
                AccountMeta::new_readonly(ctx.accounts.tick_array_upper.key(), false),
                AccountMeta::new(ctx.accounts.token_a_vault.key(), false),
                AccountMeta::new(ctx.accounts.token_b_vault.key(), false),
                AccountMeta::new_readonly(ctx.accounts.base_vault_authority.key(), false),
                AccountMeta::new(ctx.accounts.token_a_ata.key(), false),
                AccountMeta::new(ctx.accounts.token_b_ata.key(), false),
                AccountMeta::new_readonly(ctx.accounts.token_a_mint.key(), false),
                AccountMeta::new_readonly(ctx.accounts.token_b_mint.key(), false),
                AccountMeta::new(ctx.accounts.user_shares_ata.key(), false),
                AccountMeta::new(ctx.accounts.shares_mint.key(), false),
                AccountMeta::new_readonly(ctx.accounts.shares_mint_authority.key(), false),
                AccountMeta::new_readonly(ctx.accounts.scope_prices.key(), false),
                AccountMeta::new_readonly(ctx.accounts.token_infos.key(), false),
                AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
                AccountMeta::new_readonly(ctx.accounts.token_a_token_program.key(), false),
                AccountMeta::new_readonly(ctx.accounts.token_b_token_program.key(), false),
                AccountMeta::new_readonly(ctx.accounts.instruction_sysvar_account.key(), false),
            ],
            data: (0u8, token_max_a, token_max_b).try_to_vec()?,
        };
        invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.strategy.to_account_info(),
                ctx.accounts.global_config.to_account_info(),
                ctx.accounts.pool.to_account_info(),
                ctx.accounts.position.to_account_info(),
                ctx.accounts.tick_array_lower.to_account_info(),
                ctx.accounts.tick_array_upper.to_account_info(),
                ctx.accounts.token_a_vault.to_account_info(),
                ctx.accounts.token_b_vault.to_account_info(),
                ctx.accounts.base_vault_authority.to_account_info(),
                ctx.accounts.token_a_ata.to_account_info(),
                ctx.accounts.token_b_ata.to_account_info(),
                ctx.accounts.token_a_mint.to_account_info(),
                ctx.accounts.token_b_mint.to_account_info(),
                ctx.accounts.user_shares_ata.to_account_info(),
                ctx.accounts.shares_mint.to_account_info(),
                ctx.accounts.shares_mint_authority.to_account_info(),
                ctx.accounts.scope_prices.to_account_info(),
                ctx.accounts.token_infos.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.token_a_token_program.to_account_info(),
                ctx.accounts.token_b_token_program.to_account_info(),
                ctx.accounts.instruction_sysvar_account.to_account_info(),
            ],
        )?;
        Ok(())
    }

    //Function to trigger borrow/invest function on Komino protocol program
    pub fn borrow(ctx: Context<Invest>) -> Result<()> {
        let ix = Instruction {
            program_id: KAMINO_PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(ctx.accounts.payer.key(), true),
                AccountMeta::new(ctx.accounts.strategy.key(), false),
                AccountMeta::new_readonly(ctx.accounts.global_config.key(), false),
                AccountMeta::new(ctx.accounts.token_a_vault.key(), false),
                AccountMeta::new(ctx.accounts.token_b_vault.key(), false),
                AccountMeta::new_readonly(ctx.accounts.token_a_mint.key(), false),
                AccountMeta::new_readonly(ctx.accounts.token_b_mint.key(), false),
                AccountMeta::new(ctx.accounts.base_vault_authority.key(), false),
                AccountMeta::new(ctx.accounts.pool.key(), false),
                AccountMeta::new_readonly(ctx.accounts.token_a_token_program.key(), false),
                AccountMeta::new_readonly(ctx.accounts.token_b_token_program.key(), false),
                AccountMeta::new_readonly(ctx.accounts.memo_program.key(), false),
                AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
                AccountMeta::new_readonly(ctx.accounts.token_program_2022.key(), false),
                AccountMeta::new(ctx.accounts.position.key(), false),
                AccountMeta::new(
                    ctx.accounts
                        .raydium_protocol_position_or_base_vault_authority
                        .key(),
                    false,
                ),
                AccountMeta::new(ctx.accounts.position_token_account.key(), false),
                AccountMeta::new(ctx.accounts.pool_token_vault_a.key(), false),
                AccountMeta::new(ctx.accounts.pool_token_vault_b.key(), false),
                AccountMeta::new(ctx.accounts.tick_array_lower.key(), false),
                AccountMeta::new(ctx.accounts.tick_array_upper.key(), false),
                AccountMeta::new_readonly(ctx.accounts.scope_prices.key(), false),
                AccountMeta::new_readonly(ctx.accounts.token_infos.key(), false),
                AccountMeta::new_readonly(ctx.accounts.pool_program.key(), false),
                AccountMeta::new_readonly(ctx.accounts.instruction_sysvar_account.key(), false),
            ],
            data: vec![], // No arguments are passed in the IDL
        };

        let mut accounts = vec![
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.strategy.to_account_info(),
            ctx.accounts.global_config.to_account_info(),
            ctx.accounts.token_a_vault.to_account_info(),
            ctx.accounts.token_b_vault.to_account_info(),
            ctx.accounts.token_a_mint.to_account_info(),
            ctx.accounts.token_b_mint.to_account_info(),
            ctx.accounts.base_vault_authority.to_account_info(),
            ctx.accounts.pool.to_account_info(),
            ctx.accounts.token_a_token_program.to_account_info(),
            ctx.accounts.token_b_token_program.to_account_info(),
            ctx.accounts.memo_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.token_program_2022.to_account_info(),
            ctx.accounts.position.to_account_info(),
            ctx.accounts
                .raydium_protocol_position_or_base_vault_authority
                .to_account_info(),
            ctx.accounts.position_token_account.to_account_info(),
            ctx.accounts.pool_token_vault_a.to_account_info(),
            ctx.accounts.pool_token_vault_b.to_account_info(),
            ctx.accounts.tick_array_lower.to_account_info(),
            ctx.accounts.tick_array_upper.to_account_info(),
            ctx.accounts.scope_prices.to_account_info(),
            ctx.accounts.token_infos.to_account_info(),
            ctx.accounts.pool_program.to_account_info(),
            ctx.accounts.instruction_sysvar_account.to_account_info(),
        ];

        invoke(&ix, &accounts[..])?;
        Ok(())
    }
}

//Account for the Deposit instruction
#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub strategy: AccountInfo<'info>,
    pub global_config: AccountInfo<'info>,
    pub pool: AccountInfo<'info>,
    pub position: AccountInfo<'info>,
    pub tick_array_lower: AccountInfo<'info>,
    pub tick_array_upper: AccountInfo<'info>,
    #[account(mut)]
    pub token_a_vault: AccountInfo<'info>,
    #[account(mut)]
    pub token_b_vault: AccountInfo<'info>,
    pub base_vault_authority: AccountInfo<'info>,
    #[account(mut)]
    pub token_a_ata: AccountInfo<'info>,
    #[account(mut)]
    pub token_b_ata: AccountInfo<'info>,
    pub token_a_mint: AccountInfo<'info>,
    pub token_b_mint: AccountInfo<'info>,
    #[account(mut)]
    pub user_shares_ata: AccountInfo<'info>,
    #[account(mut)]
    pub shares_mint: AccountInfo<'info>,
    pub shares_mint_authority: AccountInfo<'info>,
    pub scope_prices: AccountInfo<'info>,
    pub token_infos: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub token_a_token_program: AccountInfo<'info>,
    pub token_b_token_program: AccountInfo<'info>,
    pub instruction_sysvar_account: AccountInfo<'info>,
}

/// Account for the borrow/invest instruction.
#[derive(Accounts)]
pub struct Invest<'info> {
    #[account(mut, signer)]
    pub payer: AccountInfo<'info>,
    #[account(mut)]
    pub strategy: AccountInfo<'info>,
    pub global_config: AccountInfo<'info>,
    #[account(mut)]
    pub token_a_vault: AccountInfo<'info>,
    #[account(mut)]
    pub token_b_vault: AccountInfo<'info>,
    pub token_a_mint: AccountInfo<'info>,
    pub token_b_mint: AccountInfo<'info>,
    #[account(mut)]
    pub base_vault_authority: AccountInfo<'info>,
    #[account(mut)]
    pub pool: AccountInfo<'info>,
    pub token_a_token_program: AccountInfo<'info>,
    pub token_b_token_program: AccountInfo<'info>,
    pub memo_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub token_program_2022: AccountInfo<'info>,
    #[account(mut)]
    pub position: AccountInfo<'info>,
    #[account(mut)]
    pub raydium_protocol_position_or_base_vault_authority: AccountInfo<'info>,
    #[account(mut)]
    pub position_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub pool_token_vault_a: AccountInfo<'info>,
    #[account(mut)]
    pub pool_token_vault_b: AccountInfo<'info>,
    #[account(mut)]
    pub tick_array_lower: AccountInfo<'info>,
    #[account(mut)]
    pub tick_array_upper: AccountInfo<'info>,
    pub scope_prices: AccountInfo<'info>,
    pub token_infos: AccountInfo<'info>,
    pub pool_program: AccountInfo<'info>,
    pub instruction_sysvar_account: AccountInfo<'info>,
    #[account(mut)]
    pub event_authority: Option<AccountInfo<'info>>,
}
