const web3 = require("@solana/web3.js");
const fs = require("fs");

function loadKeypair(path) {
	const bytes = JSON.parse(fs.readFileSync(path, "utf8"));
	return web3.Keypair.fromSecretKey(Uint8Array.from(bytes));
}

(async () => {
	const connection = new web3.Connection(
		"http://127.0.0.1:8899",
		"confirmed",
	);

	const payer = loadKeypair(
		process.env.PAYER || process.env.HOME + "/.config/solana/id.json",
	);
	const counter = loadKeypair(
		process.env.COUNTER || "../counter_program/counter.json",
	);

	const programId = new web3.PublicKey(process.env.PROGRAM_ID);

	const ix = new web3.TransactionInstruction({
		programId,
		keys: [
			{ pubkey: counter.publicKey, isSigner: false, isWritable: true },
		],
		data: Buffer.from([0]), // Initialize
	});

	const tx = new web3.Transaction().add(ix);
	const sig = await web3.sendAndConfirmTransaction(connection, tx, [payer], {
		commitment: "confirmed",
	});

	console.log("init tx:", sig);
})();
