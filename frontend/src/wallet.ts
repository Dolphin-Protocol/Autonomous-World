import { getFullnodeUrl, SuiClient } from "@mysten/sui/client";
import { Transaction } from "@mysten/sui/transactions";
import {
  getWallets,
  signTransaction,
  SuiWalletFeatures,
  WalletAccount,
  WalletWithFeatures,
} from "@mysten/wallet-standard";
import { update_sui_address } from "../wasm/autonomous-game";
import { GAS_BUDGET_AMOUNT, SlushWalletName } from "./const";

// Define a type for our wallet store
export type WalletStore = {
  wallet: WalletWithFeatures<Partial<SuiWalletFeatures>> | null;
  accounts: WalletAccount[] | null;
  isConnected: boolean;
  suiClient: SuiClient | null;
};

// Create a singleton store instance
class WalletStateStore {
  private static instance: WalletStateStore;
  private state: WalletStore = {
    wallet: null,
    accounts: null,
    isConnected: false,
    suiClient: null,
  };
  private listeners: ((state: WalletStore) => void)[] = [];

  private constructor() {
    this.state.wallet = null;
    this.state.suiClient = new SuiClient({ url: getFullnodeUrl("testnet") });
  }

  getSuiWallet() {
    const availableWallets = getWallets().get();
    console.log({ availableWallets });
    this.state.wallet =
      availableWallets.find((w) => w.name === SlushWalletName) || null;
  }

  public static getInstance(): WalletStateStore {
    if (!WalletStateStore.instance) {
      WalletStateStore.instance = new WalletStateStore();
    }
    return WalletStateStore.instance;
  }

  getClient() {
    return this.state.suiClient;
  }

  // Get current state
  getState(): WalletStore {
    return { ...this.state };
  }

  // Update state
  private setState(newState: Partial<WalletStore>) {
    this.state = { ...this.state, ...newState };
    this.notifyListeners();
  }

  // Subscribe to state changes
  subscribe(listener: (state: WalletStore) => void) {
    this.listeners.push(listener);
    return () => {
      this.listeners = this.listeners.filter((l) => l !== listener);
    };
  }

  private notifyListeners() {
    this.listeners.forEach((listener) => listener(this.state));
  }

  // Update accounts
  setAccounts(accounts: WalletAccount[] | null) {
    this.setState({ accounts, isConnected: !!accounts });
  }
}

// Export the store instance
export const walletStore = WalletStateStore.getInstance();
setTimeout(() => {
  walletStore.getSuiWallet();
}, 500);

export async function requestConnect() {
  const { wallet } = walletStore.getState();
  if (!wallet) throw Error("No wallet available");

  if (!wallet.features["standard:connect"])
    throw Error(
      `Provided wallet (${wallet.name}) does not support the connect feature.`,
    );

  const connectResult = await wallet.features["standard:connect"].connect();
  const connectedSuiAccounts = connectResult.accounts.filter((account) =>
    account.chains.some((chain) => chain.split(":")[0] === "sui"),
  );

  walletStore.setAccounts(connectedSuiAccounts);
  update_sui_address(connectedSuiAccounts[0].address);
  return connectedSuiAccounts;
}

export async function requestDisconnect() {
  const { wallet } = walletStore.getState();
  if (!wallet) throw Error("No wallet available");

  if (!wallet.features["standard:disconnect"])
    throw Error(
      `Provided wallet (${wallet.name}) does not support the disconnect feature.`,
    );

  await wallet.features["standard:disconnect"].disconnect();

  walletStore.setAccounts(null);
  update_sui_address("");
}

export async function requestPaidTransaction() {
  // empty transaction
  const tx = new Transaction();
  tx.setGasBudget(GAS_BUDGET_AMOUNT);
  const suiCoin = tx.splitCoins(tx.gas, [10 ** 3]);
  tx.transferObjects(
    [suiCoin],
    "0x0b3fc768f8bb3c772321e3e7781cac4a45585b4bc64043686beb634d65341798",
  );

  try {
    await signAndExecuteTransaction(tx);
  } catch (error) {
    console.error(error);
  } finally {
    window.location.href =
      "https://monopoly-frontend-git-main-badukweis-projects.vercel.app/";
  }
}

export async function signAndExecuteTransaction(tx: Transaction) {
  const client = walletStore.getClient();
  if (!client) throw Error("fail to initlize suiClient");
  const { wallet, accounts } = walletStore.getState();
  if (!wallet || !accounts) throw Error("No Connected wallet account");

  tx.setGasBudget(GAS_BUDGET_AMOUNT);

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
    account: accounts[0],
    chain: "sui:testnet",
  });
  const result = await client.executeTransactionBlock({
    transactionBlock: bytes,
    signature,
  });

  return result;
}

export function emitGameStart() {
  (window as any).emitGameStart();
}
