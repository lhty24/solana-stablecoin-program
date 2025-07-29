use anchor_lang::{
    accounts::signer,
    prelude::*,
    solana_program::example_mocks::solana_sdk::system_program,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    token::TokenAccount,
    token_2022::{mint_to, MintTo, Token2022},
    token_interface::Mint,
};

use crate::constants::SEED_MINT_ACCOUNT;

pub fn mint_tokens<'info>(
    mint_account: &InterfaceAccount<'info, Mint>,
    token_account: &InterfaceAccount<'info, TokenAccount>,
    token_program: &Program<'info, Token2022>,
    amount: u64,
    bump: u8,
) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[SEED_MINT_ACCOUNT, &[bump]]];

    mint_to(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            MintTo {
                mint: mint_account.to_account_info(),
                to: token_account.to_account_info(),
                authority: mint_account.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )
}

pub fn deposit_sol<'info>(
    from: &Signer<'info>,
    to: &SystemAccount<'info>,
    system_program: &Program<'info, System>,
    amount: u64,
) -> Result<()> {
    transfer(
        CpiContext::new(
            system_program.to_account_info(),
            Transfer {
                from: from.to_account_info(),
                to: to.to_account_info(),
            },
        ),
        amount,
    )
}
