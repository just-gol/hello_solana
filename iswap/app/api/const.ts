import * as anchor from "@coral-xyz/anchor";
import { Iswap } from "../../target/types/iswap";
import { Program } from "@coral-xyz/anchor";

let provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.iswap as Program<Iswap>;

export { program, provider };

export function useDefaultWallet() {
  return anchor.Wallet.local();
}