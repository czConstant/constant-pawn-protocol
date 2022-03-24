import { ASSOCIATED_TOKEN_PROGRAM_ID, Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import {
  Connection, PublicKey, SYSVAR_CLOCK_PUBKEY,
  Transaction
} from "@solana/web3.js";
import {
  getKeypair,
  getProgramId,
  getPublicKey, LiquidateInstruction
} from "./utils";
import BN = require("bn.js");

const liquidate = async () => {
  const lendingProgramId = getProgramId();
  const nft_mint_pubkey = getPublicKey("mint_x");
  const borrower_account = getKeypair("alice");

  const loan_id = getPublicKey("loan");
  const offer_id = getPublicKey("offer");
  const pda_token_account = getPublicKey("pda_token");
  const pda_nft_account = getPublicKey("pda_nft");
  const lender_account = getKeypair("bob");

  const connection = new Connection("https://api.devnet.solana.com", 'singleGossip');
  let tx = new Transaction();
  //create assosiate account if haven't yet
  let assosiatedAccount = (
    await PublicKey.findProgramAddress(
      [borrower_account.publicKey.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), nft_mint_pubkey.toBuffer()],
      ASSOCIATED_TOKEN_PROGRAM_ID,
    )
  )[0];
  const createAssTokenAccountIx = Token.createAssociatedTokenAccountInstruction(
    ASSOCIATED_TOKEN_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
    nft_mint_pubkey,
    assosiatedAccount,
    lender_account.publicKey,
    lender_account.publicKey
  );
  const assosiatedAccountInfo = await connection.getAccountInfo(
    new PublicKey(
      assosiatedAccount
    )
  );
  if (assosiatedAccountInfo === null || assosiatedAccountInfo.data.length === 0) {
    console.log("Assosiated account have not initialized");
    tx.add(createAssTokenAccountIx);
  }
  //liquidate loan instruction
  const PDA = await PublicKey.findProgramAddress([Buffer.from("lending")], lendingProgramId);
  const liquidateTx = LiquidateInstruction(
    lendingProgramId,
    lender_account.publicKey,
    borrower_account.publicKey,
    loan_id,
    offer_id,
    assosiatedAccount,
    pda_nft_account,
    pda_token_account,
    TOKEN_PROGRAM_ID,
    PDA[0],
    SYSVAR_CLOCK_PUBKEY
  )
  tx.add(liquidateTx);
  console.log("Sending liquidate tx...", liquidateTx);
  await connection.sendTransaction(
    tx,
    [lender_account],
    { skipPreflight: false, preflightCommitment: "confirmed" }
  );

  //
  console.log("done");
};

liquidate();
