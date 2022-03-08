import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import {
  Connection, PublicKey, Transaction
} from "@solana/web3.js";
import {
  CancelOfferInstruction, getKeypair,
  getProgramId,
  getPublicKey
} from "./utils";
import BN = require("bn.js");

const cancel_offer = async () => {
  const lendingProgramId = getProgramId();
  const offer_id = getPublicKey("offer");
  const pda_token_account = getPublicKey("pda_token");
  const lender_token_account_pubkey = getPublicKey("bob_y");
  const lender_account = getKeypair("bob");
  const connection = new Connection("https://api.devnet.solana.com", 'singleGossip');
  // cancel offer
  const PDA = await PublicKey.findProgramAddress([Buffer.from("lending")], lendingProgramId);
  const cancelLoanTx = CancelOfferInstruction(
    lendingProgramId,
    lender_account.publicKey,
    offer_id,
    pda_token_account,
    lender_token_account_pubkey,
    TOKEN_PROGRAM_ID,
    PDA[0],

  )
  const tx1 = new Transaction().add(
    cancelLoanTx,
  );
  console.log("Sending Bob's cancel offer...", cancelLoanTx);
  await connection.sendTransaction(
    tx1,
    [lender_account],
    { skipPreflight: false, preflightCommitment: "confirmed" }
  );

  //
  console.log("done");
};

cancel_offer();
