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

  const borrower_account = getKeypair("alice");
  const nft_mint_pubkey = getPublicKey("mint_x");

  const borrower_nft_account_pubkey = (
    await PublicKey.findProgramAddress(
      [borrower_account.publicKey.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), nft_mint_pubkey.toBuffer()],
      ASSOCIATED_TOKEN_PROGRAM_ID,
    )
  )[0];

  const borrower_usd_account_pubkey = getPublicKey("alice_y");
  const usd_mint_pubkey = getPublicKey("mint_y");

  const temp_nft_account = new Keypair();
  const connection = new Connection("https://api.devnet.solana.com", 'singleGossip');
  const createTempTokenAccountIx = SystemProgram.createAccount({
    programId: TOKEN_PROGRAM_ID,
    space: AccountLayout.span,
    lamports: await connection.getMinimumBalanceForRentExemption(
      AccountLayout.span
    ),
    fromPubkey: borrower_account.publicKey,
    newAccountPubkey: temp_nft_account.publicKey,
  });
  const initTempAccountIx = Token.createInitAccountInstruction(
    TOKEN_PROGRAM_ID,
    nft_mint_pubkey,
    temp_nft_account.publicKey,
    borrower_account.publicKey
  );
  const transferNFTToTempAccIx = Token.createTransferInstruction(
    TOKEN_PROGRAM_ID,
    borrower_nft_account_pubkey,
    temp_nft_account.publicKey,
    borrower_account.publicKey,
    [],
    1
  );
  const loan_info_account = new Keypair();
  const createLoanAccountIx = SystemProgram.createAccount({
    space: LOAN_INFO_LAYOUT.span,
    lamports: await connection.getMinimumBalanceForRentExemption(
      LOAN_INFO_LAYOUT.span
    ),
    fromPubkey: borrower_account.publicKey,
    newAccountPubkey: loan_info_account.publicKey,
    programId: lendingProgramId,
  });

  const initLoanIx = InitLoanInstruction(
    lendingProgramId,
    borrower_account.publicKey,
    temp_nft_account.publicKey,
    borrower_usd_account_pubkey,
    loan_info_account.publicKey,
    SYSVAR_RENT_PUBKEY,
    TOKEN_PROGRAM_ID,
    terms.loanPrincipalAmount,
    terms.loanDuration,
    terms.interestRate,
    nft_mint_pubkey,
    usd_mint_pubkey,
  )

  const tx = new Transaction().add(
    createTempTokenAccountIx,
    initTempAccountIx,
    transferNFTToTempAccIx,
    createLoanAccountIx,
    initLoanIx
  );
  console.log("Sending Alice's transaction...");
  await connection.sendTransaction(
    tx,
    [borrower_account, temp_nft_account, borrower_account, loan_info_account],
    { skipPreflight: false, preflightCommitment: "confirmed" }
  );

  // sleep to allow time to update
  await new Promise((resolve) => setTimeout(resolve, 3000));

  const loanInfo = await connection.getAccountInfo(
    loan_info_account.publicKey
  );

  if (loanInfo === null || loanInfo.data.length === 0) {
    logError("Loan info has not been initialized properly");
    process.exit(1);
  }

  const encodedLoanState = loanInfo.data;
  const decodedLoanState = LOAN_INFO_LAYOUT.decode(
    encodedLoanState
  ) as LoanInfoLayout;

  if (!decodedLoanState.isInitialized) {
    logError("Loan info initialization flag has not been set");
    process.exit(1);
  } else if (
    !new PublicKey(decodedLoanState.borrowerPubkey).equals(
      borrower_account.publicKey
    )
  ) {
    logError(
      "borrowerPubkey has not been set correctly / not been set to Alice's public key"
    );
    process.exit(1);
  } else if (
    !new PublicKey(
      decodedLoanState.borrowerDenominationAccountPubkey
    ).equals(borrower_usd_account_pubkey)
  ) {
    logError(
      "borrowerDenominationAccountPubkey has not been set correctly / not been set to Alice's Y public key"
    );
    process.exit(1);
  } else if (
    !new PublicKey(decodedLoanState.collateralAccountPubkey).equals(
      temp_nft_account.publicKey
    )
  ) {
    logError(
      "collateralAccountPubkey has not been set correctly / not been set to temp X token account public key"
    );
    process.exit(1);
  }
  console.log(
    `Loan successfully initialized. Alice is offering ${terms.interestRate}% APR (duration : ${terms.loanDuration} seconds) for ${terms.loanPrincipalAmount} USDT/ETH✨\n`
  );
  writePublicKey(loan_info_account.publicKey, "loan");
  writePublicKey(temp_nft_account.publicKey, "pda_nft");
  console.table([
    {
      "Alice Token Account X": await getTokenBalance(
        borrower_nft_account_pubkey,
        connection
      ),
      "Alice Token Account Y": await getTokenBalance(
        borrower_usd_account_pubkey,
        connection
      ),

      "Bob Token Account Y": await getTokenBalance(
        getPublicKey("bob_y"),
        connection
      ),
      "Temporary Token Account X": await getTokenBalance(
        temp_nft_account.publicKey,
        connection
      ),
    },
  ]);

  console.log("done");
};

loan();
