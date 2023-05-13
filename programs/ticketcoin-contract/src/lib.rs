use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token;
use anchor_spl::token::{MintTo, Token};
use mpl_token_metadata::{instruction::{create_master_edition_v3, create_metadata_accounts_v3, approve_use_authority, utilize}, state::{Uses, UseMethod}};

declare_id!("69YpUBXmA97Lhjy21Wm4chkzkbh4SjwAFaHfNnNr8iJv");

#[program]
pub mod ticketcoin_contract {

    use super::*;

    pub fn mint_nft(
        ctx: Context<MintNFT>,
        creator_key: Pubkey,
        uri: String,
        title: String,
    ) -> Result<()> {
        msg!("Initializing Mint Ticket");
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        msg!("CPI Accounts Assigned");
        let cpi_program = ctx.accounts.token_program.to_account_info();
        msg!("CPI Program Assigned");
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        msg!("CPI Context Assigned");
        token::mint_to(cpi_ctx, 1)?;
        msg!("Token Minted !!!");
        let account_info = vec![
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
        msg!("Account Info Assigned");
        let creator = vec![
            mpl_token_metadata::state::Creator {
                address: creator_key,
                verified: false,
                share: 100,
            },
            mpl_token_metadata::state::Creator {
                address: ctx.accounts.mint_authority.key(),
                verified: false,
                share: 0,
            },
        ];
        msg!("Creator Assigned");
        let symbol = std::string::ToString::to_string("ZGT");
        invoke(
            &create_metadata_accounts_v3(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.payer.key(),
                title,
                symbol,
                uri,
                Some(creator),
                1,
                true,
                false,
                None,
                Some(Uses { use_method: UseMethod::Single, remaining: 1, total: 1}),
                None
            ),
            account_info.as_slice(),
        )?;
        msg!("Metadata Account Created !!!");
        let master_edition_infos = vec![
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
        msg!("Master Edition Account Infos Assigned");
        invoke(
            &create_master_edition_v3(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.master_edition.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.payer.key(),
                Some(0),
            ),
            master_edition_infos.as_slice(),
        )?;
        msg!("Master Edition Nft Minted !!!");

        msg!("Approve new Use Authority");
        let (burner, _) = mpl_token_metadata::pda::find_program_as_burner_account();
        msg!("Burner: {}", burner);
        
        
        let authority_info = vec![
            ctx.accounts.use_authority_record.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.verifier.to_account_info(),
            ctx.accounts.token_account.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.burner.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
        invoke(
            &approve_use_authority(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.use_authority_record.key(),
                ctx.accounts.verifier.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.token_account.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.burner.key(),
                1
            ),
            authority_info.as_slice(),
        )?;

        Ok(())
    }

    pub fn verify_nft(
        ctx: Context<VerifyNFT>,
    ) -> Result<()> {
        msg!("Initializing Mint Ticket");

        let authority_info = vec![
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.token_account.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.verifier.to_account_info(),
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.ata_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
            ctx.accounts.use_authority_record.to_account_info(),
            ctx.accounts.burner.to_account_info(),
        ];
        invoke(
            &utilize(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.token_account.key(),
                ctx.accounts.mint.key(),
                Some(ctx.accounts.use_authority_record.key()),
                ctx.accounts.verifier.key(),
                ctx.accounts.owner.key(),
                Some(ctx.accounts.burner.key()),
                1
            ),
            authority_info.as_slice(),
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    // #[account(mut)]
    pub token_program: Program<'info, Token>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub payer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub rent: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,

    /// CHECK: To avoid error
    #[account(mut)]
    pub use_authority_record: AccountInfo<'info>,

    /// CHECK: To avoid error
    #[account(mut)]
    pub verifier: AccountInfo<'info>,

    /// CHECK: To avoid error
    #[account(mut)]
    pub burner: AccountInfo<'info>,

}

#[derive(Accounts)]
pub struct VerifyNFT<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,


    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,

    #[account(mut)]
    pub verifier: Signer<'info>,


    /// CHECK: 
    pub owner: AccountInfo<'info>,


    // #[account(mut)]
    pub token_program: Program<'info, Token>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: UncheckedAccount<'info>,
    
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub ata_program: UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub rent: AccountInfo<'info>,

    /// CHECK: To avoid error
    #[account(mut)]
    pub use_authority_record: AccountInfo<'info>,
    /// CHECK: To avoid error
    pub burner: AccountInfo<'info>,
}