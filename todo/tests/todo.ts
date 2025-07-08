import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Todo } from "../target/types/todo";
import { expect } from "chai";

describe("todo", () => {
  // 获取连接+钱包
  const provider = anchor.AnchorProvider.env();
  // 设置全局 provider
  anchor.setProvider(provider);
  // 获取 todo 程序实例
  const program = anchor.workspace.todo as Program<Todo>;


  // 生成一个新的 Keypair 作为 todo 账户
  const todoAccount = anchor.web3.Keypair.generate();

  it("Initialize todo account", async () => {
    const title = "my first todo";
    await program.methods.initialize(title).accounts({
      todoAccount: todoAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    } as any).signers([todoAccount]).rpc();
    // 读取账户数据验证
    const account = await program.account.todoAccount.fetch(todoAccount.publicKey);
    console.log("Todo title:", account.title);
    expect(account.title).to.equal(title);
    expect(account.item.length).to.equal(0);
  })

  it("add item to todo", async () => {
    const newItem = "solana";
    await program.methods.addItem(newItem).accounts({
      todoAccount: todoAccount.publicKey,
      user: provider.wallet.publicKey,
    }).rpc();
    const account = await program.account.todoAccount.fetch(todoAccount.publicKey);
    expect(account.item.length).to.equal(1);
    expect(account.item[0]).to.equal("solana");
  })

  it("remove item to todo", async () => {
    await program.methods.removeItem(new anchor.BN(0)).accounts({
      todoAccount: todoAccount.publicKey,
      user: provider.wallet.publicKey,
    }).rpc();
    const account = await program.account.todoAccount.fetch(todoAccount.publicKey);
    expect(account.item.length).to.equal(0);
  })
});
