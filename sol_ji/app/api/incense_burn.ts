import * as anchor from "@coral-xyz/anchor";
import { program, provider } from "./wallet";
import { getUserBurnInfo, getNftMintAccount } from "./address";

export async function incenseBurn(wallet: anchor.Wallet, name: string) {
  return await program.methods.incenseBurn(
    { orangeFragrance: {} },
  )
    .accounts({
      authority: wallet.payer.publicKey,
      nftMintAccount: getNftMintAccount(name),
    })
    .rpc();

  // const [pda] = getUserBurnInfo(name);
  // const fetch = await program.account.userBurnInfo.fetch(pda);
  // return [result, fetch];
}

export async function getInfo(wallet: anchor.Wallet) {
  const pda = getUserBurnInfo(wallet);
  console.log("user info pda:", pda.toBase58());
  let info = await program.account.userInfo.fetch(pda);
  return {
    user: info.user.toBase58(),
    burnCount: info.burnCount,
    meritValue: info.meritValue.toNumber(),
    incenseValue: info.incenseValue.toNumber(),
    lastUpdateTime: new Date(info.lastUpdateTime.toNumber() * 1000).toLocaleString(),
    lotteryCount: info.lotteryCount,
    lotteryIsFree: info.lotteryIsFree,
    lotteryTime: new Date(info.lotteryTime.toNumber() * 1000).toLocaleString(),
    wishTotalCount: info.wishTotalCount,
    wishUpdateTime: new Date(info.wishUpdateTime.toNumber() * 1000).toLocaleString(),
    wishDailyCount: info.wishDailyCount,
    createAt: new Date(info.createAt.toNumber() * 1000).toLocaleString(),
  };
}

export async function destroy(wallet: anchor.Wallet, name: string) {
  return await program.methods.destroy()
    .accounts({
      authority: wallet.payer.publicKey,
      nftMintAccount: getNftMintAccount(name),
    })
    .rpc();
}