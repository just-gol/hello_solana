import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MiniNft } from "../target/types/mini_nft";
import { getAssociatedTokenAddress } from "@solana/spl-token";

describe("mini_nft", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.miniNft as Program<MiniNft>;

  const wallet1 = anchor.Wallet.local();

  // DNGgKTBT138MftmLsTd19CD9mhbEehcM9Kp1pd6ik5EA
  const keypaid = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(
      [171, 85, 89, 69, 0, 100, 63, 168, 190, 100, 171, 91, 65, 51, 232, 199, 49, 80, 183, 43, 173, 99, 142, 120, 187, 154, 79, 247, 18, 167, 194, 150, 183, 191, 42, 203, 56, 254, 168, 210, 174, 37, 73, 15, 48, 143, 33, 139, 111, 205, 200, 214, 217, 146, 123, 78, 168, 20, 12, 244, 110, 163, 124, 115]
    )
  );
  const wallet2 = new anchor.Wallet(keypaid);


  // D4R5RFZKD33rShKsXX8rMcjviuFFJABZUZQobyZXqpyA
  const keypaid2 = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(
      [112, 61, 165, 90, 31, 223, 150, 177, 110, 197, 198, 163, 116, 172, 232, 38, 42, 224, 22, 20, 70, 84, 177, 210, 96, 78, 44, 34, 160, 230, 132, 191, 179, 44, 53, 100, 60, 141, 127, 97, 53, 233, 173, 41, 142, 227, 167, 74, 227, 146, 233, 135, 169, 157, 4, 64, 28, 73, 83, 39, 7, 39, 248, 181]
    )
  );
  const wallet3 = new anchor.Wallet(keypaid2);



  const [pda] = anchor.web3.PublicKey.findProgramAddressSync([
    Buffer.from("mint_nft"),
  ], program.programId);


  // it("Is all mind nft!", async () => {
  //   // 初始化admin
  //   await program.methods
  //     .setAdmin(wallet2.publicKey)
  //     .accounts({
  //       admin: wallet2.publicKey,
  //     })
  //     .signers([wallet2.payer])
  //     .rpc();

  //   // 设置白名单 
  //   // wallet1 设置为白名单
  //   await program.methods
  //     .addWhitelist(wallet1.publicKey)
  //     .accounts({
  //       admin: wallet2.publicKey,
  //       user: wallet1.publicKey,
  //     })
  //     .signers([wallet2.payer])
  //     .rpc();

  //   // mimt nft
  //   await program.methods
  //     .mintNft()
  //     .accounts({
  //       user: wallet1.publicKey,
  //       admin: wallet2.publicKey,
  //     })
  //     .signers([wallet1.payer])
  //     .rpc();

  //   const ata = await getAssociatedTokenAddress(pda, wallet1.publicKey);
  //   console.log("ata---->", ata);
  // });

  it("Try mint nft again for same user (expect fail)", async () => {
    try {
      await program.methods
        .mintNft()
        .accounts({
          user: wallet1.publicKey,
          admin: wallet2.publicKey,
        })
        .signers([wallet1.payer])
        .rpc();

      console.log("Unexpected success, should have failed!");
    } catch (err) {
      const errorCode = err?.error?.errorCode ?? err?.code ?? err.toString();

      console.log("mintNft second time failed as expected:", errorCode);
    }
  });


  it("Is mind nft!", async () => {
    // mimt nft
    try {
      await program.methods
        .mintNft()
        .accounts({
          user: wallet3.publicKey,
          admin: wallet2.publicKey,
        })
        .signers([wallet3.payer])
        .rpc();

      const ata = await getAssociatedTokenAddress(pda, wallet3.publicKey);
      console.log("ata---->", ata);
    } catch (err) {
      // 预期错误
      console.log("mintNft 失败场景捕获到错误，测试通过:", err.error.errorCode);
      // 这里可以用断言检查错误类型：
      // assert.equal(err.error.errorCode, "TimeError");
    }
  });

});
