import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import {
  Connection, PublicKey, SYSVAR_CLOCK_PUBKEY, Transaction
} from "@solana/web3.js";
import {
  AcceptOfferInstruction, getKeypair,
  getProgramId,
  getPublicKey,
  getTerms
} from "./utils";
import BN = require("bn.js");

const accept_offer = async () => {
  const lendingProgramId = getProgramId();
  const terms = getTerms();

  const borrower_usd_account_pubkey = getPublicKey("alice_y");
  const usd_mint_pubkey = getPublicKey("mint_y");
  const borrower_account = getKeypair("alice");

  const loan_id = getPublicKey("loan");
  const offer_id = getPublicKey("offer");
  const pda_token_account = getPublicKey("pda_token");
  const lender_account = getKeypair("bob");

  const connection = new Connection("https://api.devnet.solana.com", 'singleGossip');

  //accept loan
  const PDA = await PublicKey.findProgramAddress([Buffer.from("lending")], lendingProgramId);
  const acceptOfferTx = AcceptOfferInstruction(
    lendingProgramId,
    borrower_account.publicKey,
    lender_account.publicKey,
    loan_id,
    borrower_usd_account_pubkey,
    offer_id,
    pda_token_account,
    TOKEN_PROGRAM_ID,
    PDA[0],
    SYSVAR_CLOCK_PUBKEY,
    terms.loanPrincipalAmount,
    terms.loanDuration,
    terms.interestRate,
    usd_mint_pubkey,
  )
  const tx1 = new Transaction().add(
    acceptOfferTx,
  );
  console.log("Sending Alice's accept offer...", acceptOfferTx);
  await connection.sendTransaction(
    tx1,
    [borrower_account],
    { skipPreflight: false, preflightCommitment: "confirmed" }
  );

  //
  console.log("Done");
};

accept_offer();
