import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import {
  Connection, PublicKey, SYSVAR_CLOCK_PUBKEY, Transaction
} from "@solana/web3.js";
import {
  getKeypair,
  getProgramId,
  getPublicKey,
  getTerms, PayInstruction
} from "./utils";
import BN = require("bn.js");

const pay = async () => {
  const lendingProgramId = getProgramId();

  const borrower_account = getKeypair("alice");
  const nft_mint_pubkey = getPublicKey("mint_x");

  const borrower_nft_account_pubkey = (
    await PublicKey.findProgramAddress(
      [borrower_account.publicKey.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), nft_mint_pubkey.toBuffer()],
      ASSOCIATED_TOKEN_PROGRAM_ID,
    )
  )[0];
  const borrower_usd_account_pubkey = getPublicKey("alice_y");

  const loan_id = getPublicKey("loan");
  const offer_id = getPublicKey("offer");
  const pda_token_account = getPublicKey("pda_token");
  const pda_nft_account = getPublicKey("pda_nft");
  const admin_token_pubkey = getPublicKey("admin_y");
  const lender_account = getKeypair("bob");

  const connection = new Connection("https://api.devnet.solana.com", 'singleGossip');

  //pay loan
  const PDA = await PublicKey.findProgramAddress([Buffer.from("lending")], lendingProgramId);
  const payTx = PayInstruction(
    lendingProgramId,
    borrower_account.publicKey,
    lender_account.publicKey,
    loan_id,
    offer_id,
    borrower_nft_account_pubkey,
    borrower_usd_account_pubkey,
    pda_nft_account,
    pda_token_account,
    admin_token_pubkey,
    TOKEN_PROGRAM_ID,
    PDA[0],
    SYSVAR_CLOCK_PUBKEY,
    BigInt(2000000000 + 8493150),
  )
  const tx1 = new Transaction().add(
    payTx,
  );
  console.log("Sending pay tx...", payTx);
  await connection.sendTransaction(
    tx1,
    [borrower_account],
    { skipPreflight: false, preflightCommitment: "confirmed" }
  );

  //
  console.log("done");
};

pay();
