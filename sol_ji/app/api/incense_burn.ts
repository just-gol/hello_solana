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
  const [pda] = getUserBurnInfo(wallet);
  return await program.account.userBurnInfo.fetch(pda);
}

export async function destroy(wallet: anchor.Wallet, name: string) {
  return await program.methods.destroy()
    .accounts({
      authority: wallet.payer.publicKey,
      nftMintAccount: getNftMintAccount(name),
    })
    .rpc();
}