import { createProfile, getProfile } from "./api/profile";
import { useDefauleWallet, useVisitorWallet } from "./api/wallet";

(async () => {
  const defaultWallet = useDefauleWallet();
  const visitorWallet = useVisitorWallet();
  const r1 = await createProfile(visitorWallet, "Alice");
  console.log(r1);

  const r2 = await getProfile(visitorWallet);
  console.log(r2);
})()