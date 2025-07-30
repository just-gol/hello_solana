import * as anchor from "@coral-xyz/anchor";
import { program, provider } from "./wallet";
export function getIncenseRulesConfig() {
  return anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("incense_rules_config")], program.programId);
}


export function getUserBurnInfo(wallet: anchor.Wallet) {
  return anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("user_burn_info"), wallet.publicKey.toBuffer()], program.programId);
}