import * as anchor from "@coral-xyz/anchor";
import {
	createNft,
	findMasterEditionPda,
	findMetadataPda,
	mplTokenMetadata,
	verifySizedCollectionItem,
} from "@metaplex-foundation/mpl-token-metadata";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
	KeypairSigner,
	PublicKey,
	createSignerFromKeypair,
	generateSigner,
	keypairIdentity,
	percentAmount,
} from "@metaplex-foundation/umi";
import { Program } from "@coral-xyz/anchor";
import { NftStaking } from "../target/types/nft_staking";
import {
	ASSOCIATED_TOKEN_PROGRAM_ID,
	TOKEN_PROGRAM_ID,
	getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { SendTransactionError } from "@solana/web3.js";

describe("nft-staking", () => {
	const provider = anchor.AnchorProvider.env();
	anchor.setProvider(provider);

	const program = anchor.workspace.NftStaking as Program<NftStaking>;

	const umi = createUmi(provider.connection);

	const payer = provider.wallet as NodeWallet;

	let nftMint: KeypairSigner;
	let collectionMint: KeypairSigner;

	let stakeAccount: anchor.web3.PublicKey;

	const creatorWallet = umi.eddsa.createKeypairFromSecretKey(
		new Uint8Array(payer.payer.secretKey)
	);
	const creator = createSignerFromKeypair(umi, creatorWallet);
	umi.use(keypairIdentity(creator));
	umi.use(mplTokenMetadata());

	const [config] = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from("config")],
		program.programId
	);

	const [rewardsMint] = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from("rewards"), config.toBuffer()],
		program.programId
	);

	const [userAccount] = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from("user"), provider.publicKey.toBuffer()],
		program.programId
	);

	[stakeAccount] = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from("stake"), provider.publicKey.toBuffer()],
		program.programId
	);

	// Creates an NFT collection, used to group NFTs
	it("Mint Collection NFT", async () => {
		try {
			const metadataUrl = "https://arweave.net/123";

			// Generates a private key to be the signer and creator of the collection
			collectionMint = generateSigner(umi);

			// Creates the NFT collection
			await createNft(umi, {
				mint: collectionMint, // Public key of the mint collection

				// Details of the collection
				name: "NFT Staking",
				symbol: "NFTS",
				uri: metadataUrl,

				sellerFeeBasisPoints: percentAmount(5.5), // Royalty fees for transactions
				creators: null,

				// Information about the collection
				collectionDetails: {
					__kind: "V1",
					size: 10,
				},
			}).sendAndConfirm(umi);

			console.log(
				`Created Collection NFT: ${collectionMint.publicKey.toString()}`
			);
		} catch (error) {
			if (error instanceof SendTransactionError) {
				console.log(await error.getLogs(provider.connection));
			} else {
				console.log(error);
			}
			throw error;
		}
	});

	it("Mint NFT", async () => {
		try {
			nftMint = generateSigner(umi);

			// Creates a single NFT in the collection
			await createNft(umi, {
				mint: nftMint,
				name: "NFT Staking",
				symbol: "NFTS",
				uri: "https://arweave.net/123",
				sellerFeeBasisPoints: percentAmount(5.5),

				// attributes the NFT to the collection
				collection: { verified: false, key: collectionMint.publicKey },

				creators: null,
			}).sendAndConfirm(umi);
			console.log(`\nCreated NFT: ${nftMint.publicKey.toString()}`);
		} catch (error) {
			if (error instanceof SendTransactionError) {
				console.log(await error.getLogs(provider.connection));
			} else {
				console.log(error);
			}
			throw error;
		}
	});

	it("Verify Collection NFT", async () => {
		try {
			// Find the PDAs for for the metadata and master edition
			const collectionMetadata = findMetadataPda(umi, {
				mint: collectionMint.publicKey,
			});
			const collectionMasterEdition = findMasterEditionPda(umi, {
				mint: collectionMint.publicKey,
			});

			const nftMetadata = findMetadataPda(umi, {
				mint: nftMint.publicKey,
			});

			// Verifies if the NFT is part of the given collection and is verified
			await verifySizedCollectionItem(umi, {
				metadata: nftMetadata,
				collectionAuthority: creator,
				collectionMint: collectionMint.publicKey,
				collection: collectionMetadata,
				collectionMasterEditionAccount: collectionMasterEdition,
			}).sendAndConfirm(umi);
			console.log("\nCollection NFT Verified!");
		} catch (error) {
			if (error instanceof SendTransactionError) {
				console.log(await error.getLogs(provider.connection));
			} else {
				console.log(error);
			}
			throw error;
		}
	});

	it("Initialize Config Account", async () => {
		try {
			const tx = await program.methods
				.initializeConfig(
					10, // points per stake
					10, // max stake
					0 // freeze period (days)
				)
				.accountsPartial({
					admin: provider.wallet.publicKey,
					config,
					rewardsMint,
					systemProgram: anchor.web3.SystemProgram.programId,
					tokenProgram: TOKEN_PROGRAM_ID,
				})
				.rpc();
			console.log("\nConfig Account Initialized!");
			console.log("Your transaction signature", tx);
		} catch (error) {
			if (error instanceof SendTransactionError) {
				console.log(await error.getLogs(provider.connection));
			} else {
				console.log(error);
			}
			throw error;
		}
	});

	it("Initialize User Account", async () => {
		try {
			const tx = await program.methods
				.initializeUser()
				.accountsPartial({
					user: provider.wallet.publicKey,
					userAccount,
					systemProgram: anchor.web3.SystemProgram.programId,
				})
				.rpc();
			console.log("\nUser Account Initialized!");
			console.log("Your transaction signature", tx);
		} catch (error) {
			if (error instanceof SendTransactionError) {
				console.log(await error.getLogs(provider.connection));
			} else {
				console.log(error);
			}
			throw error;
		}
	});

	it("Stake NFT", async () => {
		// account for the specific NFT mint, which holds the NFTs
		const mintAta = getAssociatedTokenAddressSync(
			new anchor.web3.PublicKey(nftMint.publicKey as PublicKey),
			provider.wallet.publicKey
		);

		// PDAs for the metadata and master edition of the NFT, used to verify the NFT in its collection
		const nftMetadata = findMetadataPda(umi, { mint: nftMint.publicKey });
		const nftEdition = findMasterEditionPda(umi, {
			mint: nftMint.publicKey,
		});

		// Find the PDAs for the stake account with the given NFT mint and config
		stakeAccount = anchor.web3.PublicKey.findProgramAddressSync(
			[
				Buffer.from("stake"),
				new anchor.web3.PublicKey(nftMint.publicKey).toBuffer(),
				config.toBuffer(),
			],
			program.programId
		)[0];

		const tx = await program.methods
			.stake()
			.accountsPartial({
				user: provider.wallet.publicKey,
				mint: nftMint.publicKey, // mint of the NFT
				collection: collectionMint.publicKey, // mint of the collection
				mintAta, // associated token account for the NFT mint
				metadata: new anchor.web3.PublicKey(nftMetadata[0]), // metadata PDA
				edition: new anchor.web3.PublicKey(nftEdition[0]), // master edition PDA
				config, // config account
				stakeAccount, // stake account
				userAccount,
			})
			.rpc();

		console.log("\nNFT Staked!");
		console.log("Your transaction signature", tx);
	});

	it("Unstake NFT", async () => {
		// account for the specific NFT mint, which holds the NFTs
		const mintAta = getAssociatedTokenAddressSync(
			new anchor.web3.PublicKey(nftMint.publicKey as PublicKey),
			provider.wallet.publicKey
		);

		// PDAs for the metadata and master edition of the NFT, used to verify the NFT in its collection
		const nftMetadata = findMetadataPda(umi, { mint: nftMint.publicKey });
		const nftEdition = findMasterEditionPda(umi, {
			mint: nftMint.publicKey,
		});

		// Find the PDAs for the stake account with the given NFT mint and config
		stakeAccount = anchor.web3.PublicKey.findProgramAddressSync(
			[
				Buffer.from("stake"),
				new anchor.web3.PublicKey(nftMint.publicKey).toBuffer(),
				config.toBuffer(),
			],
			program.programId
		)[0];

		const tx = await program.methods
			.unstake()
			.accountsPartial({
				user: provider.wallet.publicKey,
				mint: nftMint.publicKey, // mint of the NFT
				mintAta, // associated token account for the NFT mint
				metadata: new anchor.web3.PublicKey(nftMetadata[0]), // metadata PDA
				edition: new anchor.web3.PublicKey(nftEdition[0]), // master edition PDA
				config, // config account
				stakeAccount, // stake account
				userAccount,
			})
			.rpc();

		console.log("\nNFT unstaked!");
		console.log("Your transaction signature", tx);

		let account = await program.account.userAccount.fetch(userAccount);
		console.log("user points: ", account.points);
	});

	it("Claim Rewards", async () => {
		// ATA for the rewards mint, where the rewards will be sent to
		const rewardsAta = getAssociatedTokenAddressSync(
			rewardsMint,
			provider.wallet.publicKey
		);

		const tx = await program.methods
			.claim()
			.accountsPartial({
				user: provider.wallet.publicKey,
				userAccount, // user account
				rewardsMint, // rewards mint
				config, // config account
				rewardsAta, // associated token account for the rewards mint
				systemProgram: SYSTEM_PROGRAM_ID,
				tokenProgram: TOKEN_PROGRAM_ID,
				associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
			})
			.rpc();

		console.log("\nRewards claimed");
		console.log("Your transaction signature", tx);

		let account = await program.account.userAccount.fetch(userAccount);
		console.log("user points: ", account.points);
	});
});
