import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TransferSol } from "../target/types/transfer_sol";
import { getAssociatedTokenAddress } from "@solana/spl-token";


describe("transfer_sol", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const wallet1 = anchor.Wallet.local();

  const keypaid = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(
      [171, 85, 89, 69, 0, 100, 63, 168, 190, 100, 171, 91, 65, 51, 232, 199, 49, 80, 183, 43, 173, 99, 142, 120, 187, 154, 79, 247, 18, 167, 194, 150, 183, 191, 42, 203, 56, 254, 168, 210, 174, 37, 73, 15, 48, 143, 33, 139, 111, 205, 200, 214, 217, 146, 123, 78, 168, 20, 12, 244, 110, 163, 124, 115]
    )
  );

  const wallet2 = new anchor.Wallet(keypaid);

  const program = anchor.workspace.transferSol as Program<TransferSol>;

  const [mintPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("mint")],
    program.programId
  );

  it("Is transfer!", async () => {

    const ata1 = await getAssociatedTokenAddress(mintPda, wallet1.publicKey);
    const ata2 = await getAssociatedTokenAddress(mintPda, wallet2.publicKey);
    console.log(ata1.toBase58(), ata2.toBase58());
    let result = await program.methods.transfer()
      .accounts({
        from: wallet1.publicKey,
        secondOwner: wallet2.publicKey,
      })
      .signers([wallet1.payer])
      .rpc();
    console.log(result);
  });
});
