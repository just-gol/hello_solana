import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DecentralizedMessageBoard } from "../target/types/decentralized_message_board";

describe("decentralized_message_board", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.decentralizedMessageBoard as Program<DecentralizedMessageBoard>;

  const wallet = anchor.Wallet.local();

  it("Is initialized!", async () => {
    // Add your test here.
    await program.methods.pushBoard("小明", "今天在干嘛").accounts({
      user: wallet.publicKey,
    }).signers([wallet.payer]).rpc();

    await program.methods.pushBoard("小华", "今天在干嘛").accounts({
      user: wallet.publicKey,
    }).signers([wallet.payer]).rpc();

    const [pda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("board"), wallet.publicKey.toBuffer()],
      program.programId
    );

    await program.account.board.fetch(pda).then(board => {
      console.log(board.messages);
    })
  });
});
