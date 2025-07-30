import { burn } from "./api/burn";
import { initialize, updateIncense } from "./api/incense_config";
import { nftMint } from "./api/nft_mint";
import { getWallet } from "./api/wallet";
(async () => {
  const wallet = getWallet();
  // const r1 = await initialize();
  // console.log("Initialization Result:", r1);

  // const r2 = await updateIncense();
  // console.log("Update Incense Result:", r2);

  // const r3 = await burn(wallet);
  // console.log("User:", r3.user.toBase58());
  // console.log("Burn Counts:", r3.burnCount);
  // console.log("Merit Value:", r3.meritValue.toNumber());
  // console.log("Incense Value:", r3.incenseValue.toNumber());
  // console.log("Last Update Time:", new Date(r3.lastUpdateTime.toNumber() * 1000));
  // console.log("Is Reset:", r3.isReset);

  const r4 = await nftMint("Test NFT", "TNFT", "https://poor-gold-wildebeest.myfilebase.com/ipfs/QmPCWecKXa6darBrnsKuveDqyhYxFGcnJZzoo7fkFFn6oS");
  console.log("NFT Mint Result:", r4);
})()