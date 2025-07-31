import * as anchor from "@coral-xyz/anchor";
import { program, provider } from "./wallet";
import { getUserBurnInfo, getNftMintAccount } from "./address";

export async function burn(wallet: anchor.Wallet, name: string) {
  return await program.methods.burn(
    { faintScent: {} },
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

export async function getInfo(name: string) {
  const [pda] = getUserBurnInfo(name);
  return await program.account.userBurnInfo.fetch(pda);
}