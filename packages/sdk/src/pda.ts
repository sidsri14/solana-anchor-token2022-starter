import { PublicKey } from "@solana/web3.js";

export const PROGRAM_ID = new PublicKey("Tokn221111111111111111111111111111111111111");

/**
 * Derives the ProtocolConfig PDA for a given mint
 */
export function getProtocolConfigPda(mint: PublicKey, programId: PublicKey = PROGRAM_ID): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("protocol-config"), mint.toBuffer()],
    programId
  );
}

/**
 * Derives the ExtraAccountMetas PDA required for Token-2022 Transfer Hook resolution
 */
export function getExtraAccountMetasPda(mint: PublicKey, programId: PublicKey = PROGRAM_ID): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("extra-account-metas"), mint.toBuffer()],
    programId
  );
}
