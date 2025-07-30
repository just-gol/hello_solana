import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolJi } from "../..//target/types/sol_ji";


// Configure the client to use the local cluster.
const provider = anchor.AnchorProvider.env();

anchor.setProvider(provider);

const program = anchor.workspace.solJi as Program<SolJi>;

export { program, provider };


export function getWallet() {
  return anchor.Wallet.local();
}