use crate::constants::DEFAULT_OBSERVATION_LENGTH;
use crate::state::bin_array_bitmap_extension::BinArrayBitmapExtension;
use crate::state::lb_pair::LbPair;
use crate::state::oracle::Oracle;
use crate::state::preset_parameters::PresetParameter;
use crate::utils;
use crate::utils::seeds::BIN_ARRAY_BITMAP_SEED;
use crate::utils::seeds::ILM_BASE_KEY;
use crate::utils::seeds::ORACLE;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};

use std::cmp::{max, min};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CustomizableParams {
    /// Pool price
    pub active_id: i32,
    /// Bin step
    pub bin_step: u16,
    /// Base factor
    pub base_factor: u16,
    /// Activation type. 0 = Slot, 1 = Time. Check ActivationType enum
    pub activation_type: u8,
    /// Whether the pool has an alpha vault
    pub has_alpha_vault: bool,
    /// Decide when does the pool start trade. None = Now
    pub activation_point: Option<u64>,
    /// Padding, for future use
    pub padding: [u8; 64],
}

#[event_cpi]
#[derive(Accounts)]
#[instruction(params: CustomizableParams)]
pub struct InitializeCustomizablePermissionlessLbPair<'info> {
    /// Mint account for Token X
    #[account(constraint = token_mint_x.key() != token_mint_y.key())]
    pub token_mint_x: Account<'info, Mint>,

    /// Mint account for Token Y
    pub token_mint_y: Account<'info, Mint>,

    /// Loader account for LbPair
    #[account(
    init,
    seeds = [
        ILM_BASE_KEY.as_ref(),
        min(token_mint_x.key().as_ref(), token_mint_y.key().as_ref()),
        max(token_mint_x.key().as_ref(), token_mint_y.key().as_ref()),
    ],
    bump,
    payer = funder,
    space = 8 + LbPair::INIT_SPACE
    )]
    pub lb_pair: AccountLoader<'info, LbPair>,

    /// Optional Loader account for BinArrayBitmapExtension
    #[account(
        init,
        seeds = [
            BIN_ARRAY_BITMAP_SEED,
            lb_pair.key().as_ref(),
        ],
        bump,
        payer = funder,
        space = 8 + BinArrayBitmapExtension::INIT_SPACE
    )]
    pub bin_array_bitmap_extension: Option<AccountLoader<'info, BinArrayBitmapExtension>>,

    /// Token account for Reserve X
    #[account(
        init,
        seeds = [
            lb_pair.key().as_ref(),
            token_mint_x.key().as_ref()
        ],
        bump,
        payer = funder,
        token::mint = token_mint_x,
        token::authority = lb_pair,
    )]
    pub reserve_x: Account<'info, TokenAccount>,

    /// Token account for Reserve Y
    #[account(
        init,
        seeds = [
            lb_pair.key().as_ref(),
            token_mint_y.key().as_ref()
        ],
        bump,
        payer = funder,
        token::mint = token_mint_y,
        token::authority = lb_pair,
    )]
    pub reserve_y: Account<'info, TokenAccount>,

    /// Loader account for Oracle
    #[account(
        init,
        seeds = [
            ORACLE,
            lb_pair.key().as_ref()
        ],
        bump,
        payer = funder,
        space = Oracle::space(DEFAULT_OBSERVATION_LENGTH)
    )]
    pub oracle: AccountLoader<'info, Oracle>,

    /// User Token Account for Token X
    #[account(
        token::authority = funder,
        token::mint = token_mint_x,
    )]
    pub user_token_x: Account<'info, TokenAccount>,

    /// Funder Signer
    #[account(mut)]
    pub funder: Signer<'info>,

    /// Token Program
    pub token_program: Program<'info, Token>,

    /// System Program
    pub system_program: Program<'info, System>,

    /// Rent Sysvar
    pub rent: Sysvar<'info, Rent>,
}

pub fn handle(
    ctx: Context<InitializeCustomizablePermissionlessLbPair>,
    params: CustomizableParams,
) -> Result<()> {
    Ok(())
}
