import * as anchor from "@coral-xyz/anchor";
import { Program, Wallet } from "@coral-xyz/anchor";
import { TicketcoinContract } from "../target/types/ticketcoin_contract";
import { TOKEN_PROGRAM_ID, createAssociatedTokenAccountInstruction, getAssociatedTokenAddress, createInitializeMintInstruction, MINT_SIZE, mintTo, mintToInstructionData, createMint } from '@solana/spl-token' // IGNORE THESE ERRORS IF ANY
import { Metadata, createCreateMetadataAccountV3Instruction } from "@metaplex-foundation/mpl-token-metadata";
const { SystemProgram } = anchor.web3;
import { keypairIdentity, Metaplex } from '@metaplex-foundation/js';



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

    // private Key of the collection authority
    const collectionAuthorityKey: anchor.web3.Keypair = anchor.web3.Keypair.fromSecretKey(
      Uint8Array.from([100,218,28,88,90,31,202,200,105,72,96,24,31,246,50,57,151,60,149,141,43,223,167,143,28,2,94,104,234,179,102,114,145,1,117,91,193,118,183,70,160,142,139,28,243,129,103,12,62,181,184,220,153,179,13,231,90,58,192,49,154,207,241,139])
    );

    //////////////////////////////////////////////
    //           1 - COLLECTION  PART           //
    //////////////////////////////////////////////
    //     WILL BE DONE BEFORE USER CAN MINT    //
    //////////////////////////////////////////////
    console.log("//////////////////////////////////////////////");
    console.log("//           1 - COLLECTION  PART           //");
    console.log("//////////////////////////////////////////////");
    console.log("//     WILL BE DONE BEFORE USER CAN MINT    //");
    console.log("//////////////////////////////////////////////");
    // Should be run once
    const metaplex = new Metaplex(provider.connection).use(keypairIdentity(collectionAuthorityKey));
    // A metaplex NFT does the job for the NFT collection
    const collectNFT = await metaplex.nfts().create({
      name: "Ed Sheeran - Exclusive Concert",
      sellerFeeBasisPoints: 0,
      uri: "https://github.com/zigtur",
      isMutable: false,
      isCollection: true,
    });

    console.log("Collection NFT address: ", collectNFT.nft.address);
    const collectNftPubKey = collectNFT.nft.address;

    


    //////////////////////////////////////////////
    //          2 - NFT MINTING PART            //
    //////////////////////////////////////////////
    //        WILL BE DONE IN THE USER APP      //
    //////////////////////////////////////////////

    console.log("//////////////////////////////////////////////");
    console.log("//          2 - NFT MINTING PART            //");
    console.log("//////////////////////////////////////////////");
    console.log("//        WILL BE DONE IN THE USER APP      //");
    console.log("//////////////////////////////////////////////");
    // Should be run once per user
    
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
    /*console.log(
      await program.provider.connection.getParsedAccountInfo(mintKey.publicKey)
    );*/

    console.log("NFT Account: ", res);
    //console.log("Mint key: ", mintKey.publicKey.toString());
    //console.log("User: ", wallet.publicKey.toString());

    const metadataAddress = await getMetadata(mintKey.publicKey);
    const masterEdition = await getMasterEdition(mintKey.publicKey);
    const authorityRecord = await getUseAuthority(mintKey.publicKey, verifierKey.publicKey);
    const burnerAddress = await getBurner();

    //console.log("Metadata address: ", metadataAddress.toBase58());
    //console.log("MasterEdition: ", masterEdition.toBase58());
    //console.log("AuthorityRecord: ", authorityRecord.toBase58());
    //console.log("burner: ", burnerAddress.toBase58());

    const tx = await program.methods.mintNft(
      mintKey.publicKey,
      "https://github.com/zigtur",
      "Zigtur Collection",
    )
      .accounts({
        mintAuthority: wallet.publicKey,
        collection: collectNftPubKey,
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
    //console.log("Your transaction signature", tx);
    //console.log("Metadata of NFT: ", await Metadata.fromAccountAddress(provider.connection, metadataAddress));
    console.log("Collection of NFT: ", (await Metadata.fromAccountAddress(provider.connection, metadataAddress)).collection);
    console.log("Uses of NFT: ", (await Metadata.fromAccountAddress(provider.connection, metadataAddress)).uses);

    

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

    //////////////////////////////////////////////
    //       3 - NFT COLLECTION VERIFY          //
    //////////////////////////////////////////////
    //  WILL BE DONE BY THE TICKETCOIN TEAM (NO APP?) //
    //////////////////////////////////////////////
    console.log("//////////////////////////////////////////////");
    console.log("//       3 - NFT COLLECTION VERIFY          //");
    console.log("//////////////////////////////////////////////");
    console.log("//  WILL BE DONE BY THE TICKETCOIN TEAM (NO APP?) //");
    console.log("//////////////////////////////////////////////");
    // Should be run once per user
    
    const collectionVerificationForNFT = await metaplex.nfts().verifyCollection({
      mintAddress: mintKey.publicKey,
      collectionMintAddress: collectNFT.mintAddress,
      collectionAuthority: collectionAuthorityKey,
    });
    console.log("Collection of NFT: ", (await Metadata.fromAccountAddress(provider.connection, metadataAddress)).collection);


    //////////////////////////////////////////////
    //          4 - VERIFICATION PART           //
    //////////////////////////////////////////////
    //      WILL BE DONE IN THE VERIFIER APP    //
    //////////////////////////////////////////////
    console.log("//////////////////////////////////////////////");
    console.log("//          4 - VERIFICATION PART           //");
    console.log("//////////////////////////////////////////////");
    console.log("//      WILL BE DONE IN THE VERIFIER APP    //");
    console.log("//////////////////////////////////////////////");
    
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

    const verifProgram = programPaidBy(verifierKey);
    
    //console.log("ATA address: ", findAssociatedTokenAddress(wallet.publicKey, mintKey.publicKey));
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
    //console.log("Your transaction signature", tx2);
    console.log("Uses of NFT after verifier verified it: ", (await Metadata.fromAccountAddress(provider.connection, metadataAddress)).uses);


  });
});
