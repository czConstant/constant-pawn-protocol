import { AccountLayout, Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";
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
  getTokenBalance, InitOfferInstruction, logError, OfferInfoLayout,
  OFFER_INFO_LAYOUT, writePublicKey
} from "./utils";
import BN = require("bn.js");

const offer = async () => {
  const lendingProgramId = getProgramId();
  const terms = getTerms();
  const borrower_account = getKeypair("alice");
  const loan_id = getPublicKey("loan");
  const lender_usd_account_pubkey = getPublicKey("bob_y");
  const usd_mint_pubkey = getPublicKey("mint_y");
  const lender_account = getKeypair("bob");
  const expired = BigInt(60488);//7days
  const temp_usd_account = new Keypair();
  const connection = new Connection("https://api.devnet.solana.com", 'singleGossip');
  const createTempTokenAccountIx = SystemProgram.createAccount({
    programId: TOKEN_PROGRAM_ID,
    space: AccountLayout.span,
    lamports: await connection.getMinimumBalanceForRentExemption(
      AccountLayout.span
    ),
    fromPubkey: lender_account.publicKey,
    newAccountPubkey: temp_usd_account.publicKey,
  });
  const initTempAccountIx = Token.createInitAccountInstruction(
    TOKEN_PROGRAM_ID,
    usd_mint_pubkey,
    temp_usd_account.publicKey,
    lender_account.publicKey
  );
  const transferUsdToTempAccIx = Token.createTransferInstruction(
    TOKEN_PROGRAM_ID,
    lender_usd_account_pubkey,
    temp_usd_account.publicKey,
    lender_account.publicKey,
    [],
    9
  );
  const offer_info_account = new Keypair();
  const createOfferAccountIx = SystemProgram.createAccount({
    space: OFFER_INFO_LAYOUT.span,
    lamports: await connection.getMinimumBalanceForRentExemption(
      OFFER_INFO_LAYOUT.span
    ),
    fromPubkey: lender_account.publicKey,
    newAccountPubkey: offer_info_account.publicKey,
    programId: lendingProgramId,
  });

  const initOfferIx = InitOfferInstruction(
    lendingProgramId,
    lender_account.publicKey,
    borrower_account.publicKey,
    temp_usd_account.publicKey,
    offer_info_account.publicKey,
    SYSVAR_RENT_PUBKEY,
    TOKEN_PROGRAM_ID,
    loan_id,
    terms.loanPrincipalAmount,
    terms.loanDuration,
    terms.interestRate,
    usd_mint_pubkey,
    expired,
  )

  const tx = new Transaction().add(
    createTempTokenAccountIx,
    initTempAccountIx,
    transferUsdToTempAccIx,
    createOfferAccountIx,
    initOfferIx
  );
  console.log("Sending Bob's transaction...");
  await connection.sendTransaction(
    tx,
    [lender_account, temp_usd_account, offer_info_account],
    { skipPreflight: false, preflightCommitment: "confirmed" }
  );

  // sleep to allow time to update
  await new Promise((resolve) => setTimeout(resolve, 3000));

  const offerInfo = await connection.getAccountInfo(
    offer_info_account.publicKey
  );

  if (offerInfo === null || offerInfo.data.length === 0) {
    logError("Offer info has not been initialized properly");
    process.exit(1);
  }

  const encodedOfferState = offerInfo.data;
  const decodedOfferState = OFFER_INFO_LAYOUT.decode(
    encodedOfferState
  ) as OfferInfoLayout;

  if (!decodedOfferState.isInitialized) {
    logError("Loan info initialization flag has not been set");
    process.exit(1);
  } else if (
    !new PublicKey(decodedOfferState.lenderPubkey).equals(
      lender_account.publicKey
    )
  ) {
    logError(
      "borrowerPubkey has not been set correctly / not been set to Alice's public key"
    );
    process.exit(1);
  }
  console.log(
    `✨Offer successfully initialized. Bob is offering ${terms.interestRate}% APR (duration : ${terms.loanDuration} seconds) for ${terms.loanPrincipalAmount} USDT/ETH✨\n`
  );
  writePublicKey(offer_info_account.publicKey, "offer");
  writePublicKey(temp_usd_account.publicKey, "pda_token");
  console.table([
    {
      "Bob Token Account Y": await getTokenBalance(
        getPublicKey("bob_y"),
        connection
      ),
      "Temporary Token Account X": await getTokenBalance(
        temp_usd_account.publicKey,
        connection
      ),
    },
  ]);

  //
  console.log("done");
};

offer();
