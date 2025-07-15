import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { expect } from "chai";


describe("counter", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // 获取结构体
  const program = anchor.workspace.counter as Program<Counter>;

  // 获取钱包
  const wallet = provider.wallet as anchor.Wallet;

  // 全局生成一个Keypair，代表测试中的counter账户
  // const counterKeypair = anchor.web3.Keypair.generate();

  // const counterKeypair = new anchor.web3.Keypair;

  const [counterKeypair, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("counter"), wallet.publicKey.toBytes()],
    program.programId
  );

  before(async () => {
    console.log("pda:{}", counterKeypair);
    const tx = await program.methods.initialize()
      .accounts({
        payer: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([wallet.payer])
      .rpc();
    console.log("Your transaction signature", tx);

  })

  it("Increment once ", async () => {
    await program.methods.increment()
      .accounts({
        counter: counterKeypair,
      } as any)
      .rpc();
    const data = await program.account.counter.fetch(counterKeypair);
    console.log("Counter value:", data.count); // 应该是 1
  });

  it("Increment twice ", async () => {
    await program.methods.increment()
      .accounts({
        counter: counterKeypair,
      } as any)
      .rpc();
    const data = await program.account.counter.fetch(counterKeypair);
    console.log("Counter value:", data.count); // 应该是 2
  });
});
