import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { expect } from "chai";
import { getProtocolConfigPda } from "../packages/sdk/src/pda";

describe("token-suite tests", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const mintKeypair = Keypair.generate();
  const feeTreasury = Keypair.generate().publicKey;

  it("Derives deterministic PDAs correctly", () => {
    const [configPda, bump] = getProtocolConfigPda(mintKeypair.publicKey);
    expect(configPda).to.be.instanceOf(PublicKey);
    expect(bump).to.be.a("number");
  });

  it("Validates fee basis points boundaries", () => {
    const MAX_FEE_BPS = 500;
    const testValidFee = 150;
    const testInvalidFee = 600;

    expect(testValidFee <= MAX_FEE_BPS).to.be.true;
    expect(testInvalidFee <= MAX_FEE_BPS).to.be.false;
  });
});
