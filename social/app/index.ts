import { createProfile, getProfile } from "./api/profile";
import { createTweet, getTweet, createLike } from "./api/tweet";
import { useDefauleWallet, useVisitorWallet } from "./api/wallet";

(async () => {
  const defaultWallet = useDefauleWallet();
  const visitorWallet = useVisitorWallet();
  // const r1 = await createProfile(defaultWallet, "Bob");
  // console.log(r1);

  // const r2 = await getProfile(defaultWallet);
  // console.log(r2);

  const [pda, r3] = await createTweet(defaultWallet, "hello solana");
  console.log(r3);

  const r4 = await getTweet(pda);
  console.log(r4);

  const r5 = await createLike(defaultWallet, pda);
  console.log(r5);

  const r6 = await getTweet(pda);
  console.log(r6);

  const r7 = await createLike(defaultWallet, pda);
  console.log(r7);

})()