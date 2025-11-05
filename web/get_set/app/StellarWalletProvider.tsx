'use client';
import { ReactNode } from "react";
import { NetworkDetails, SorobanReactProvider, WalletNetwork } from "stellar-react";
import deployments from './deployments.json';
const StellarWalletProvider = ({ children }: { children: ReactNode }) => {


    const testnetNetworkDetails: NetworkDetails = {
        network: WalletNetwork.TESTNET,
        sorobanRpcUrl: 'https://soroban-testnet.stellar.org/',
        horizonRpcUrl: 'https://horizon-testnet.stellar.org'
    }
    return (
        <SorobanReactProvider
            appName={"Hello World Stellar App"}
            allowedNetworkDetails={[testnetNetworkDetails]}
            activeNetwork={WalletNetwork.TESTNET}
            deployments={deployments}
        >
            {children}
        </SorobanReactProvider>
    )
}
export default StellarWalletProvider;