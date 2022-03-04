import { AccountLayout, Token, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";
import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  Transaction
} from "@solana/web3.js";
import {
  getKeypair,
  getProgramId,
  getPublicKey,
  getTerms,
  getTokenBalance, InitLoanInstruction, LoanInfoLayout,
  LOAN_INFO_LAYOUT, logError,
  writePublicKey, CancelLoanInstruction
} from "./utils";
import BN = require("bn.js");

const loan = async () => {
  const lendingProgramId = getProgramId();
  const terms = getTerms();
  const nft_mint_pubkey = getPublicKey("mint_x");
  const borrower_account = getKeypair("alice");
  const borrower_nft_account_pubkey = (
    await PublicKey.findProgramAddress(
      [borrower_account.publicKey.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), nft_mint_pubkey.toBuffer()],
      ASSOCIATED_TOKEN_PROGRAM_ID,
    )
  )[0];

  const loan_id = getPublicKey("loan");
  const pda_nft_account = getPublicKey("pda_nft");
  const connection = new Connection("https://api.devnet.solana.com", 'singleGossip');
  // //cancel loan
  const PDA = await PublicKey.findProgramAddress([Buffer.from("lending")], lendingProgramId);
  const cancelLoanTx = CancelLoanInstruction(
    lendingProgramId,
    borrower_account.publicKey,
    loan_id,
    pda_nft_account,
    borrower_nft_account_pubkey,
    TOKEN_PROGRAM_ID,
    PDA[0],

  )
  const tx1 = new Transaction().add(
    cancelLoanTx,
  );
  console.log("Sending Alice's cancel transaction...", cancelLoanTx);
  await connection.sendTransaction(
    tx1,
    [borrower_account],
    { skipPreflight: false, preflightCommitment: "confirmed" }
  );

  //
  console.log("xong");
};

loan();
