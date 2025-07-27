
import * as anchor from "@coral-xyz/anchor";
import { program } from "./const";
import { PublicKey } from "@solana/web3.js";

export function deriveEtfTokenMintAccount(symbol: string) {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("ETF_token_v3"), Buffer.from(symbol)],
    program.programId
  );

}

export function deriveEtfInfoAccount(seeds: string | PublicKey) {
  let mintAccount: PublicKey;
  if (typeof seeds === "string") {
    [mintAccount,] = deriveEtfTokenMintAccount(seeds);
  } else {
    mintAccount = seeds;
  }

  return anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("ETF_token_v3"), mintAccount.toBuffer()],
    program.programId
  );

}
