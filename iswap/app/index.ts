import { createETF, tokenMint } from "./api/etf_token";
import { useDefaultWallet } from "./api/const";
import * as anchor from "@coral-xyz/anchor";
import { deriveEtfTokenMintAccount } from "./api/address";

(async () => {

  const defaultWallet = useDefaultWallet();
  const [name, symbol, description, url] = ["yama15", "YAMA15", "yama description", "https://note-public-img.oss-cn-beijing.aliyuncs.com/nya/nya.json"];
  const tx = await createETF(
    defaultWallet,
    name,
    symbol,
    description,
    url,
    [
      {
        token: new anchor.web3.PublicKey("ACFknvjfQ4qZdnePwJvGdhVR5heGaQRbXrN9CWJSqK2f"),
        weight: 10,
      },
      {
        token: new anchor.web3.PublicKey("FsniKpaLXjhV4DGUpPKQNzu5vsGuJQ5Q2VaTds5a1BkR"),
        weight: 90,
      }
    ]
  )
  console.log(tx);
  const [eft,] = deriveEtfTokenMintAccount("YAMA15");
  console.log(`ETF token mint account: ${eft.toString()}`);
  const r2 = await tokenMint(defaultWallet, eft, 10000000000);
  console.log(`Minted 10 YAMA15 tokens, tx: ${r2}`);

})()