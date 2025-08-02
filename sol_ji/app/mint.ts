import { incenseBurn, destroy, getInfo } from "./api/incense_burn";
import { initialize, updateIncense } from "./api/incense_config";
import { nftMint } from "./api/nft_mint";
import { getWallet } from "./api/wallet";
(async () => {
  const wallet = getWallet();

  const name = "Test NFT 01";

  const r2 = await updateIncense();
  console.log("Update Incense Result:", r2);

  const r3 = await nftMint(name, "TNFT", "https://poor-gold-wildebeest.myfilebase.com/ipfs/QmPCWecKXa6darBrnsKuveDqyhYxFGcnJZzoo7fkFFn6oS");
  console.log("NFT Mint Result:", r3);

  const r4 = await incenseBurn(wallet, name);
  console.log("Burn Result:", r4);

  const r5 = await getInfo(wallet);
  console.log("User:", r5);

  const r6 = await destroy(wallet, "Test NFT 11");
  console.log("Destroy Result:", r6);
})()