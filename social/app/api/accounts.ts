import * as anchor from "@coral-xyz/anchor";
import { program } from "./wallet";

export function getNftMintAccount(
    id: string,
) {
    const [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("nft"),
            Buffer.from(id),
        ],
        program.programId,
    );

    return mintAccount;
}