import { EnokiClient, EnokiFlow } from "@mysten/enoki";
import "./style.css";
import { getWallets, Wallet, signTransaction } from "@mysten/wallet-standard";
import { getFullnodeUrl, SuiClient } from "@mysten/sui/client";
import { Transaction } from "@mysten/sui/transactions";

// Create and append login button to the DOM
const loginButton = document.createElement("button");
loginButton.innerHTML = "Login with Google";
loginButton.className = "login-button";
document.body.appendChild(loginButton);

// Create and append login button to the DOM
const signButton = document.createElement("button");
signButton.innerHTML = "Sign the Tx";
signButton.className = "login-button";
document.body.appendChild(signButton);

const apiKey = import.meta.env.VITE_ENOKI_PRIVATE_KEY;
const client = new SuiClient({ url: getFullnodeUrl("testnet") });

const availableWallets = getWallets().get();
const suiWallet = availableWallets.find(
  (wallet) => wallet.name == "Sui Wallet",
);

// state
let accounts = null;
if (suiWallet) {
  console.log(suiWallet.accounts);
  loginButton.addEventListener("click", () => connectWallet(suiWallet));
  signButton.addEventListener("click", () => signSuiTransaction(suiWallet));
}

async function connectWallet(wallet: Wallet) {
  const connectResult = await wallet.features["standard:connect"].connect();

  if (!!connectResult.accounts.length) accounts = connectResult.accounts;
}

async function signAndExecuteTransaction(wallet: Wallet) {
  const tx = new Transaction();

  tx.setSender(accounts[0].address);
  // tx.setGasBudget(50000);

  // const txJson = await tx.toJSON({ supportedIntents: [], client });
  const { bytes, signature } = await signTransaction(wallet, {
    transaction: {
      async toJSON() {
        return await tx.toJSON({
          supportedIntents: [],
          client,
        });
      },
    },
    account: accounts[0] as any,
    chain: "sui:testnet",
  });
  const result = await client.executeTransactionBlock({
    transactionBlock: bytes,
    signature,
  });

  console.log({ result });
}
