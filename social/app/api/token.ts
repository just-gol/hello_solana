import *  as anchor from "@coral-xyz/anchor";
import { program } from "./wallet";


export async function createTokenMintAccount(wallet: anchor.Wallet) {
  const [splTokenPda] = anchor.web3.PublicKey.findProgramAddressSync([
    Buffer.from("mint_v9"),
  ], program.programId);

  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

  const resp = await program.methods.createTokenMintAccount()
    .accounts({

    })
    .rpc();
  return [splTokenPda, resp];
}