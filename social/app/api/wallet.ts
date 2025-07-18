import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Social } from "../../target/types/social";


// 使用环境变量中定义的 RPC 节点 + 钱包，创建 provider
let provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.social as Program<Social>;

export { program, provider };

// 获取本地钱包地址
export function useDefauleWallet() {
  return anchor.Wallet.local();
}

// pub-key:DNGgKTBT138MftmLsTd19CD9mhbEehcM9Kp1pd6ik5EA
export function useVisitorWallet() {
  const keypaid = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(
      [171, 85, 89, 69, 0, 100, 63, 168, 190, 100, 171, 91, 65, 51, 232, 199, 49, 80, 183, 43, 173, 99, 142, 120, 187, 154, 79, 247, 18, 167, 194, 150, 183, 191, 42, 203, 56, 254, 168, 210, 174, 37, 73, 15, 48, 143, 33, 139, 111, 205, 200, 214, 217, 146, 123, 78, 168, 20, 12, 244, 110, 163, 124, 115]
    )
  );
  console.log("visitor pubkey:", keypaid.publicKey.toBase58());
  return new anchor.Wallet(keypaid);
}