import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LaunchpadFoundations } from "../target/types/launchpad_foundations";
import { Keypair } from "@solana/web3.js";
import { expect } from "chai";

describe("launchpad_foundations", () => {
	const provider = anchor.AnchorProvider.env();
	anchor.setProvider(provider);

	const program = anchor.workspace
		.launchpadFoundations as Program<LaunchpadFoundations>;
	const configKeypair = Keypair.generate();

	it("Is initialized!", async () => {
		const tx = await program.methods
			.initialize(new anchor.BN(25_000_000))
			.accounts({
				signer: provider.wallet.publicKey,
				config: configKeypair.publicKey,
			})
			.signers([configKeypair])
			.rpc();
		console.log("initialized signature", tx);

		const config = await program.account.launchpadConfig.fetch(
			configKeypair.publicKey,
		);
		expect(config.feeUsd.toNumber()).to.equal(25_000_000);
		expect(config.admin.toBase58()).to.equal(
			provider.wallet.publicKey.toBase58(),
		);
	});

	it("Is updated!", async () => {
		const tx = await program.methods
			.update(new anchor.BN(30_000_000))
			.accounts({
				config: configKeypair.publicKey,
			})
			.rpc();
		console.log("updated signature", tx);
		const config = await program.account.launchpadConfig.fetch(
			configKeypair.publicKey,
		);
		expect(config.feeUsd.toNumber()).to.equal(30_000_000);
	});

	it("second initialize fails", async () => {
		try {
			await program.methods
				.initialize(new anchor.BN(0))
				.accounts({
					config: configKeypair.publicKey,
					signer: provider.wallet.publicKey,
				})
				.signers([configKeypair])
				.rpc();
			expect.fail("expected error");
		} catch (e) {
			expect(e).to.be.ok;
		}
	});
});
