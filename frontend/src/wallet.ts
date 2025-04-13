import { SuiClient } from "@mysten/sui/client";
import { Transaction } from "@mysten/sui/transactions";
import {
  getWallets,
  signTransaction,
  SuiWalletFeatures,
  WalletAccount,
  WalletWithFeatures,
} from "@mysten/wallet-standard";

// Define a type for our wallet store
export type WalletStore = {
  wallet: WalletWithFeatures<Partial<SuiWalletFeatures>> | null;
  accounts: WalletAccount[] | null;
  isConnected: boolean;
};

// Create a singleton store instance
class WalletStateStore {
  private static instance: WalletStateStore;
  private state: WalletStore = {
    wallet: null,
    accounts: null,
    isConnected: false,
  };
  private listeners: ((state: WalletStore) => void)[] = [];

  private constructor() {
    // Initialize wallet
    const availableWallets = getWallets().get();
    this.state.wallet =
      availableWallets.find((w) => w.name === "Sui Wallet") || null;
  }

  public static getInstance(): WalletStateStore {
    if (!WalletStateStore.instance) {
      WalletStateStore.instance = new WalletStateStore();
    }
    return WalletStateStore.instance;
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

export async function connectWallet() {
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
  return connectedSuiAccounts;
}

export async function disConnectWallet() {
  const { wallet } = walletStore.getState();
  if (!wallet) throw Error("No wallet available");

  if (!wallet.features["standard:disconnect"])
    throw Error(
      `Provided wallet (${wallet.name}) does not support the disconnect feature.`,
    );

  await wallet.features["standard:disconnect"].disconnect();

  walletStore.setAccounts(null);
}

export async function signAndExecuteTransaction(
  client: SuiClient,

  tx: Transaction,
) {
  const { wallet, accounts } = walletStore.getState();
  if (!wallet || !accounts) throw Error("No Connected wallet account");

  tx.setGasBudget(50000);

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

  console.log({ result });
}
