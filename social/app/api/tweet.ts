import *  as anchor from "@coral-xyz/anchor";
import { program } from "./wallet";

export async function createTweet(wallet: anchor.Wallet, body: string)
  : Promise<[anchor.web3.PublicKey, string]> {

  if (Buffer.from(body).length > 50) {
    throw new Error("Tweet body too long");
  }

  const [profilePDA,] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("profile"),
      wallet.publicKey.toBuffer()
    ], program.programId);

  const profile = await program.account.solanaTwitterProfile.fetch(profilePDA);

  const [tweetPda,] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("tweet"),
      profilePDA.toBuffer(),
      Buffer.from(`${profile.tweetCount + 1}`),
    ],
    program.programId,
  );

  const resp = await program.methods.createTweet(body)
    .accounts({
      authority: wallet.publicKey,
      tweet: tweetPda,
      systemProgram: anchor.web3.SystemProgram.programId,
    } as any)
    .rpc();

  return [
    tweetPda,
    resp,
  ];

}

export async function getTweet(tweetPda: anchor.web3.PublicKey) {
  return await program.account.solanaTwitterTweet.fetch(tweetPda);
}

export async function createLike(wallet: anchor.Wallet, tweetPda: anchor.web3.PublicKey) {
  const [profilePDA,] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("profile"),
      wallet.publicKey.toBuffer()
    ], program.programId);


  const [likePDA,] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("like"), profilePDA.toBuffer(), tweetPda.toBuffer()],
    program.programId
  );

  const tweet = await program.account.solanaTwitterTweet.fetch(tweetPda);

  return await program.methods.createLike().accounts({
    authorWallet: tweet.author,
    authority: wallet.publicKey,
    like: likePDA,
    tweet: tweetPda,
    systemProgram: anchor.web3.SystemProgram.programId,
  } as any).signers([wallet.payer])
    .rpc();
}