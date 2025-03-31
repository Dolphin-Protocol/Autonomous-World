import "./style.css";
import { getWallets } from "@mysten/wallet-standard";

const availableWallets = getWallets().get();

console.log(availableWallets);
