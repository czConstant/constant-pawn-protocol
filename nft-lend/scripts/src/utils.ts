import { struct, u8, UTF8 } from '@solana/buffer-layout';
import { publicKey, u64 } from '@solana/buffer-layout-utils';
import { Connection, Keypair, PublicKey, TransactionInstruction } from "@solana/web3.js";
import * as fs from "fs";

export const logError = (msg: string) => {
  console.log(`\x1b[31m${msg}\x1b[0m`);
};

export const writePublicKey = (publicKey: PublicKey, name: string) => {
  fs.writeFileSync(
    `./keys/${name}_pub.json`,
    JSON.stringify(publicKey.toString())
  );
};

export const getPublicKey = (name: string) =>
  new PublicKey(
    JSON.parse(fs.readFileSync(`./keys/${name}_pub.json`) as unknown as string)
  );

export const getPrivateKey = (name: string) =>
  Uint8Array.from(
    JSON.parse(fs.readFileSync(`./keys/${name}.json`) as unknown as string)
  );

export const getKeypair = (name: string) =>
  new Keypair({
    publicKey: getPublicKey(name).toBytes(),
    secretKey: getPrivateKey(name),
  });

export const getProgramId = () => {
  try {
    return getPublicKey("program");
  } catch (e) {
    logError("Given programId is missing or incorrect");
    process.exit(1);
  }
};

export const getTerms = (): {
  loanPrincipalAmount: bigint,
  loanDuration: bigint,
  interestRate: bigint
} => {
  return { loanPrincipalAmount: BigInt(2e9), loanDuration: BigInt(60), interestRate: BigInt(1000) };//10%
};

export const getTokenBalance = async (
  pubkey: PublicKey,
  connection: Connection
) => {
  return parseInt(
    (await connection.getTokenAccountBalance(pubkey)).value.amount
  );
};
export interface LoanInfoLayout {
  isInitialized: number;
  borrowerPubkey: PublicKey;
  borrowerDenominationAccountPubkey: PublicKey;
  collateralAccountPubkey: PublicKey;
  loanPrincipalAmount: bigint;
  loanDuration: bigint;
  interestRate: bigint;
  nftCollateralContract: PublicKey;
  loanDenomination: PublicKey;

  status: number,
  loanStartAt: bigint,
  payAmount: bigint,
  lenderPubkey: PublicKey,
  offerInfo: PublicKey,
}

export const LOAN_INFO_LAYOUT = struct<LoanInfoLayout>([
  u8("isInitialized"),
  publicKey("borrowerPubkey"),
  publicKey("collateralAccountPubkey"),
  publicKey("borrowerDenominationAccountPubkey"),
  u64("loanPrincipalAmount"),
  u64("loanDuration"),
  u64("interestRate"),
  publicKey("nftCollateralContract"),
  publicKey("loanDenomination"),
  u8("status"),
  u64("loanStartAt"),
  u64("payAmount"),
  publicKey("lenderPubkey"),
  publicKey("offerInfo"),
]);

export interface OfferInfoLayout {
  isInitialized: number;
  lenderPubkey: PublicKey;
  loanPubkey: PublicKey;
  loanPrincipalAmount: bigint;
  loanDuration: bigint;
  interestRate: bigint;
  loanDenomination: PublicKey;
  tmpDenominationAccountPubkey: PublicKey,
  status: number;
  paidAt: bigint;
  paidAmount: bigint;
}

export const OFFER_INFO_LAYOUT = struct<OfferInfoLayout>([
  u8("isInitialized"),
  publicKey("lenderPubkey"),
  publicKey("loanPubkey"),
  u64("loanPrincipalAmount"),
  u64("loanDuration"),
  u64("interestRate"),
  publicKey("loanDenomination"),
  publicKey("tmpDenominationAccountPubkey"),
  u8("status"),
  u64("paidAt"),
  u64("paidAmount"),
]);


//duynq
export enum LendingInstruction {
  InitLoan = 0,
  MakeOffer = 1,
  Order = 8,
}

export interface InitLoanInstructionData {
  instruction: LendingInstruction.InitLoan;
  loan_principal_amount: bigint,
  loan_duration: bigint,
  interest_rate: bigint,
  nft_collateral_contract: PublicKey,
  loan_currency: PublicKey
}
export const initLoanInstructionData = struct<InitLoanInstructionData>([
  u8('instruction'),
  u64('loan_principal_amount'),
  u64('loan_duration'),
  u64('interest_rate'),
  publicKey('nft_collateral_contract'),
  publicKey('loan_currency'),
]);


export function InitLoanInstruction(
  lending_program_id: PublicKey,
  borrower_account: PublicKey,
  borrower_temp_token_account: PublicKey,
  borrower_denomination_account: PublicKey,
  loan_info_account: PublicKey,
  sys_var_rent: PublicKey,
  token_program: PublicKey,

  loan_principal_amount: bigint,
  loan_duration: bigint,
  interest_rate: bigint,
  nft_collateral_contract: PublicKey,
  loan_currency: PublicKey
): TransactionInstruction {
  const keys = [
    { pubkey: borrower_account, isSigner: true, isWritable: false },
    { pubkey: borrower_temp_token_account, isSigner: false, isWritable: true },
    { pubkey: borrower_denomination_account, isSigner: false, isWritable: false },
    { pubkey: loan_info_account, isSigner: false, isWritable: true },
    { pubkey: sys_var_rent, isSigner: false, isWritable: false },
    { pubkey: token_program, isSigner: false, isWritable: false },
  ];


  const data = Buffer.alloc(initLoanInstructionData.span);
  initLoanInstructionData.encode(
    {
      instruction: LendingInstruction.InitLoan,
      loan_principal_amount: loan_principal_amount,
      loan_duration: loan_duration,
      interest_rate: interest_rate,
      nft_collateral_contract: nft_collateral_contract,
      loan_currency: loan_currency
    },
    data
  );

  return new TransactionInstruction({ keys, programId: lending_program_id, data });
}

export interface InitOfferInstructionData {
  instruction: LendingInstruction.MakeOffer;
  loan_id: PublicKey,
  loan_principal_amount: bigint,
  loan_duration: bigint,
  interest_rate: bigint,
  loan_currency: PublicKey,
}
export const initOfferInstructionData = struct<InitOfferInstructionData>([
  u8('instruction'),
  publicKey('loan_id'),
  u64('loan_principal_amount'),
  u64('loan_duration'),
  u64('interest_rate'),
  publicKey('loan_currency'),
]);


export function InitOfferInstruction(
  lending_program_id: PublicKey,
  lender_account: PublicKey,
  borrower_account: PublicKey,
  lender_temp_token_account: PublicKey,
  offer_info_account: PublicKey,
  sys_var_rent: PublicKey,
  token_program: PublicKey,

  loan_id: PublicKey,
  loan_principal_amount: bigint,
  loan_duration: bigint,
  interest_rate: bigint,
  loan_currency: PublicKey,
): TransactionInstruction {
  const keys = [
    { pubkey: lender_account, isSigner: true, isWritable: false },
    { pubkey: borrower_account, isSigner: false, isWritable: false },
    { pubkey: loan_id, isSigner: false, isWritable: true },
    { pubkey: lender_temp_token_account, isSigner: false, isWritable: true },
    { pubkey: offer_info_account, isSigner: false, isWritable: true },
    { pubkey: sys_var_rent, isSigner: false, isWritable: false },
    { pubkey: token_program, isSigner: false, isWritable: false },
  ];


  const data = Buffer.alloc(initOfferInstructionData.span);
  initOfferInstructionData.encode(
    {
      instruction: LendingInstruction.MakeOffer,
      loan_id: loan_id,
      loan_principal_amount: loan_principal_amount,
      loan_duration: loan_duration,
      interest_rate: interest_rate,
      loan_currency: loan_currency
    },
    data
  );

  return new TransactionInstruction({ keys, programId: lending_program_id, data });
}


export interface CancelLoanInstructionData {
  instruction: number,
  loan_id: PublicKey,
}
export const cancelLoanInstructionData = struct<CancelLoanInstructionData>([
  u8('instruction'),
  publicKey('loan_id'),
]);


export function CancelLoanInstruction(
  lending_program_id: PublicKey,
  borrower_account: PublicKey,
  loan_info_account: PublicKey,
  pda_collateral_account: PublicKey,
  borrower_nft_account: PublicKey,
  token_program: PublicKey,
  pda_account: PublicKey,
): TransactionInstruction {
  const keys = [
    { pubkey: borrower_account, isSigner: true, isWritable: false },
    { pubkey: loan_info_account, isSigner: false, isWritable: true },
    { pubkey: pda_collateral_account, isSigner: false, isWritable: true },
    { pubkey: borrower_nft_account, isSigner: false, isWritable: true },
    { pubkey: token_program, isSigner: false, isWritable: false },
    { pubkey: pda_account, isSigner: false, isWritable: false },
  ];


  const data = Buffer.alloc(cancelLoanInstructionData.span);
  cancelLoanInstructionData.encode(
    {
      instruction: 3,
      loan_id: loan_info_account,
    },
    data
  );

  return new TransactionInstruction({ keys, programId: lending_program_id, data });
}



export interface CancelOfferInstructionData {
  instruction: number,
  offer_id: PublicKey,
}
export const cancelOfferInstructionData = struct<CancelOfferInstructionData>([
  u8('instruction'),
  publicKey('offer_id'),
]);


export function CancelOfferInstruction(
  lending_program_id: PublicKey,
  lender_account: PublicKey,
  offer_info_account: PublicKey,
  pda_token_account: PublicKey,
  lender_token_account: PublicKey,
  token_program: PublicKey,
  pda_account: PublicKey,
): TransactionInstruction {
  const keys = [
    { pubkey: lender_account, isSigner: true, isWritable: false },
    { pubkey: offer_info_account, isSigner: false, isWritable: true },
    { pubkey: pda_token_account, isSigner: false, isWritable: true },
    { pubkey: lender_token_account, isSigner: false, isWritable: true },
    { pubkey: token_program, isSigner: false, isWritable: false },
    { pubkey: pda_account, isSigner: false, isWritable: false },
  ];


  const data = Buffer.alloc(cancelOfferInstructionData.span);
  cancelOfferInstructionData.encode(
    {
      instruction: 4,
      offer_id: offer_info_account,
    },
    data
  );

  return new TransactionInstruction({ keys, programId: lending_program_id, data });
}


//Accept Offer

export interface AcceptOfferInstructionData {
  instruction: number,
  offer_id: PublicKey,
  loan_id: PublicKey,
  loan_principal_amount: bigint,
  loan_duration: bigint,
  interest_rate: bigint,
  loan_currency: PublicKey,
}

export const acceptOfferInstructionData = struct<AcceptOfferInstructionData>([
  u8('instruction'),
  publicKey('loan_id'),
  publicKey('offer_id'),
  u64('loan_principal_amount'),
  u64('loan_duration'),
  u64('interest_rate'),
  publicKey('loan_currency'),
]);


export function AcceptOfferInstruction(
  lending_program_id: PublicKey,
  borrower_account: PublicKey,
  lender_account: PublicKey,
  loan_info_account: PublicKey,
  borrower_token_account: PublicKey,
  offer_info_account: PublicKey,
  pda_token_account: PublicKey,
  token_program: PublicKey,
  pda_account: PublicKey,
  sys_var_clock: PublicKey,
  loan_principal_amount: bigint,
  loan_duration: bigint,
  interest_rate: bigint,
  loan_currency: PublicKey,
): TransactionInstruction {
  const keys = [
    { pubkey: borrower_account, isSigner: true, isWritable: false },
    { pubkey: lender_account, isSigner: false, isWritable: false },
    { pubkey: loan_info_account, isSigner: false, isWritable: true },
    { pubkey: borrower_token_account, isSigner: false, isWritable: true },
    { pubkey: offer_info_account, isSigner: false, isWritable: true },
    { pubkey: pda_token_account, isSigner: false, isWritable: true },
    { pubkey: token_program, isSigner: false, isWritable: false },
    { pubkey: pda_account, isSigner: false, isWritable: false },
    { pubkey: sys_var_clock, isSigner: false, isWritable: false },
  ];


  const data = Buffer.alloc(acceptOfferInstructionData.span);
  acceptOfferInstructionData.encode(
    {
      instruction: 2,
      loan_id: loan_info_account,
      offer_id: offer_info_account,
      loan_principal_amount: loan_principal_amount,
      loan_duration: loan_duration,
      interest_rate: interest_rate,
      loan_currency: loan_currency
    },
    data
  );

  return new TransactionInstruction({ keys, programId: lending_program_id, data });
}

//
//Liquidate Loan

export interface LiquidateInstructionData {
  instruction: number,
  loan_id: PublicKey,
  offer_id: PublicKey,
}

export const liquidateInstructionData = struct<LiquidateInstructionData>([
  u8('instruction'),
  publicKey('loan_id'),
  publicKey('offer_id'),
]);


export function LiquidateInstruction(
  lending_program_id: PublicKey,
  lender_account: PublicKey,
  borrower_account: PublicKey,
  loan_info_account: PublicKey,
  offer_info_account: PublicKey,
  lender_nft_account: PublicKey,
  pda_nft_account: PublicKey,
  pda_token_account: PublicKey,
  token_program: PublicKey,
  pda_account: PublicKey,
  sys_var_clock: PublicKey,
): TransactionInstruction {
  const keys = [
    { pubkey: lender_account, isSigner: true, isWritable: false },
    { pubkey: borrower_account, isSigner: false, isWritable: false },
    { pubkey: loan_info_account, isSigner: false, isWritable: true },
    { pubkey: offer_info_account, isSigner: false, isWritable: true },
    { pubkey: lender_nft_account, isSigner: false, isWritable: true },
    { pubkey: pda_nft_account, isSigner: false, isWritable: true },
    { pubkey: pda_token_account, isSigner: false, isWritable: true },
    { pubkey: token_program, isSigner: false, isWritable: false },
    { pubkey: pda_account, isSigner: false, isWritable: false },
    { pubkey: sys_var_clock, isSigner: false, isWritable: false },
  ];


  const data = Buffer.alloc(liquidateInstructionData.span);
  liquidateInstructionData.encode(
    {
      instruction: 6,
      loan_id: loan_info_account,
      offer_id: offer_info_account,
    },
    data
  );

  return new TransactionInstruction({ keys, programId: lending_program_id, data });
}

//

//Pay Loan

export interface PayInstructionData {
  instruction: number,
  loan_id: PublicKey,
  offer_id: PublicKey,
  pay_amount: bigint
}

export const payInstructionData = struct<PayInstructionData>([
  u8('instruction'),
  publicKey('loan_id'),
  publicKey('offer_id'),
  u64('pay_amount'),
]);


export function PayInstruction(
  lending_program_id: PublicKey,
  borrower_account: PublicKey,
  lender_account: PublicKey,
  loan_info_account: PublicKey,
  offer_info_account: PublicKey,
  borrower_nft_account: PublicKey,
  borrower_token_account: PublicKey,
  pda_nft_account: PublicKey,
  pda_token_account: PublicKey,
  admin_token_pubkey: PublicKey,
  token_program: PublicKey,
  pda_account: PublicKey,
  sys_var_clock: PublicKey,
  pay_amount: bigint,
): TransactionInstruction {
  const keys = [
    { pubkey: borrower_account, isSigner: true, isWritable: false },
    { pubkey: lender_account, isSigner: false, isWritable: false },
    { pubkey: loan_info_account, isSigner: false, isWritable: true },
    { pubkey: offer_info_account, isSigner: false, isWritable: true },
    { pubkey: borrower_nft_account, isSigner: false, isWritable: true },
    { pubkey: borrower_token_account, isSigner: false, isWritable: true },
    { pubkey: pda_nft_account, isSigner: false, isWritable: true },
    { pubkey: pda_token_account, isSigner: false, isWritable: true },
    { pubkey: admin_token_pubkey, isSigner: false, isWritable: true },
    { pubkey: token_program, isSigner: false, isWritable: false },
    { pubkey: pda_account, isSigner: false, isWritable: false },
    { pubkey: sys_var_clock, isSigner: false, isWritable: false },
  ];


  const data = Buffer.alloc(payInstructionData.span);
  payInstructionData.encode(
    {
      instruction: 5,
      loan_id: loan_info_account,
      offer_id: offer_info_account,
      pay_amount: pay_amount,
    },
    data
  );

  return new TransactionInstruction({ keys, programId: lending_program_id, data });
}

//
//Close offer

export interface CloseOfferInstructionData {
  instruction: number,
  offer_id: PublicKey,
}

export const closeOfferInstructionData = struct<CloseOfferInstructionData>([
  u8('instruction'),
  publicKey('offer_id'),
]);


export function CloseOfferInstruction(
  lending_program_id: PublicKey,
  lender_account: PublicKey,
  offer_info_account: PublicKey,
  lender_token_account: PublicKey,
  pda_token_account: PublicKey,
  token_program: PublicKey,
  pda_account: PublicKey,
): TransactionInstruction {
  const keys = [
    { pubkey: lender_account, isSigner: true, isWritable: false },
    { pubkey: offer_info_account, isSigner: false, isWritable: true },
    { pubkey: lender_token_account, isSigner: false, isWritable: true },
    { pubkey: pda_token_account, isSigner: false, isWritable: true },
    { pubkey: token_program, isSigner: false, isWritable: false },
    { pubkey: pda_account, isSigner: false, isWritable: false },
  ];


  const data = Buffer.alloc(closeOfferInstructionData.span);
  closeOfferInstructionData.encode(
    {
      instruction: 7,
      offer_id: offer_info_account,
    },
    data
  );

  return new TransactionInstruction({ keys, programId: lending_program_id, data });
}

//


export interface OrderInstructionData {
  instruction: number;
  loan_id: PublicKey,
}
export const initOrderInstructionData = struct<OrderInstructionData>([
  u8('instruction'),
  publicKey('loan_id'),
]);


export function InitOrderInstruction(
  lending_program_id: PublicKey,
  lender_account: PublicKey,
  borrower_account: PublicKey,
  loan_info_account: PublicKey,
  borrower_token_account: PublicKey,
  lender_temp_token_account: PublicKey,
  offer_info_account: PublicKey,
  sys_var_rent: PublicKey,
  token_program: PublicKey,
  pda_account: PublicKey,
  sys_var_clock: PublicKey,

  loan_id: PublicKey,
): TransactionInstruction {
  const keys = [
    { pubkey: lender_account, isSigner: true, isWritable: false },
    { pubkey: borrower_account, isSigner: false, isWritable: false },
    { pubkey: loan_info_account, isSigner: false, isWritable: true },
    { pubkey: borrower_token_account, isSigner: false, isWritable: true },
    { pubkey: lender_temp_token_account, isSigner: false, isWritable: true },
    { pubkey: offer_info_account, isSigner: false, isWritable: true },
    { pubkey: sys_var_rent, isSigner: false, isWritable: false },
    { pubkey: token_program, isSigner: false, isWritable: false },
    { pubkey: pda_account, isSigner: false, isWritable: false },
    { pubkey: sys_var_clock, isSigner: false, isWritable: false },
  ];


  const data = Buffer.alloc(initOrderInstructionData.span);
  initOrderInstructionData.encode(
    {
      instruction: LendingInstruction.Order,
      loan_id: loan_id,
    },
    data
  );

  return new TransactionInstruction({ keys, programId: lending_program_id, data });
}