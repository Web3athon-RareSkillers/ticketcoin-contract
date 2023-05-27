use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use solana_program::sysvar::instructions::{ID as IX_ID, load_instruction_at_checked};
use solana_program::ed25519_program::{ID as ED25519_ID};
use anchor_spl::token::{self, TokenAccount, Mint};
use anchor_spl::token::{MintTo, Token};
use mpl_token_metadata::{instruction::{create_master_edition_v3, create_metadata_accounts_v3, approve_use_authority, utilize}, state::{Uses, UseMethod, Collection, Metadata}};

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
        let creators = vec![
            mpl_token_metadata::state::Creator {
                address: *ctx.program_id,
                verified: false, // TO IMPROVE TO GET MORE SECURITY
                share: 0,
            },
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
                Some(creators),
                0,
                false,
                false,
                Some(Collection { verified: false, key: ctx.accounts.collection.key() }),
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
    
    /// External instruction that only gets executed if
    /// an `Ed25519Program.createInstructionWithPublicKey`
    /// instruction was sent in the same transaction.
    pub fn verify_nft(
        ctx: Context<VerifyNFT>, pubkey: [u8; 32], msg: Vec<u8>, sig: [u8; 64]
    ) -> Result<()> {
        // Get what should be the Ed25519Program instruction
        let ix: Instruction = load_instruction_at_checked(0, &ctx.accounts.ix_sysvar)?;

        // Check that ix is what we expect to have been sent
        utils::verify_ed25519_ix(&ix, &pubkey, &msg, &sig)?;

        // Do other stuff    
        
        let nft_token_account = &ctx.accounts.token_account;
        let nft_mint_account = &ctx.accounts.mint;


        // Check that owner in arguments is real owner
        assert_eq!(ctx.accounts.owner.key(), nft_token_account.owner);

        // Check the mint on the token account
        assert_eq!(nft_mint_account.key(), nft_token_account.mint);
        
        // Check amount on the token account
        assert_eq!(nft_token_account.amount, 1);

        //let NFTmetadata = Metadata::from_account_info(ctx.accounts.metadata
        /*let NFTmetadata = Metadata::deserialize(ctx.accounts.metadata.data.into());
        if NFTmetadata.is_mutable {
            return Err(Error::ProgramError("NFT is mutable..."))
        }*/



        // Set ticket as used
        msg!("Set Ticket as used");

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


pub mod utils {
    use super::*;

    /// Verify Ed25519Program instruction fields
    pub fn verify_ed25519_ix(ix: &Instruction, pubkey: &[u8], msg: &[u8], sig: &[u8]) -> ProgramResult {
        if  ix.program_id       != ED25519_ID                   ||  // The program id we expect
            ix.accounts.len()   != 0                            ||  // With no context accounts
            ix.data.len()       != (16 + 64 + 32 + msg.len())       // And data of this size
        {
            return Err(ErrorCode::SigVerificationFailed.into());    // Otherwise, we can already throw err
        }

        check_ed25519_data(&ix.data, pubkey, msg, sig)?;            // If that's not the case, check data

        Ok(())
    }

    /// Verify Secp256k1Program instruction fields
    pub fn verify_secp256k1_ix(ix: &Instruction, eth_address: &[u8], msg: &[u8], sig: &[u8], recovery_id: u8) -> ProgramResult {
        if  ix.program_id       != SECP256K1_ID                 ||  // The program id we expect
            ix.accounts.len()   != 0                            ||  // With no context accounts
            ix.data.len()       != (12 + 20 + 64 + 1 + msg.len())   // And data of this size
        {
            return Err(ErrorCode::SigVerificationFailed.into());    // Otherwise, we can already throw err
        }

        check_secp256k1_data(&ix.data, eth_address, msg, sig, recovery_id)?; // If that's not the case, check data

        Ok(())
    }

    /// Verify serialized Ed25519Program instruction data
    pub fn check_ed25519_data(data: &[u8], pubkey: &[u8], msg: &[u8], sig: &[u8]) -> ProgramResult {
        // According to this layout used by the Ed25519Program
        // https://github.com/solana-labs/solana-web3.js/blob/master/src/ed25519-program.ts#L33

        // "Deserializing" byte slices

        let num_signatures                  = &[data[0]];        // Byte  0
        let padding                         = &[data[1]];        // Byte  1
        let signature_offset                = &data[2..=3];      // Bytes 2,3
        let signature_instruction_index     = &data[4..=5];      // Bytes 4,5
        let public_key_offset               = &data[6..=7];      // Bytes 6,7
        let public_key_instruction_index    = &data[8..=9];      // Bytes 8,9
        let message_data_offset             = &data[10..=11];    // Bytes 10,11
        let message_data_size               = &data[12..=13];    // Bytes 12,13
        let message_instruction_index       = &data[14..=15];    // Bytes 14,15

        let data_pubkey                     = &data[16..16+32];  // Bytes 16..16+32
        let data_sig                        = &data[48..48+64];  // Bytes 48..48+64
        let data_msg                        = &data[112..];      // Bytes 112..end

        // Expected values

        let exp_public_key_offset:      u16 = 16; // 2*u8 + 7*u16
        let exp_signature_offset:       u16 = exp_public_key_offset + pubkey.len() as u16;
        let exp_message_data_offset:    u16 = exp_signature_offset + sig.len() as u16;
        let exp_num_signatures:          u8 = 1;
        let exp_message_data_size:      u16 = msg.len().try_into().unwrap();

        // Header and Arg Checks

        // Header
        if  num_signatures                  != &exp_num_signatures.to_le_bytes()        ||
            padding                         != &[0]                                     ||
            signature_offset                != &exp_signature_offset.to_le_bytes()      ||
            signature_instruction_index     != &u16::MAX.to_le_bytes()                  ||
            public_key_offset               != &exp_public_key_offset.to_le_bytes()     ||
            public_key_instruction_index    != &u16::MAX.to_le_bytes()                  ||
            message_data_offset             != &exp_message_data_offset.to_le_bytes()   ||
            message_data_size               != &exp_message_data_size.to_le_bytes()     ||
            message_instruction_index       != &u16::MAX.to_le_bytes()  
        {
            return Err(ErrorCode::SigVerificationFailed.into());
        }

        // Arguments
        if  data_pubkey != pubkey   ||
            data_msg    != msg      ||
            data_sig    != sig
        {
            return Err(ErrorCode::SigVerificationFailed.into());
        }

        Ok(())
    }

    /// Verify serialized Secp256k1Program instruction data
    pub fn check_secp256k1_data(data: &[u8], eth_address: &[u8], msg: &[u8], sig: &[u8], recovery_id: u8) -> ProgramResult {
        // According to this layout used by the Secp256k1Program
        // https://github.com/solana-labs/solana-web3.js/blob/master/src/secp256k1-program.ts#L49

        // "Deserializing" byte slices

        let num_signatures                  = &[data[0]];           // Byte  0
        let signature_offset                = &data[1..=2];         // Bytes 1,2
        let signature_instruction_index     = &[data[3]];           // Byte  3
        let eth_address_offset              = &data[4..=5];         // Bytes 4,5
        let eth_address_instruction_index   = &[data[6]];           // Byte  6
        let message_data_offset             = &data[7..=8];         // Bytes 7,8
        let message_data_size               = &data[9..=10];        // Bytes 9,10
        let message_instruction_index       = &[data[11]];          // Byte  11

        let data_eth_address                = &data[12..12+20];     // Bytes 12..12+20
        let data_sig                        = &data[32..32+64];     // Bytes 32..32+64
        let data_recovery_id                = &[data[96]];          // Byte  96
        let data_msg                        = &data[97..];          // Bytes 97..end

        // Expected values

        const SIGNATURE_OFFSETS_SERIALIZED_SIZE:    u16 = 11;
        const DATA_START:                           u16 = 1 + SIGNATURE_OFFSETS_SERIALIZED_SIZE;

        let msg_len:                    u16 = msg.len().try_into().unwrap();
        let eth_address_len:            u16 = eth_address.len().try_into().unwrap();
        let sig_len:                    u16 = sig.len().try_into().unwrap();

        let exp_eth_address_offset:     u16 = DATA_START;
        let exp_signature_offset:       u16 = DATA_START + eth_address_len;
        let exp_message_data_offset:    u16 = exp_signature_offset + sig_len + 1;
        let exp_num_signatures:          u8 = 1;

        // Header and Arg Checks

        // Header
        if  num_signatures                  != &exp_num_signatures.to_le_bytes()         ||
            signature_offset                != &exp_signature_offset.to_le_bytes()       ||
            signature_instruction_index     != &[0]                                      ||
            eth_address_offset              != &exp_eth_address_offset.to_le_bytes()     ||
            eth_address_instruction_index   != &[0]                                      ||
            message_data_offset             != &exp_message_data_offset.to_le_bytes()    ||
            message_data_size               != &msg_len.to_le_bytes()                    ||
            message_instruction_index       != &[0]
        {
            return Err(ErrorCode::SigVerificationFailed.into());
        }

        // Arguments
        if  data_eth_address    != eth_address      ||
            data_sig            != sig              ||
            data_recovery_id    != &[recovery_id]   ||
            data_msg            != msg
        {
            return Err(ErrorCode::SigVerificationFailed.into());
        }

        Ok(())
    }
}


#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    
    /// CHECK: To check later
    #[account(mut)]
    pub collection: AccountInfo<'info>,

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
    pub metadata: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,


    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub verifier: Signer<'info>,

    /// CHECK: 
    pub owner: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: UncheckedAccount<'info>,
    
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub ata_program: UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub rent: AccountInfo<'info>,
    
    #[account(address = IX_ID)]
    pub ix_sysvar: AccountInfo<'info>,

    /// CHECK: To avoid error
    #[account(mut)]
    pub use_authority_record: AccountInfo<'info>,
    /// CHECK: To avoid error
    pub burner: AccountInfo<'info>,
}

#[error]
pub enum ErrorCode {
    #[msg("Signature verification failed.")]
    SigVerificationFailed,
}
