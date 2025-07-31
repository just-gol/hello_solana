import { incenseBurn, destroy, getInfo } from "./api/burn";
import { initialize, updateIncense } from "./api/incense_config";
import { nftMint } from "./api/nft_mint";
import { getWallet } from "./api/wallet";
(async () => {
  const wallet = getWallet();
  // const r1 = await initialize();
  // console.log("Initialization Result:", r1);

  // const r2 = await updateIncense();
  // console.log("Update Incense Result:", r2);

  // const r3 = await nftMint("Test NFT", "TNFT", "https://poor-gold-wildebeest.myfilebase.com/ipfs/QmPCWecKXa6darBrnsKuveDqyhYxFGcnJZzoo7fkFFn6oS");
  // console.log("NFT Mint Result:", r3);

  // const r4 = await burn(wallet, "Test NFT");
  // console.log("Burn Result:", r4);

  // const r5 = await getInfo("Test NFT");

  // console.log("User:", r5.user.toBase58());
  // console.log("Burn Counts:", r5.burnCount);
  // console.log("Merit Value:", r5.meritValue.toNumber());
  // console.log("Incense Value:", r5.incenseValue.toNumber());
  // console.log("Last Update Time:", new Date(r5.lastUpdateTime.toNumber() * 1000));
  // console.log("Is Reset:", r5.isReset);

  const r6 = await destroy(wallet, "Test NFT");
  console.log("Destroy Result:", r6);
})()