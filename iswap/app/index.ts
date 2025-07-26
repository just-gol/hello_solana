import { createETF } from "./api/etf_token";
import { useDefaultWallet } from "./api/const";
import * as anchor from "@coral-xyz/anchor";

(async () => {

  const defaultWallet = useDefaultWallet();
  const [name, symbol, description, url] = ["yama7", "YAMA7", "yama description", "https://note-public-img.oss-cn-beijing.aliyuncs.com/nya/nya.json"];
  const tx = await createETF(
    defaultWallet,
    name,
    symbol,
    description,
    url,
    [
      {
        token: new anchor.web3.PublicKey("HiGqeAow4UXzqo6Qdk9oBJfvp7s9XP9hb7ZN8cuPpVn"),
        weight: 10,
      },
      {
        token: new anchor.web3.PublicKey("DigipbbVz22c1LEhK5jaWdCtVn2XypdViQqBX6U6RqRi"),
        weight: 90,
      }
    ]
  )
  console.log(tx);
})()