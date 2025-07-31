import * as anchor from "@coral-xyz/anchor";
import { program, provider } from "./wallet";
import { PublicKey } from "@solana/web3.js";
// 获取烧香规则pda
export function getIncenseRulesConfig() {
  return anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("incense_rules_config")], program.programId);
}

// 获取 nft_mint_account pda

export function getNftMintAccount(name: string) {
  const [nftMintAccount] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("create_burn_token"), Buffer.from(name)], program.programId);
  return nftMintAccount;
}

// 
export function getUserBurnInfo(seeds: string | PublicKey) {
  let nftMintAccount: PublicKey
  if (typeof seeds === "string") {
    nftMintAccount = getNftMintAccount(seeds)
  } else {
    nftMintAccount = seeds
  }
  return anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("user_burn_info"), nftMintAccount.toBuffer()], program.programId);
}