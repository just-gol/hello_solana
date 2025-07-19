import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MultiUserCounter } from "../target/types/multi_user_counter";

describe("multi_user_counter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const wallet = anchor.Wallet.local();

  const keypaid = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(
      [171, 85, 89, 69, 0, 100, 63, 168, 190, 100, 171, 91, 65, 51, 232, 199, 49, 80, 183, 43, 173, 99, 142, 120, 187, 154, 79, 247, 18, 167, 194, 150, 183, 191, 42, 203, 56, 254, 168, 210, 174, 37, 73, 15, 48, 143, 33, 139, 111, 205, 200, 214, 217, 146, 123, 78, 168, 20, 12, 244, 110, 163, 124, 115]
    )
  );
  const wallet2 = new anchor.Wallet(keypaid);

  const program = anchor.workspace.multiUserCounter as Program<MultiUserCounter>;

  it("Is increment!", async () => {
    await program.methods.increment(wallet.publicKey).accounts({
      authority: wallet.publicKey,
    }).signers([wallet.payer]).rpc();


    const [pda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("counter"), wallet.publicKey.toBuffer()], program.programId);

    await program.account.counter.fetch(pda).then(console.log);

    await program.methods.decriment(wallet.publicKey).accounts({
      authority: wallet.publicKey,
    }).signers([wallet.payer]).rpc();

    await program.account.counter.fetch(pda).then(console.log);


  });
});
