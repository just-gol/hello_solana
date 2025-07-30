import * as anchor from "@coral-xyz/anchor";
import { program, provider } from "./wallet";

export async function nftMint(name: string, symbol: string, url: string) {
  return await program.methods.nftMint(
    {
      name: name,
      symbol: symbol,
      url: url,
    }
  ).accounts({})
    .rpc();
}