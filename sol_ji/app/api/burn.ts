import * as anchor from "@coral-xyz/anchor";
import { program, provider } from "./wallet";
import { getUserBurnInfo } from "./address";

export async function burn(wallet: anchor.Wallet) {
  await program.methods.burn(
    { faintScent: {} },
  )
    .accounts({
    })
    .rpc();

  const [pda] = getUserBurnInfo(wallet);
  return await program.account.userBurnInfo.fetch(pda);
}