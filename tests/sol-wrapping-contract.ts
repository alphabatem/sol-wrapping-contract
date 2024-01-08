import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolWrappingContract } from "../target/types/sol_wrapping_contract";
import {TOKEN_PROGRAM_ID, TOKEN_2022_PROGRAM_ID, getAssociatedTokenAddressSync, NATIVE_MINT, ASSOCIATED_TOKEN_PROGRAM_ID} from "@solana/spl-token";
import { Connection, clusterApiUrl, SystemProgram } from '@solana/web3.js';
import { getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
// import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";

export function loadWalletKey(keypairFile:string): anchor.web3.Keypair {
  if (!keypairFile || keypairFile == '') {
    throw new Error('Keypair is required!');
  }
  const fs = require("fs");
  const loaded = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(fs.readFileSync(keypairFile).toString())),
  );
  return loaded;
}

describe("sol-wrapping-contract", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

  const program = anchor.workspace.SolWrappingContract as Program<SolWrappingContract>;

  let owner = loadWalletKey("/Users/Temitope/.config/solana/id.json");

  let other_mint = new anchor.web3.PublicKey("CBY7ngsg3hGGusRCW2oat42w2eWTvAfGvw1ifLphJoT3");


  let amount = 1 * 1e9; /* Wrapped SOL's decimals is 9 */

  it("Is wrapping sol!", async () => {
    // Add your test here.

    const wsol_ata = await getOrCreateAssociatedTokenAccount(
      connection, 
      owner,
      NATIVE_MINT,
      owner.publicKey,
      true,
      null,
      null,
      TOKEN_PROGRAM_ID
    );

    const tx = await program.methods.wrap(new anchor.BN(amount)).accounts({
      owner: owner.publicKey,
      wsolAccount: wsol_ata.address,
      nativeMint: NATIVE_MINT,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId
    }).rpc({skipPreflight: true});
    console.log("Your transaction signature", tx);
  });

  it("Is unwrapping sol!", async () => {
    // Add your test here.

    const wsol_ata = await getOrCreateAssociatedTokenAccount(
      connection, 
      owner,
      NATIVE_MINT,
      owner.publicKey,
      true,
      null,
      null,
      TOKEN_PROGRAM_ID
    );

    const tx = await program.methods.unwrap().accounts({
      owner: owner.publicKey,
      wsolAccount: wsol_ata.address,
      mint: NATIVE_MINT,
      tokenProgram: TOKEN_PROGRAM_ID,
    }).rpc({skipPreflight: true});
    console.log("Your transaction signature", tx);
  });

  it("Is closing other token account!", async () => {
    // Add your test here.

    const ata = await getOrCreateAssociatedTokenAccount(
      connection, 
      owner,
      other_mint,
      owner.publicKey,
      true,
      null,
      null,
      TOKEN_PROGRAM_ID
    );

    const tx = await program.methods.unwrap().accounts({
      owner: owner.publicKey,
      wsolAccount: ata.address,
      mint: other_mint,
      tokenProgram: TOKEN_PROGRAM_ID,
    }).rpc({skipPreflight: true});
    console.log("Your transaction signature", tx);
  });
});
