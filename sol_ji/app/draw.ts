import { incenseBurn, destroy, getInfo } from "./api/incense_burn";
import { initialize, updateIncense } from "./api/incense_config";
import { nftMint } from "./api/nft_mint";
import { getWallet } from "./api/wallet";
import { initializeLotteryPoetry, drawLots, initializeDrawLots } from "./api/draw_lots";
(async () => {
  const wallet = getWallet();

  const name = "Test NFT 01";

  // const r2 = await updateIncense();
  // console.log("Update Incense Result:", r2);

  // const r3 = await nftMint(name, "TNFT", "https://poor-gold-wildebeest.myfilebase.com/ipfs/QmPCWecKXa6darBrnsKuveDqyhYxFGcnJZzoo7fkFFn6oS");
  // console.log("NFT Mint Result:", r3);

  // const r4 = await incenseBurn(wallet, name);
  // console.log("Burn Result:", r4);

  // 初始化签文
  // const r7 = await initializeLotteryPoetry();
  // console.log("Initialize Lottery Poetry Result:", r7);

  const r8 = await initializeDrawLots();
  console.log("Initialize Draw Lots Result:", r8);

  // 签文
  const r9 = await drawLots(name, wallet);
  console.log("Draw Lots Result:", r9);


  const r5 = await getInfo(wallet);
  console.log("User:", r5);


})()