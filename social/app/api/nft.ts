import *  as anchor from "@coral-xyz/anchor";
import { program } from "./wallet";

export async function nftMint(
  wallet: anchor.Wallet,
  id: string,
) {
  return await program.methods.nftMint(id)
    .accounts({})
    .signers([wallet.payer])
    .rpc()
}
