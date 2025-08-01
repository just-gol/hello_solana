import { incenseBurn, destroy, getInfo } from "./api/incense_burn";
import { initialize, updateIncense } from "./api/incense_config";
import { nftMint } from "./api/nft_mint";
import { getWallet } from "./api/wallet";
import { initializeLotteryPoetry, drawLots } from "./api/draw_lots";
import { getLotteryCountPda } from "./api/address";
(async () => {
  const wallet = getWallet();
  // const r1 = await initialize();
  // console.log("Initialization Result:", r1);

  // const r2 = await updateIncense();
  // console.log("Update Incense Result:", r2);

  const name = "Test NFT 11";
  // const r3 = await nftMint(name, "TNFT", "https://poor-gold-wildebeest.myfilebase.com/ipfs/QmPCWecKXa6darBrnsKuveDqyhYxFGcnJZzoo7fkFFn6oS");
  // console.log("NFT Mint Result:", r3);

  // const r4 = await incenseBurn(wallet, name);
  // console.log("Burn Result:", r4);

  // const r5 = await getInfo(wallet);
  // console.log("User:", r5.user.toBase58());
  // console.log("Burn Counts:", r5.burnCount);
  // console.log("Merit Value:", r5.meritValue.toNumber());
  // console.log("Incense Value:", r5.incenseValue.toNumber());
  // console.log("Last Update Time:", new Date(r5.lastUpdateTime.toNumber() * 1000));
  // console.log("Is Reset:", r5.isReset);

  // 销毁
  // const r6 = await destroy(wallet, "Test NFT 11");
  // console.log("Destroy Result:", r6);

  // 签文
  // const r7 = await initializeLotteryPoetry();
  // console.log("Initialize Lottery Poetry Result:", r7);

  const r8 = await drawLots(name, 4, wallet);
  console.log("Draw Lots Result:", r8);


  // const r11 = await getInfo(wallet);
  // console.log("User:", r11.user.toBase58());
  // console.log("Burn Counts:", r11.burnCount);
  // console.log("Merit Value:", r11.meritValue.toNumber());
  // console.log("Incense Value:", r11.incenseValue.toNumber());
  // console.log("Last Update Time:", new Date(r11.lastUpdateTime.toNumber() * 1000));
  // console.log("Is Reset:", r11.isReset);

})()