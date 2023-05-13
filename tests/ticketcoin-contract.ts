import * as anchor from "@coral-xyz/anchor";
import { Program, Wallet } from "@coral-xyz/anchor";
import { TicketcoinContract } from "../target/types/ticketcoin_contract";
import { TOKEN_PROGRAM_ID, createAssociatedTokenAccountInstruction, getAssociatedTokenAddress, createInitializeMintInstruction, MINT_SIZE } from '@solana/spl-token' // IGNORE THESE ERRORS IF ANY
import { Metadata } from "@metaplex-foundation/mpl-token-metadata";
const { SystemProgram } = anchor.web3



describe("ticketcoin-contract", () => {
  // Configure the client to use the cluster in env.
  const provider = anchor.AnchorProvider.env();
  const wallet = provider.wallet as Wallet;
  anchor.setProvider(provider);

  const program = anchor.workspace.TicketcoinContract as Program<TicketcoinContract>;

  function programPaidBy(payer: anchor.web3.Keypair): anchor.Program {
    const newProvider = new anchor.AnchorProvider(provider.connection, new anchor.Wallet(payer), {});
  
    return new anchor.Program(program.idl as anchor.Idl, program.programId, newProvider)
  }

  it("Is initialized!", async () => {
    // Add your test here.

    const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
      "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
    );
    const lamports: number =
      await program.provider.connection.getMinimumBalanceForRentExemption(
        MINT_SIZE
      );
    const getMetadata = async (
      mint: anchor.web3.PublicKey
    ): Promise<anchor.web3.PublicKey> => {
      return (
        await anchor.web3.PublicKey.findProgramAddress(
          [
            Buffer.from("metadata"),
            TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            mint.toBuffer(),
          ],
          TOKEN_METADATA_PROGRAM_ID
        )
      )[0];
    };

    const getUseAuthority = async (
      mint: anchor.web3.PublicKey,
      verifier: anchor.web3.PublicKey
    ): Promise<anchor.web3.PublicKey> => {
      return (
        await anchor.web3.PublicKey.findProgramAddress(
          [
            Buffer.from("metadata"),
            TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            mint.toBuffer(),
            Buffer.from("user"),
            verifier.toBuffer()
          ],
          TOKEN_METADATA_PROGRAM_ID
        )
      )[0];
    };

    const getBurner = async (
    ): Promise<anchor.web3.PublicKey> => {
      return (
        await anchor.web3.PublicKey.findProgramAddress(
          [
            Buffer.from("metadata"),
            TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            Buffer.from("burn")
          ],
          TOKEN_METADATA_PROGRAM_ID
        )
      )[0];
    };

    const getMasterEdition = async (
      mint: anchor.web3.PublicKey
    ): Promise<anchor.web3.PublicKey> => {
      return (
        await anchor.web3.PublicKey.findProgramAddress(
          [
            Buffer.from("metadata"),
            TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            mint.toBuffer(),
            Buffer.from("edition"),
          ],
          TOKEN_METADATA_PROGRAM_ID
        )
      )[0];
    };

    // private Key of the verifier
    const verifierKey: anchor.web3.Keypair = anchor.web3.Keypair.fromSecretKey(
      Uint8Array.from([101,14,176,133,137,147,99,228,239,38,213,73,217,54,202,27,19,192,49,152,95,155,11,157,206,125,120,114,232,66,178,200,160,30,158,23,36,46,127,141,85,130,181,111,247,17,147,61,228,144,171,230,158,67,136,167,142,128,152,222,162,31,103,173])
      );

    const mintKey: anchor.web3.Keypair = anchor.web3.Keypair.generate();
    const NftTokenAccount = await getAssociatedTokenAddress(
      mintKey.publicKey,
      wallet.publicKey
    );
    console.log("NFT Account: ", NftTokenAccount.toBase58());

    const mint_tx = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.createAccount({
        fromPubkey: wallet.publicKey,
        newAccountPubkey: mintKey.publicKey,
        space: MINT_SIZE,
        programId: TOKEN_PROGRAM_ID,
        lamports,
      }),
      createInitializeMintInstruction(
        mintKey.publicKey,
        0,
        wallet.publicKey,
        wallet.publicKey
      ),
      createAssociatedTokenAccountInstruction(
        wallet.publicKey,
        NftTokenAccount,
        wallet.publicKey,
        mintKey.publicKey
      )
    );

    const res = await program.provider.sendAndConfirm(mint_tx, [mintKey]);
    console.log(
      await program.provider.connection.getParsedAccountInfo(mintKey.publicKey)
    );

    console.log("Account: ", res);
    console.log("Mint key: ", mintKey.publicKey.toString());
    console.log("User: ", wallet.publicKey.toString());

    const metadataAddress = await getMetadata(mintKey.publicKey);
    const masterEdition = await getMasterEdition(mintKey.publicKey);
    const authorityRecord = await getUseAuthority(mintKey.publicKey, verifierKey.publicKey);
    const burnerAddress = await getBurner();

    console.log("Metadata address: ", metadataAddress.toBase58());
    console.log("MasterEdition: ", masterEdition.toBase58());
    console.log("AuthorityRecord: ", authorityRecord.toBase58());
    console.log("burner: ", burnerAddress.toBase58());

    /*console.log(wallet.publicKey.toBase58());
    console.log(mintKey.publicKey.toBase58());
    console.log(NftTokenAccount.toBase58());
    console.log(TOKEN_PROGRAM_ID.toBase58());
    console.log(metadataAddress.toBase58());
    console.log(TOKEN_METADATA_PROGRAM_ID.toBase58());
    console.log(wallet.publicKey.toBase58());
    console.log(SystemProgram.programId.toBase58());
    console.log(anchor.web3.SYSVAR_RENT_PUBKEY.toBase58());
    console.log(masterEdition.toBase58());*/

    const tx = await program.methods.mintNft(
      mintKey.publicKey,
      "https://github.com/zigtur",
      "Zigtur Collection",
    )
      .accounts({
        mintAuthority: wallet.publicKey,
        mint: mintKey.publicKey,
        tokenAccount: NftTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
        metadata: metadataAddress,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        payer: wallet.publicKey,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        masterEdition: masterEdition,
        useAuthorityRecord: authorityRecord,
        verifier: verifierKey.publicKey,
        burner: burnerAddress,
      },
      )
      .rpc();
    console.log("Your transaction signature", tx);
    console.log("Metadata of NFT: ", await Metadata.fromAccountAddress(provider.connection, metadataAddress));
    console.log("Uses of NFT: ", (await Metadata.fromAccountAddress(provider.connection, metadataAddress)).uses);

    // Verifier part
    const SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID: anchor.web3.PublicKey = new anchor.web3.PublicKey(
      'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL',
    );
    
    function findAssociatedTokenAddress(
        walletAddress: anchor.web3.PublicKey,
        tokenMintAddress: anchor.web3.PublicKey
    ): anchor.web3.PublicKey {
        return anchor.web3.PublicKey.findProgramAddressSync(
            [
                walletAddress.toBuffer(),
                TOKEN_PROGRAM_ID.toBuffer(),
                tokenMintAddress.toBuffer(),
            ],
            SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID
        )[0];
    }


    // AIRDROP IF NEEDED
    /*const txAirdrop = await program.provider.connection.requestAirdrop(
      verifierKey.publicKey,
      anchor.web3.LAMPORTS_PER_SOL * 1
    );
  
    const latestBlockHash = await program.provider.connection.getLatestBlockhash();
    await program.provider.connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: txAirdrop
    });*/


    const verifProgram = programPaidBy(verifierKey);
    console.log("ATA address: ", findAssociatedTokenAddress(wallet.publicKey, mintKey.publicKey));
    const tx2 = await verifProgram.methods.verifyNft(
    )
      .accounts({
        verifier: verifierKey.publicKey, //wallet.publicKey,
        owner: wallet.publicKey,
        mint: mintKey.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        metadata: metadataAddress,
        tokenAccount: NftTokenAccount,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        ataProgram: SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        useAuthorityRecord: authorityRecord,
        burner: burnerAddress,
      },
      )
      .rpc();
    console.log("Your transaction signature", tx2);
    console.log("Uses of NFT: ", (await Metadata.fromAccountAddress(provider.connection, metadataAddress)).uses);
    

  });
});
