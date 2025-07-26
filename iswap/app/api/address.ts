
import * as anchor from "@coral-xyz/anchor";
import { program } from "./const";

export function deriveEtfTokenMintAccount(symbol: string) {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("ETF_token_v3"), Buffer.from(symbol)],
    program.programId
  );

}

export function deriveEtfInfoAccount(symbol: string) {
  const [mintAccount,] = deriveEtfTokenMintAccount(symbol);

  return anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("ETF_info_v3"), mintAccount.toBuffer()],
    program.programId
  );

}