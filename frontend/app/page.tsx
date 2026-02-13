"use client";
import { useState } from "react";

export default function Home() {
  const [account, setAccount] = useState("");
  const [hash, setHash] = useState("");

  const registerAgent = async () => {
    alert(`Simulating register for ${account} with codehash ${hash}`);
    // NEAR CLI call / backend mock
  };

  return (
    <div style={{ padding: "2rem" }}>
      <h1>Sovereign Agent NEAR Demo</h1>
      <input placeholder="Account" value={account} onChange={e => setAccount(e.target.value)} />
      <input placeholder="Codehash" value={hash} onChange={e => setHash(e.target.value)} />
      <button onClick={registerAgent}>Register Agent</button>
    </div>
  );
}
