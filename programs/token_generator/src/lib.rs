use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_spl::token::{self, Mint, Token, TokenAccount, InitializeMint, MintTo};

declare_id!("4SbnEov5361vyTdAkVHgFhWrsmCmAkCLM5Gna7K8bTxs");

#[program]
pub mod token_generator {
    use super::*;

    pub fn initialize_service_token(ctx: Context<InitializeServiceToken>, description: String, cost: u64) -> Result<()> {
        let service_token = &mut ctx.accounts.service_token;
        service_token.description = description;
        service_token.cost = cost;
        service_token.provider = *ctx.accounts.provider.key;
        service_token.mint = *ctx.accounts.mint.to_account_info().key;
        service_token.initialized = true;

        // Initialize mint
        let cpi_accounts = InitializeMint {
            mint: ctx.accounts.mint.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program.clone(), cpi_accounts);
        token::initialize_mint(cpi_ctx, 9, &ctx.accounts.provider.key(), Some(&ctx.accounts.provider.key()))?;

        // Mint tokens to doctor's wallet
        let cpi_accounts_mint = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.doctor_token_account.to_account_info(),
            authority: ctx.accounts.provider.to_account_info(),
        };
        let cpi_ctx_mint = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts_mint);
        token::mint_to(cpi_ctx_mint, 100)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeServiceToken<'info> {
    #[account(init, payer = provider, space = 8 + 32 + 8 + 32 + 1)]
    pub service_token: Account<'info, ServiceToken>,
    #[account(init, payer = provider, mint::decimals = 9, mint::authority = provider)]
    pub mint: Account<'info, Mint>,
    #[account(init, payer = provider, token::authority = provider, token::mint = mint)]
    pub doctor_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub provider: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct ServiceToken {
    pub description: String,
    pub cost: u64,
    pub provider: Pubkey,
    pub mint: Pubkey,
    pub initialized: bool,
}
