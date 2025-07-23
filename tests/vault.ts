import { AnchorProvider, Program, Wallet, web3, BN } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";
import { LiteSVM, TransactionMetadata } from "../local-litesvm";
import IDL from "../target/idl/vault.json";
import { assert } from "chai";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";

describe("vault", () => {
  let svm: LiteSVM;
  let payer: Keypair;
  let programId: PublicKey;
  let program: Program<Vault>;

  before(async () => {
    svm = new LiteSVM();
    payer = new Keypair();
    svm.airdrop(payer.publicKey, BigInt(LAMPORTS_PER_SOL));
    const provider = new AnchorProvider(svm as any, new Wallet(payer), {
      commitment: "confirmed",
    });
    program = new Program(IDL, provider);
    programId = program.programId;

    svm.addProgramFromFile(programId, "target/deploy/vault.so");

    // Add coverage generation
    svm.withCoverage([["vault", programId.toBuffer()]], [], payer.secretKey);
  });

  it("test_init_vault", async () => {
    const [vaultPDA] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), payer.publicKey.toBuffer()],
      programId
    );
    const initInstruction = await program.methods
      .initVault()
      .accounts({
        owner: payer.publicKey,
        //@ts-ignore
        vault: vaultPDA,
        systemProgram: web3.SystemProgram.programId,
      })
      .instruction();

    const blockhash = svm.latestBlockhash();
    const tx = new web3.Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(initInstruction);
    tx.sign(payer);

    const result = svm.sendTransaction(tx);

    assert.instanceOf(result, TransactionMetadata);

    const vaultBalance = svm.getBalance(vaultPDA);
    const rentExemptBalance = svm.getRent().minimumBalance(BigInt(0));

    assert.isTrue(
      vaultBalance === rentExemptBalance,
      "Vault should be rent-exempt"
    );
  });

  it("test_deposit", async () => {
    const [vaultPDA] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), payer.publicKey.toBuffer()],
      program.programId
    );
    const depositAmount = BigInt(0.5 * web3.LAMPORTS_PER_SOL);

    const depositInstruction = await program.methods
      .deposit(new BN(depositAmount))
      .accounts({
        owner: payer.publicKey,
        //@ts-ignore
        vault: vaultPDA,
        systemProgram: web3.SystemProgram.programId,
      })
      .instruction();

    const blockhash = svm.latestBlockhash();

    const tx = new web3.Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(depositInstruction);
    tx.sign(payer);

    const initialPayerBalance = svm.getBalance(payer.publicKey);
    const initialVaultBalance = svm.getBalance(vaultPDA) ?? BigInt(0);

    const result = svm.sendTransaction(tx);

    assert.instanceOf(result, TransactionMetadata);

    const finalPayerBalance = BigInt(svm.getBalance(payer.publicKey));
    const finalVaultBalance = BigInt(svm.getBalance(vaultPDA));

    assert.equal(
      finalVaultBalance - initialVaultBalance,
      depositAmount,
      "Vault balance should increase by deposit amount"
    );

    assert.isTrue(
      initialPayerBalance - finalPayerBalance >= depositAmount,
      "Payer balance should decrease by at least deposit amount (including tx fee)"
    );
  });

  it("test_withdraw", async () => {
    const [vaultPDA] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), payer.publicKey.toBuffer()],
      program.programId
    );

    const withdrawAmount = new BN(0.25 * web3.LAMPORTS_PER_SOL);

    const initialPayerBalance = BigInt(svm.getBalance(payer.publicKey));
    const initialVaultBalance = BigInt(svm.getBalance(vaultPDA));

    const withdrawInstruction = await program.methods
      .withdraw(withdrawAmount)
      .accounts({
        owner: payer.publicKey,
        //@ts-ignore
        vault: vaultPDA,
        systemProgram: web3.SystemProgram.programId,
      })
      .instruction();

    const tx = new web3.Transaction();
    tx.recentBlockhash = svm.latestBlockhash();
    tx.add(withdrawInstruction);
    tx.sign(payer);

    const result = svm.sendTransaction(tx);

    assert.instanceOf(result, TransactionMetadata);

    const finalPayerBalance = BigInt(svm.getBalance(payer.publicKey));
    const finalVaultBalance = BigInt(svm.getBalance(vaultPDA));

    assert.equal(
      initialVaultBalance - finalVaultBalance,
      BigInt(withdrawAmount.toString()),
      "Vault balance should decrease by withdraw amount"
    );

    const balanceIncrease = finalPayerBalance - initialPayerBalance;
    assert.isTrue(
      balanceIncrease > BigInt(0) &&
        balanceIncrease <= BigInt(withdrawAmount.toString()),
      "Payer should receive withdrawn amount minus tx fee"
    );

    const rentExemptBalance = svm.getRent().minimumBalance(BigInt(0));
    assert.isTrue(
      finalVaultBalance >= rentExemptBalance,
      "Vault should maintain rent-exempt balance"
    );
  });
});
