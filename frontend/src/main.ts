import "./style.css";
import { getFullnodeUrl, SuiClient } from "@mysten/sui/client";
import { walletStore, connectWallet, disConnectWallet } from "./wallet";
import {
  get_sui_address,
  update_sui_address,
} from "../wasm/autonomous-game.js";

declare global {
  interface Window {
    wasmInitialized: Promise<void>;
  }
}

// Wait for WASM initialization before running main code
async function init() {
  await window.wasmInitialized;

  // Your main.ts initialization code goes here
  console.log({ foo: get_sui_address() });
}

init().catch(console.error);

// // Create and append connect button to the DOM
// const connectButton = document.createElement("button");
// connectButton.innerHTML = "Connect Wallet";
// connectButton.className = "login-button";
// document.body.appendChild(connectButton);
//
// // Create and append sign button to the DOM
// const signButton = document.createElement("button");
// signButton.innerHTML = "Sign the Tx";
// signButton.className = "login-button";
// signButton.style.display = "none"; // Hide initially
// document.body.appendChild(signButton);

// Update UI based on initial wallet state
// const updateButtonsState = (isConnected: boolean, accounts: any[] | null) => {
//   if (isConnected && accounts?.length) {
//     connectButton.innerHTML = `Connected: ${accounts[0].address.slice(0, 6)}...${accounts[0].address.slice(-4)}`;
//     connectButton.classList.add("connected");
//     signButton.style.display = "block";
//   } else {
//     connectButton.innerHTML = "Connect Wallet";
//     connectButton.classList.remove("connected");
//     signButton.style.display = "none";
//   }
// };
//
// // Initialize buttons state
// updateButtonsState(
//   walletStore.getState().isConnected,
//   walletStore.getState().accounts,
// );
//
// console.log({ foo: get_sui_address() });

const client = new SuiClient({ url: getFullnodeUrl("testnet") });

// Subscribe to wallet state changes
// walletStore.subscribe((state) => {
//   updateButtonsState(state.isConnected, state.accounts);
// });

// connectButton.addEventListener("click", () =>
//   walletStore.getState().isConnected ? disConnectWallet() : connectWallet(),
// );
