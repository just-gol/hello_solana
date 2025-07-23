import *  as anchor from "@coral-xyz/anchor";
import { program } from "./wallet";
import { getNftMintAccount } from "./accounts";

export async function stakeNft(
  wallet: anchor.Wallet,
  id: string,
) {
  return await program.methods.stake()
    .accounts({
      nftMintAccount: getNftMintAccount(id),
      authority: wallet.payer.publicKey,
    })
    .signers([wallet.payer])
    .rpc()
}

export async function nftUnStake(
  wallet: anchor.Wallet,
  id: string,
) {
  return await program.methods.unstake()
    .accounts({
      nftMintAccount: getNftMintAccount(id),
      authority: wallet.payer.publicKey,
    })
    .signers([wallet.payer])
    .rpc()
}