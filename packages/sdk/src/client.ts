import { Connection, PublicKey, TransactionInstruction, ComputeBudgetProgram } from "@solana/web3.js";
import { Program, AnchorProvider, BN } from "@coral-xyz/anchor";
import { getProtocolConfigPda, getExtraAccountMetasPda, PROGRAM_ID } from "./pda";

export interface PriorityFeeConfig {
  microLamports?: number;
  computeUnits?: number;
}

export class TokenSuiteClient {
  public readonly programId: PublicKey;

  constructor(
    public readonly connection: Connection,
    public readonly provider: AnchorProvider,
    programId?: PublicKey
  ) {
    this.programId = programId ?? PROGRAM_ID;
  }

  /**
   * Helper to create priority fee instructions to ensure transaction inclusion
   */
  public createPriorityFeeInstructions(config: PriorityFeeConfig = {}): TransactionInstruction[] {
    const instructions: TransactionInstruction[] = [];
    const microLamports = config.microLamports ?? 50_000;
    const computeUnits = config.computeUnits ?? 250_000;

    instructions.push(
      ComputeBudgetProgram.setComputeUnitPrice({ microLamports }),
      ComputeBudgetProgram.setComputeUnitLimit({ units: computeUnits })
    );

    return instructions;
  }

  /**
   * Fetches the current protocol configuration
   */
  public async getProtocolConfig(mint: PublicKey) {
    const [configPda] = getProtocolConfigPda(mint, this.programId);
    const accountInfo = await this.connection.getAccountInfo(configPda);
    if (!accountInfo) {
      throw new Error(`Protocol config not found for mint ${mint.toBase58()}`);
    }
    return {
      address: configPda,
      data: accountInfo.data,
    };
  }
}
