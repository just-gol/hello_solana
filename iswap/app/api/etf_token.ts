import { program, provider } from "./const";
import * as anchor from "@coral-xyz/anchor";

import { PublicKey } from "@solana/web3.js";
import { createAssociatedTokenAccountInstruction, getAccount, getAssociatedTokenAddressSync, getMint, TokenAccountNotFoundError } from "@solana/spl-token";
import { deriveEtfInfoAccount } from "./address";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { mplTokenMetadata, fetchDigitalAsset } from "@metaplex-foundation/mpl-token-metadata";
import { fromWeb3JsPublicKey } from "@metaplex-foundation/umi-web3js-adapters";

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
        // etfTokenInfoAddress 持有的 item.token 的 ATA 地址
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

// 查询资产信息
export async function etfList() {
  const etfInfo = await program.account.etfToken.all();
  return etfInfo;
}

// 查询资产详情
export async function etfDetail(
  wallet: anchor.Wallet,
  etfAddress: PublicKey) {
  // 通过 etfAddress 获取 etfTokenInfoAddress
  const [etfTokenInfoAccount] = deriveEtfInfoAccount(etfAddress);
  // 获取资产
  const etfInfo = await program.account.etfToken.fetch(etfTokenInfoAccount);
  // 获取mint account 信息
  // const mintAccount = getMint(provider.connection, etfAddress);
  // 安装 pnpm install @metaplex-foundation/umi-bundle-defaults
  // 获取metadata信息
  const umi = createUmi(provider.connection.rpcEndpoint);
  // pnpm install @metaplex-foundation/umi
  umi.use(mplTokenMetadata());
  //  pnpm install @metaplex-foundation/mpl-token-metadata
  // pnpm install @metaplex-foundation/umi-web3js-adapters
  const mintAccount = await fetchDigitalAsset(umi, fromWeb3JsPublicKey(etfAddress));
  let logo = ""
  if (mintAccount.metadata.uri) {
    const response = await fetch(mintAccount.metadata.uri);
    const rj = await response.json() as { image?: string };
    logo = rj.image
  }
  return {
    public_key: etfAddress.toString(),
    supply: mintAccount.mint.supply,
    decimals: mintAccount.mint.decimals,
    name: mintAccount.metadata.name,
    symbol: mintAccount.metadata.symbol,
    description: etfInfo.descriptor,
    creator: etfInfo.creator.toString(),
    create_at: etfInfo.createAt.toNumber(),
    logo: logo,
  }
}

export async function etfBalance(
  wallet: anchor.Wallet,
  etfAddress: PublicKey) {
  // 获取用户的 ata 地址
  const ata = getAssociatedTokenAddressSync(etfAddress, wallet.publicKey);
  try {
    // 获取用户的 ata 信息
    const account = await getAccount(provider.connection, ata);
    return account.amount;
  } catch (e) {
    console.error("❌ 获取用户的 ata 信息失败", e);
    return 0;
  }
}