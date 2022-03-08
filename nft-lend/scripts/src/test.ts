import {
  Connection, PublicKey
} from "@solana/web3.js";
import {
  getProgramId,
  getPublicKey, LoanInfoLayout, LOAN_INFO_LAYOUT
} from "./utils";
import BN = require("bn.js");

const test = async () => {
  const lendingProgramId = getProgramId();
  // const terms = getTerms();

  // const borrower_nft_account_pubkey = getPublicKey("alice_x");
  // const borrower_usd_account_pubkey = getPublicKey("alice_y");
  // const nft_mint_pubkey = getPublicKey("mint_x");
  // const usd_mint_pubkey = getPublicKey("mint_y");
  // const borrower_account = getKeypair("alice");

  // const temp_nft_account = new Keypair();
  const connection = new Connection("https://api.devnet.solana.com", 'singleGossip');
  // let x = (
  //   await PublicKey.findProgramAddress(
  //     [borrower_account.publicKey.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), nft_mint_pubkey.toBuffer()],
  //     ASSOCIATED_TOKEN_PROGRAM_ID,
  //   )
  // )[0];

  // const createAssTokenAccountIx = Token.createAssociatedTokenAccountInstruction(
  //   ASSOCIATED_TOKEN_PROGRAM_ID,
  //   TOKEN_PROGRAM_ID,
  //   nft_mint_pubkey,
  //   x,
  //   borrower_account.publicKey,
  //   borrower_account.publicKey
  // );

  // const loanInfo = await connection.getAccountInfo(
  //   new PublicKey(
  //     x
  //   )
  // );
  // if (loanInfo === null || loanInfo.data.length === 0) {
  //   console.log("Loan info has not been initialized properly");
  //   process.exit(1);
  // }
  // console.log(loanInfo);

  const loan_id = getPublicKey("loan");
  const PDA = await PublicKey.findProgramAddress([Buffer.from("lending")], lendingProgramId);
  console.log(PDA[0].toString());

  const loanInfo = await connection.getAccountInfo(
    loan_id
  );
  if (loanInfo === null || loanInfo.data.length === 0) {
    process.exit(1);
  }

  const encodedLoanState = loanInfo.data;
  const decodedLoanState = LOAN_INFO_LAYOUT.decode(
    encodedLoanState
  ) as LoanInfoLayout;

  console.log(decodedLoanState.lenderPubkey.toString());
  console.log(decodedLoanState.offerInfo.toString());
};

test();
