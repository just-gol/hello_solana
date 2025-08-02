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
export function getUserBurnInfo(wallet: anchor.Wallet) {
  // let nftMintAccount: PublicKey
  // if (typeof seeds === "string") {
  //   nftMintAccount = getNftMintAccount(seeds)
  // } else {
  //   nftMintAccount = seeds
  // }
  // return anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("user_burn_info"), nftMintAccount.toBuffer()], program.programId);
  return anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("user_burn_info"), wallet.publicKey.toBuffer()], program.programId);
}

// 签文的pda
export function getLotteryArrayPda() {
  return anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("lottery_array")], program.programId);
}

// 抽签次数 pda
export function getLotteryCountPda(wallet: anchor.Wallet) {
  let [pda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("lottery_count"), wallet.publicKey.toBuffer()], program.programId);
  return pda;
}

export function getLotteryRecordPda(count: number, wallet: anchor.Wallet) {
  let [pda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("lottery_record"),
      wallet.publicKey.toBuffer(),
      Buffer.from(`${count + 1}`),
    ],
    program.programId
  );
  return pda;
}

// wish_user pda
export function getWishUserPda(wallet: anchor.Wallet) {
  let [pda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("wish_user"), wallet.publicKey.toBuffer()], program.programId);
  return pda;
}

// publish_wish PDA
export function getPublishWishPda(count: number, wallet: anchor.Wallet) {
  let wishUserPda = getWishUserPda(wallet);
  let [pda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("publish_wish"), wishUserPda.toBuffer(), Buffer.from(`${count + 1}`),], program.programId);
  return pda;
}