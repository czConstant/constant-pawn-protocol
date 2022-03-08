import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import {
  Connection, PublicKey, Transaction
} from "@solana/web3.js";
import {
  CloseOfferInstruction, getKeypair,
  getProgramId,
  getPublicKey
} from "./utils";
import BN = require("bn.js");

const closeOffer = async () => {
  const lendingProgramId = getProgramId();
  const offer_id = getPublicKey("offer");
  const pda_token_account = getPublicKey("pda_token");
  const lender_token_account_pubkey = getPublicKey("bob_y");
  const lender_account = getKeypair("bob");

  const connection = new Connection("https://api.devnet.solana.com", 'singleGossip');

  //close offer
  const PDA = await PublicKey.findProgramAddress([Buffer.from("lending")], lendingProgramId);
  const closeOfferInstructionTx = CloseOfferInstruction(
    lendingProgramId,
    lender_account.publicKey,
    offer_id,
    lender_token_account_pubkey,
    pda_token_account,
    TOKEN_PROGRAM_ID,
    PDA[0]
  )
  const tx1 = new Transaction().add(
    closeOfferInstructionTx,
  );
  console.log("Sending close offer tx...", closeOfferInstructionTx);
  await connection.sendTransaction(
    tx1,
    [lender_account],
    { skipPreflight: false, preflightCommitment: "confirmed" }
  );

  //
  console.log("done");
};

closeOffer();
