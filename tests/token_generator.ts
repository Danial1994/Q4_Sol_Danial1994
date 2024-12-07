import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Connection, SystemProgram, SYSVAR_RENT_PUBKEY, Keypair } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
import * as assert from "assert";

// Uncomment this import once the IDL has been generated
// import { TokenGenerator } from "../target/types/token_generator";

describe("token_generator", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TokenGenerator as Program<any>;

  it("Initializes a service token and airdrops tokens to doctor!", async () => {
    const provider = anchor.AnchorProvider.env();
    const serviceToken = Keypair.generate();
    const mint = Keypair.generate();

    // Use provider.wallet.payer directly, but cast to Keypair
    const payer = provider.wallet as anchor.Wallet & { payer: Keypair };
    const payerKeypair = payer.payer;

    // Create the doctor's associated token account
    const doctorTokenAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection as unknown as Connection,
      payerKeypair,
      mint.publicKey,
      provider.wallet.publicKey
    );

    await program.rpc.initializeServiceToken("General Checkup", new anchor.BN(100), {
      accounts: {
        serviceToken: serviceToken.publicKey,
        mint: mint.publicKey,
        doctorTokenAccount: doctorTokenAccount.address,
        provider: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [serviceToken, mint],
    });

    const account = await (program.account as any)["serviceToken"].fetch(serviceToken.publicKey);
    console.log('Service Token:', account);

    assert.ok(account.initialized);
    assert.equal(account.description, "General Checkup");
    assert.equal(account.cost.toNumber(), 100);
    assert.equal(account.mint.toBase58(), mint.publicKey.toBase58());
  });
});

