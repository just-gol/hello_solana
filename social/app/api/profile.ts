import *  as anchor from "@coral-xyz/anchor";
import { program } from "./wallet";
export async function getProfile(wallet: anchor.Wallet) {
  const [profilePDA,] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("profile"), wallet.publicKey.toBuffer()], program.programId);

  return await program.account.solanaTwitterProfile.fetch(profilePDA);
}


export async function createProfile(wallet: anchor.Wallet, displayName: string) {
  return await program.methods.createProfile(displayName)
    .accounts({
      authority: wallet.publicKey
    })
    .signers([wallet.payer])
    .rpc();

}
