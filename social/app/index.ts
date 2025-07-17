import { createProfile, getProfile } from "./api/profile";
import { createTweet, getTweet, createLike } from "./api/tweet";
import { useDefauleWallet, useVisitorWallet } from "./api/wallet";
import { createTokenMintAccount } from "./api/token";

(async () => {
  const defaultWallet = useDefauleWallet();
  const visitorWallet = useVisitorWallet();
  // const r1 = await createProfile(visitorWallet, "Alice");
  // console.log(r1);

  // const r2 = await getProfile(defaultWallet);
  // console.log(r2);

  const [pda, r3] = await createTweet(defaultWallet, "hello solana just");
  console.log(r3);

  const r4 = await getTweet(pda);
  console.log(r4);

  const r5 = await createLike(visitorWallet, pda);
  console.log(r5);

  const r6 = await getTweet(pda);
  console.log(r6);

  // const [pda, r] = await createTokenMintAccount(defaultWallet);
  // console.log(pda.toString(), r);

})()