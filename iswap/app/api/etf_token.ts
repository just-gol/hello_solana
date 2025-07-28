import { program, provider } from "./const";
import * as anchor from "@coral-xyz/anchor";

import { PublicKey } from "@solana/web3.js";
import { createAssociatedTokenAccountInstruction, getAccount, getAssociatedTokenAddressSync, TokenAccountNotFoundError } from "@solana/spl-token";
import { deriveEtfInfoAccount } from "./address";
import { BN } from "bn.js";

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
    // 获取关联token  pda -- ata 地址
    /**
     *  mint: PublicKey, 这个 token 的 Mint 地址
        owner: PublicKey,  谁持有这个 token（钱包或 PDA）
        allowOwnerOffCurve = false, 如果是pda 则需要设置为 true
     */
    const address = getAssociatedTokenAddressSync(token, etfTokenInfoAddress, true);
    try {
      // 根据ata查找token account信息,如果pda没有被创建过，则会报错
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

export async function tokenMint(
  wallet: anchor.Wallet,
  etfAddress: PublicKey, // 购买的etf地址
  lamports: number) { // 购买ETF数量
  const [etfTokenInfoAddress,] = deriveEtfInfoAccount(etfAddress);
  console.log(`etfTokenInfoAddress: ${etfTokenInfoAddress.toBase58()}`);
  try {
    const etfInfo = await program.account.etfToken.fetch(etfTokenInfoAddress);
    const accounts = etfInfo.assets.flatMap((item) => {
      return [
        // 用户钱包ata
        getAssociatedTokenAddressSync(item.token, wallet.publicKey), // 用户的ata
        // 合约的ata
        getAssociatedTokenAddressSync(item.token, etfTokenInfoAddress, true) // 合约的ata
      ]
    })

    /**
     * 资产数量是动态的（etfInfo.assets.length），你没法在 .accounts() 里提前写死这些账户。
      这些 token account 会被程序 transfer 操作，所以必须标记为 isWritable: true。
      这些账户不是 signer，不需要 isSigner: true。
      必须顺序准确地列出，保持和 on-chain 程序中读取顺序一致。
     */
    return await program.methods
      .etfMint(new anchor.BN(lamports))
      .accounts({
        etfTokenMintAccount: etfAddress,
      })
      .remainingAccounts(accounts.map(item => (
        {
          pubkey: item,
          isSigner: false,
          isWritable: true,
        }
      )))
      .rpc();
  } catch (e) {
    console.error("❌ etfInfo 获取失败", e);
  }
}

export async function tokenBurn(
  wallet: anchor.Wallet,
  etfAddress: PublicKey, // token mint address pda
  lamports: number) {
  // 通过 etfAddress 获取 etfTokenInfoAddress
  const [etfTokenInfoAccount] = deriveEtfInfoAccount(etfAddress);
  // 获取资产
  const etfInfo = await program.account.etfToken.fetch(etfTokenInfoAccount);

  // 获取ata
  const accounts = etfInfo.assets.flatMap((item) => {
    return [
      // 用户钱包ata
      getAssociatedTokenAddressSync(item.token, wallet.publicKey), // 用户的ata
      // 合约的ata
      getAssociatedTokenAddressSync(item.token, etfTokenInfoAccount, true) // 合约的ata
    ]
  });

  return await program.methods.etfBurn(new anchor.BN(lamports)).accounts({ etfTokenMintAccount: etfAddress, })
    .remainingAccounts(accounts.map(item => (
      {
        pubkey: item,
        isSigner: false,
        isWritable: true,
      }
    ))).signers([wallet.payer])
    .rpc();
}