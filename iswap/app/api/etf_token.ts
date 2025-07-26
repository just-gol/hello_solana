import { program, provider } from "./const";
import * as anchor from "@coral-xyz/anchor";

import { PublicKey } from "@solana/web3.js";
import { createAssociatedTokenAccountInstruction, getAccount, getAssociatedTokenAddressSync, TokenAccountNotFoundError } from "@solana/spl-token";
import { deriveEtfInfoAccount } from "./address";

export async function createETF(
  wallet: anchor.Wallet,
  name: string,
  symbol: string,
  description: string,
  url: string,
  assets: { token: PublicKey, weight: number }[],
) {
  // 获取tokenInfo 地址
  const [etfTokenInfoAddress,] = deriveEtfInfoAccount(symbol);

  // 创建一个交易
  let tx = new anchor.web3.Transaction();

  // 创建ata
  for (const { token, } of assets) {
    // 获取关联token地址
    const address = getAssociatedTokenAddressSync(token, etfTokenInfoAddress, true);
    try {
      // 如果token没有被创建过，则会报错
      await getAccount(provider.connection, address)
    } catch (e) {
      // token 的 account 不存在，则创建一个
      if (e instanceof TokenAccountNotFoundError) {
        tx.add(
          createAssociatedTokenAccountInstruction(
            wallet.payer.publicKey,
            address,
            etfTokenInfoAddress,
            token,
          )
        )
        console.log(`Creating associated token account for ${token.toBase58()} at ${address.toBase58()}`);
      }
    }
  }

  tx.add(
    await program.methods.eftCreate(
      {
        name,
        symbol,
        description,
        url,
        assets
      }
    ).transaction()
  )

  return await anchor.web3.sendAndConfirmTransaction(
    provider.connection,
    tx,
    [wallet.payer],
  )
}
