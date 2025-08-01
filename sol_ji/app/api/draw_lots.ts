import * as anchor from "@coral-xyz/anchor";
import { program, provider } from "./wallet";
import { getLotteryArrayPda, getNftMintAccount, getLotteryCountPda, getLotteryRecordPda } from "./address";

export async function initializeLotteryPoetry() {
  await program.methods.initializeLotteryPoetry()
    .accounts({
    })
    .rpc();
  const [pda] = getLotteryArrayPda();
  return await program.account.lotteryConfig.fetch(pda);
}

export async function drawLots(name: string, count: number, wallet: anchor.Wallet) {
  let pda = getLotteryRecordPda(count, wallet);
  let r = await program.methods.drawLots(count, new anchor.BN(5))
    .accounts({
      nftMintAccount: getNftMintAccount(name),
      lotteryRecord: pda,
    })
    .rpc();

  let r1 = await program.account.lotteryCount.fetch(getLotteryCountPda(wallet));
  let r2 = await program.account.lotteryRecord.fetch(pda);
  return [r, r1, r2];
}