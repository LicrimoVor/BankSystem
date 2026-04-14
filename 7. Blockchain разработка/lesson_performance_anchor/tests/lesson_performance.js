const anchor = require("@coral-xyz/anchor");
const { expect } = require("chai");
const {
  getAccount,
  getMint,
  getAssociatedTokenAddressSync,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} = require("@solana/spl-token");

describe("lesson_performance", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.LessonPerformance;

  const treasury = anchor.web3.Keypair.generate();
  const [oraclePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("oracle")],
    program.programId
  );

  async function airdrop(pubkey, lamports) {
    const signature = await provider.connection.requestAirdrop(pubkey, lamports);
    await provider.connection.confirmTransaction(signature);
  }

  before("initialize oracle + treasury", async () => {
    await airdrop(treasury.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL);
    await program.methods
      .initializeOracle(new anchor.BN(25_000_000))
      .accounts({
        oracle: oraclePda,
        admin: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
  });

  it("baseline path: pays fee and mints", async () => {
    const mint = anchor.web3.Keypair.generate();
    const [mintAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("mint_authority"), mint.publicKey.toBuffer()],
      program.programId
    );
    const payerAta = getAssociatedTokenAddressSync(mint.publicKey, provider.wallet.publicKey);

    const treasuryBefore = await provider.connection.getBalance(treasury.publicKey);

    await program.methods
      .createTokenWithFeeBaseline(new anchor.BN(100), new anchor.BN(25_000_000))
      .accounts({
        mint: mint.publicKey,
        payerAta,
        mintAuthority,
        payer: provider.wallet.publicKey,
        treasury: treasury.publicKey,
        oracle: oraclePda,
        oracleConfig: anchor.web3.SystemProgram.programId,
        metadataProgram: anchor.web3.SystemProgram.programId,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([mint])
      .rpc();

    const treasuryAfter = await provider.connection.getBalance(treasury.publicKey);
    expect(treasuryAfter - treasuryBefore).to.equal(anchor.web3.LAMPORTS_PER_SOL);

    const mintInfo = await getMint(provider.connection, mint.publicKey);
    const ataInfo = await getAccount(provider.connection, payerAta);
    const expectedRaw = BigInt(100_000_000);
    expect(mintInfo.supply).to.equal(expectedRaw);
    expect(ataInfo.amount).to.equal(expectedRaw);
  });

  it("optimized path: same business logic with smaller context", async () => {
    await program.methods
      .updatePrice(new anchor.BN(50_000_000))
      .accounts({
        oracle: oraclePda,
        admin: provider.wallet.publicKey,
      })
      .rpc();

    const mint = anchor.web3.Keypair.generate();
    const [mintAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("mint_authority"), mint.publicKey.toBuffer()],
      program.programId
    );
    const payerAta = getAssociatedTokenAddressSync(mint.publicKey, provider.wallet.publicKey);

    const treasuryBefore = await provider.connection.getBalance(treasury.publicKey);

    await program.methods
      .createTokenWithFeeOptimized(new anchor.BN(200), new anchor.BN(25_000_000))
      .accounts({
        mint: mint.publicKey,
        payerAta,
        mintAuthority,
        payer: provider.wallet.publicKey,
        treasury: treasury.publicKey,
        oracle: oraclePda,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([mint])
      .rpc();

    const treasuryAfter = await provider.connection.getBalance(treasury.publicKey);
    expect(treasuryAfter - treasuryBefore).to.equal(anchor.web3.LAMPORTS_PER_SOL / 2);

    const mintInfo = await getMint(provider.connection, mint.publicKey);
    const ataInfo = await getAccount(provider.connection, payerAta);
    const expectedRaw = BigInt(200_000_000);
    expect(mintInfo.supply).to.equal(expectedRaw);
    expect(ataInfo.amount).to.equal(expectedRaw);
  });

  it("optimized path rejects stale oracle", async () => {
    await program.methods
      .setOracleLastUpdatedSlot(new anchor.BN(0))
      .accounts({
        oracle: oraclePda,
        admin: provider.wallet.publicKey,
      })
      .rpc();

    const mint = anchor.web3.Keypair.generate();
    const [mintAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("mint_authority"), mint.publicKey.toBuffer()],
      program.programId
    );
    const payerAta = getAssociatedTokenAddressSync(mint.publicKey, provider.wallet.publicKey);

    try {
      await program.methods
        .createTokenWithFeeOptimized(new anchor.BN(1), new anchor.BN(25_000_000))
        .accounts({
          mint: mint.publicKey,
          payerAta,
          mintAuthority,
          payer: provider.wallet.publicKey,
          treasury: treasury.publicKey,
          oracle: oraclePda,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        })
        .signers([mint])
        .rpc();
      expect.fail("expected stale oracle error");
    } catch (e) {
      expect(String(e.message || e)).to.match(/stale/i);
    }
  });
});
