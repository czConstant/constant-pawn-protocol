use solana_program::msg;
pub const DAY_SECS: u64 = 86400;
pub fn estimate_pay_amount(
    loan_principal_amount: u64,
    loan_duration: u64,
    interest_rate: u64,
) -> u64 {
    let mut result: u64 = 0;
    result = ((loan_principal_amount * interest_rate / 10000) * (loan_duration / DAY_SECS)) / 365;
    result + loan_principal_amount
}
pub fn calculate_fee(loan_principal_amount: u64) -> u64 {
    let fee = loan_principal_amount * 1 / 100;
    fee
}

pub fn calculate_pay_amount(
    loan_principal_amount: u64,
    loan_duration: u64,
    interest_rate: u64,
    loan_started_at: u64,
    pay_at: u64,
) -> u64 {
    //1%(principla) + 100% interest to pay_at + 50% interest for the rest
    let mut max_loan_day: u64 = loan_duration / DAY_SECS;
    if max_loan_day == 0 {
        max_loan_day = 1;
    }
    let mut loan_day: u64 = max_loan_day;
    if pay_at < loan_started_at + loan_duration && pay_at > loan_started_at {
        loan_day = ((pay_at - loan_started_at) / DAY_SECS) + 1;
    }
    if loan_day >= max_loan_day {
        loan_day = max_loan_day
    }
    //100% interest loan day
    let mut full_interst = ((loan_principal_amount * interest_rate / 10000) * loan_day) / 365;
    if max_loan_day > loan_day {
        //50% interest remain day
        full_interst = full_interst
            + (((loan_principal_amount * interest_rate / 10000) * (max_loan_day - loan_day)) / 365)
                / 2;
    }
    //1% fee (base on principal amount)
    let fee = calculate_fee(loan_principal_amount);
    msg!(
        "loan_principal_amount: {}, loan_duration: {},  interest_rate: {},  loan_day: {}, interest: {}, fee : {}",
        loan_principal_amount,
        loan_duration,
        interest_rate,
        loan_day,
        full_interst,
        fee,
    );
    fee + full_interst + loan_principal_amount
}
