import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ContentPlatform } from "../target/types/content_platform";
import { getAssociatedTokenAddress } from "@solana/spl-token";


describe("content_platform", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.contentPlatform as Program<ContentPlatform>;
  const wallet1 = anchor.Wallet.local();

  // DNGgKTBT138MftmLsTd19CD9mhbEehcM9Kp1pd6ik5EA
  const keypaid = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(
      [171, 85, 89, 69, 0, 100, 63, 168, 190, 100, 171, 91, 65, 51, 232, 199, 49, 80, 183, 43, 173, 99, 142, 120, 187, 154, 79, 247, 18, 167, 194, 150, 183, 191, 42, 203, 56, 254, 168, 210, 174, 37, 73, 15, 48, 143, 33, 139, 111, 205, 200, 214, 217, 146, 123, 78, 168, 20, 12, 244, 110, 163, 124, 115]
    )
  );
  // 打款人
  const wallet2 = new anchor.Wallet(keypaid);


  // D4R5RFZKD33rShKsXX8rMcjviuFFJABZUZQobyZXqpyA
  const keypaid2 = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(
      [112, 61, 165, 90, 31, 223, 150, 177, 110, 197, 198, 163, 116, 172, 232, 38, 42, 224, 22, 20, 70, 84, 177, 210, 96, 78, 44, 34, 160, 230, 132, 191, 179, 44, 53, 100, 60, 141, 127, 97, 53, 233, 173, 41, 142, 227, 167, 74, 227, 146, 233, 135, 169, 157, 4, 64, 28, 73, 83, 39, 7, 39, 248, 181]
    )
  );
  // 作者
  const wallet3 = new anchor.Wallet(keypaid2);

  // it("Is createArticle!", async () => {
  //   // Add your test here.
  //   await program.methods.createArticle("solana", "加油努力")
  //     .accounts({
  //       author: wallet3.publicKey,
  //     }).signers([wallet3.payer]).rpc();

  //   await program.methods.viewArticles(new anchor.BN(0), new anchor.BN(1))
  //     .accounts({
  //       author: wallet1.publicKey,
  //     }).rpc();
  // });


  // it("Is create_mint!", async () => {
  //   const [mindPad] = anchor.web3.PublicKey.findProgramAddressSync([
  //     Buffer.from("mint_account"),
  //   ], program.programId);

  //   const ata = await getAssociatedTokenAddress(mindPad, wallet2.publicKey);
  //   console.log("=============>", ata.toBase58());
  //   await program.methods.createMint()
  //     .accounts({
  //       payer: wallet1.publicKey,
  //       giverAtaWallet: wallet2.publicKey,
  //     }).signers([wallet1.payer]).rpc();

  // });

  // it("Is reward_author!", async () => {
  //   await program.methods.rewardAuthor()
  //     .accounts({
  //       payer: wallet1.publicKey,
  //       giverAta: "jqEKUY1s7cJYpKRiQCNKNKvginKGbTrjeoHkfS2ZP7G",
  //       giverAtaWallet: wallet2.publicKey,
  //     }).signers([wallet1.payer, wallet2.payer]).rpc();

  // });

  it("Is withdraw!", async () => {
    await program.methods.withdraw()
      .accounts({
        payer: wallet1.publicKey,
        authorWallet: wallet3.publicKey,
      }).signers([wallet1.payer]).rpc();
  })
});
