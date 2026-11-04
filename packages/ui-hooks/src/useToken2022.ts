import { useState, useCallback } from "react";
import { Connection, PublicKey } from "@solana/web3.js";
import { getProtocolConfigPda } from "@token-suite/sdk";

export interface UseToken2022State {
  loading: boolean;
  error: Error | null;
  configAddress: PublicKey | null;
}

export function useToken2022(connection: Connection, mintAddress?: string) {
  const [state, setState] = useState<UseToken2022State>({
    loading: false,
    error: null,
    configAddress: null,
  });

  const fetchConfig = useCallback(async () => {
    if (!mintAddress) return;
    try {
      setState((prev) => ({ ...prev, loading: true, error: null }));
      const mint = new PublicKey(mintAddress);
      const [pda] = getProtocolConfigPda(mint);
      setState({
        loading: false,
        error: null,
        configAddress: pda,
      });
    } catch (err: any) {
      setState({
        loading: false,
        error: err instanceof Error ? err : new Error(String(err)),
        configAddress: null,
      });
    }
  }, [mintAddress]);

  return {
    ...state,
    refetch: fetchConfig,
  };
}
