import "./index.css";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import App from "./App.tsx";
import AuthGuard from "./AuthGuard.tsx";
import React from "react";
import ReactDOM from "react-dom/client";
import { SiweIdentityProvider } from "ic-siwe-js/react";
import { Toaster } from "react-hot-toast";
import { WagmiProvider } from "wagmi";
import { wagmiConfig } from "./wagmi/wagmi.config.ts";
import { canisterId as siweProviderCanisterId } from "../../ic_siwe_provider/declarations/index";
import {
  createActorHook,
} from "ic-use-actor";
import { canisterId as backendCanisterId, idlFactory } from "../../backend/declarations/index";
import { _SERVICE } from "../../backend/declarations/backend.did";

export const useBackend = createActorHook<_SERVICE>({
  canisterId: backendCanisterId,
  idlFactory,
});

const queryClient = new QueryClient();

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <WagmiProvider config={wagmiConfig}>
      <QueryClientProvider client={queryClient}>
        <SiweIdentityProvider canisterId={siweProviderCanisterId}>
          <AuthGuard>
            <App />
          </AuthGuard>
        </SiweIdentityProvider>
      </QueryClientProvider>
    </WagmiProvider>
    <Toaster />
  </React.StrictMode>,
);
