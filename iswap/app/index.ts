import { createETF, tokenBurn, tokenMint } from "./api/etf_token";
import { useDefaultWallet } from "./api/const";
import * as anchor from "@coral-xyz/anchor";
import { deriveEtfTokenMintAccount } from "./api/address";

(async () => {

  const defaultWallet = useDefaultWallet();
  // const [name, symbol, description, url] = ["yama16", "YAMA16", "yama description", "https://note-public-img.oss-cn-beijing.aliyuncs.com/nya/nya.json"];
  // const tx = await createETF(
  //   defaultWallet,
  //   name,
  //   symbol,
  //   description,
  //   url,
  //   [
  //     {
  //       token: new anchor.web3.PublicKey("A34AwLZ5jFMssAfGoK6K1DtWkKo7zUodKwY3CRPLSFkq"),
  //       weight: 10,
  //     },
  //     {
  //       token: new anchor.web3.PublicKey("9nPghmDu9hjrRHcZXyXMmNakywU7Mk6AP8ACKNJDGYVg"),
  //       weight: 90,
  //     }
  //   ]
  // )
  // console.log(tx);
  const [eft,] = deriveEtfTokenMintAccount("YAMA16");
  console.log(`ETF token mint account: ${eft.toString()}`);

  // const r2 = await tokenMint(defaultWallet, eft, 10000000000);
  // console.log(`Minted 10 YAMA16 tokens, tx: ${r2}`);

  const r3 = await tokenBurn(defaultWallet, eft, 10000000000)
  console.log(`Burned 10 YAMA16 tokens, tx: ${r3}`);

})()