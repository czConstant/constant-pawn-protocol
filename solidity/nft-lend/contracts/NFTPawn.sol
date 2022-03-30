pragma solidity ^0.8.0;
import "./openzeppelin/IERC721.sol";
import "./openzeppelin/SafeMath.sol";
import "./NFTPawnAdmin.sol";
import "./NFTPawnSigningUtils.sol";

contract NFTPawn is NFTPawnAdmin, NFTPawnSigningUtils {
    using SafeMath for uint256;

    /* ********** */
    /* DATA TYPES */
    /* ********** */

    // @notice The main Loan struct. The struct fits in six 256-bits words due
    //         to Solidity's rules for struct packing.
    struct Loan {
        // A unique identifier for this particular loan, sourced from the
        // continuously increasing parameter totalNumLoans.
        uint256 loanId;
        // The original sum of money transferred from lender to borrower at the
        // beginning of the loan, measured in loanERC20Denomination's smallest units.
        uint256 loanPrincipalAmount;
        // The amount of time (measured in seconds) that can elapse before the
        // lender can liquidate the loan and seize the underlying collateral.
        uint32 loanDuration;
        // The interest rate
        uint32 loanInterestRate;
        // The ID within the NFTCollateralContract for the NFT being used as
        // collateral for this loan. The NFT is stored within this contract
        // during the duration of the loan.
        uint256 nftCollateralId;
        // The block.timestamp when the loan first began (measured in seconds).
        uint64 loanStartTime;
        // admin fee  (ex: input 1 ~1%)
        uint32 loanAdminFee;
        // The ERC721 contract of the NFT collateral
        address nftCollateralContract;
        // The ERC20 contract of the currency being used as principal/interest
        // for this loan.
        address loanCurrency;
        // The address of the borrower.
        address borrower;
        // The address of the borrower.
        address lender;
    }

    /* ****** */
    /* EVENTS */
    /* ****** */

    event LoanStarted(
        uint256 loanId,
        address borrower,
        address lender,
        uint256 borrowerNonce,
        uint256 lenderNonce
    );

    event OfferNow(
        uint256 loanId,
        address borrower,
        address lender,
        uint256 borrowerNonce,
        uint256 lenderNonce
    );
    event CancelNonce(address sender, uint256 nonce);

    event LoanRepaid(
        uint256 loanId,
        address borrower,
        address lender,
        uint256 loanPrincipalAmount,
        uint256 nftCollateralId,
        uint256 amountPaidToLender,
        uint256 adminFee,
        address nftCollateralContract,
        address loanCurrencry
    );

    event LoanLiquidated(
        uint256 loanId,
        address borrower,
        address lender,
        uint256 loanPrincipalAmount,
        uint256 nftCollateralId,
        uint256 loanMaturityDate,
        uint256 loanLiquidationDate,
        address nftCollateralContract
    );

    /* ******* */
    /* STORAGE */
    /* ******* */

    // @notice A continuously increasing counter that simultaneously allows
    //         every loan to have a unique ID and provides a running count of
    //         how many loans have been started by this contract.
    uint256 public totalNumLoans = 0;

    // @notice A counter of the number of currently outstanding loans.
    uint256 public totalActiveLoans = 0;

    // @notice A mapping from a loan's identifier to the loan's details,
    //         represted by the loan struct. To fetch the lender, call
    //         NFTfi.ownerOf(loanId).
    mapping(uint256 => Loan) public loanIdToLoan;

    // @notice A mapping tracking whether a loan has either been repaid or
    //         liquidated. This prevents an attacker trying to repay or
    //         liquidate the same loan twice.
    mapping(uint256 => bool) public loanRepaidOrLiquidated;

    mapping(address => mapping(uint256 => bool))
        private _nonceHasBeenUsedForUser;

    /* *********** */
    /* CONSTRUCTOR */
    /* *********** */

    // constructor(string memory name, string memory symbol)
    // // ERC721(name, symbol)
    // {

    // }

    constructor() {}

    /* ********* */
    /* FUNCTIONS */
    /* ********* */

    function beginLoan(
        uint256 _loanPrincipalAmount,
        uint256 _nftCollateralId,
        uint256 _loanDuration,
        uint256 _loanInterestRate,
        uint256 _adminFee,
        uint256[2] memory _borrowerAndLenderNonces,
        address _nftCollateralContract,
        address _loanCurrency,
        address _lender,
        bytes[2] memory _borrowerAndLenderSignature
    ) public whenNotPaused nonReentrant {
        // bytes memory _borrowerSignature = _borrowerAndLenderSignature[0];
        bytes memory _lenderSignature = _borrowerAndLenderSignature[1];

        Loan memory loan;

        loan.loanId = totalNumLoans; //currentLoanId,
        loan.loanPrincipalAmount = _loanPrincipalAmount;
        loan.nftCollateralId = _nftCollateralId;
        loan.loanStartTime = uint64(block.timestamp); //_loanStartTime
        loan.loanDuration = uint32(_loanDuration);
        loan.loanInterestRate = uint32(_loanInterestRate);
        loan.loanAdminFee = uint32(_adminFee);
        loan.nftCollateralContract = _nftCollateralContract;
        loan.loanCurrency = _loanCurrency;
        loan.borrower = msg.sender;
        loan.lender = _lender;

        // Sanity check loan values.
        require(
            uint256(loan.loanDuration) <= maximumLoanDuration,
            "Loan duration exceeds maximum loan duration"
        );
        require(
            uint256(loan.loanDuration) != 0,
            "Loan duration cannot be zero"
        );
        require(
            uint256(loan.loanAdminFee) == adminFeeInBasisPoints,
            "The admin fee has changed since this order was signed."
        );

        // Check that both the collateral and the principal come from supported
        // contracts.
        require(
            erc20CurrencyIsWhitelisted[loan.loanCurrency],
            "Currency denomination is not whitelisted to be used by this contract"
        );
        // require(
        //     nftContractIsWhitelisted[loan.nftCollateralContract],
        //     "NFT collateral contract is not whitelisted to be used by this contract"
        // );

        require(
            !_nonceHasBeenUsedForUser[msg.sender][_borrowerAndLenderNonces[0]],
            "Borrower nonce invalid, borrower has either cancelled/begun this loan, or reused this nonce when signing"
        );
        _nonceHasBeenUsedForUser[msg.sender][
            _borrowerAndLenderNonces[0]
        ] = true;
        require(
            !_nonceHasBeenUsedForUser[_lender][_borrowerAndLenderNonces[1]],
            "Lender nonce invalid, lender has either cancelled/begun this loan, or reused this nonce when signing"
        );
        _nonceHasBeenUsedForUser[_lender][_borrowerAndLenderNonces[1]] = true;

        // Check that both signatures are valid.
        // require(
        //     isValidBorrowerSignature(
        //         loan.nftCollateralId,
        //         _borrowerAndLenderNonces[0], //_borrowerNonce,
        //         loan.nftCollateralContract,
        //         msg.sender, //borrower,
        //         _borrowerSignature
        //     ),
        //     "Borrower signature is invalid"
        // );
        require(
            isValidLenderSignature(
                loan.loanPrincipalAmount,
                loan.nftCollateralId,
                loan.loanDuration,
                loan.loanInterestRate,
                loan.loanAdminFee,
                _borrowerAndLenderNonces[1], //_lenderNonce,
                loan.nftCollateralContract,
                loan.loanCurrency,
                _lender,
                _lenderSignature
            ),
            "Lender signature is invalid"
        );

        // Add the loan to storage before moving collateral/principal to follow
        // the Checks-Effects-Interactions pattern.
        loanIdToLoan[totalNumLoans] = loan;
        totalNumLoans = totalNumLoans.add(1);

        // Update number of active loans.
        totalActiveLoans = totalActiveLoans.add(1);
        require(
            totalActiveLoans <= maximumNumberOfActiveLoans,
            "Contract has reached the maximum number of active loans allowed by admins"
        );

        // Transfer collateral from borrower to this contract to be held until
        // loan completion.
        IERC721(loan.nftCollateralContract).transferFrom(
            msg.sender,
            address(this),
            loan.nftCollateralId
        );

        // Transfer principal from lender to borrower.
        IERC20(loan.loanCurrency).transferFrom(
            _lender,
            msg.sender,
            loan.loanPrincipalAmount
        );

        // Issue an ERC721 promissory note to the lender that gives them the
        // right to either the principal-plus-interest or the collateral.
        // _mint(_lender, loan.loanId);

        // Emit an event with all relevant details from this transaction.
        emit LoanStarted(
            loan.loanId,
            loan.borrower,
            loan.lender,
            _borrowerAndLenderNonces[0],
            _borrowerAndLenderNonces[1]
        );
    }

    function offerNow(
        uint256 _loanPrincipalAmount,
        uint256 _nftCollateralId,
        uint256 _loanDuration,
        uint256 _loanInterestRate,
        uint256 _adminFee,
        uint256[2] memory _borrowerAndLenderNonces,
        address _nftCollateralContract,
        address _loanCurrency,
        address _borrower,
        bytes[2] memory _borrowerAndLenderSignature
    ) public whenNotPaused nonReentrant {
        bytes memory _borrowerSignature = _borrowerAndLenderSignature[0];
        bytes memory _lenderSignature = _borrowerAndLenderSignature[1];

        Loan memory loan;

        loan.loanId = totalNumLoans; //currentLoanId,
        loan.loanPrincipalAmount = _loanPrincipalAmount;
        loan.nftCollateralId = _nftCollateralId;
        loan.loanStartTime = uint64(block.timestamp); //_loanStartTime
        loan.loanDuration = uint32(_loanDuration);
        loan.loanInterestRate = uint32(_loanInterestRate);
        loan.loanAdminFee = uint32(_adminFee);
        loan.nftCollateralContract = _nftCollateralContract;
        loan.loanCurrency = _loanCurrency;
        loan.lender = msg.sender;
        loan.borrower = _borrower;

        // Sanity check loan values.
        require(
            uint256(loan.loanDuration) <= maximumLoanDuration,
            "Loan duration exceeds maximum loan duration"
        );
        require(
            uint256(loan.loanDuration) != 0,
            "Loan duration cannot be zero"
        );
        require(
            uint256(loan.loanAdminFee) == adminFeeInBasisPoints,
            "The admin fee has changed since this order was signed."
        );

        // Check that both the collateral and the principal come from supported
        // contracts.
        require(
            erc20CurrencyIsWhitelisted[loan.loanCurrency],
            "Currency denomination is not whitelisted to be used by this contract"
        );
        // require(
        //     nftContractIsWhitelisted[loan.nftCollateralContract],
        //     "NFT collateral contract is not whitelisted to be used by this contract"
        // );

        require(
            !_nonceHasBeenUsedForUser[_borrower][_borrowerAndLenderNonces[0]],
            "Borrower nonce invalid, borrower has either cancelled/begun this loan, or reused this nonce when signing"
        );
        _nonceHasBeenUsedForUser[_borrower][_borrowerAndLenderNonces[0]] = true;
        require(
            !_nonceHasBeenUsedForUser[msg.sender][_borrowerAndLenderNonces[1]],
            "Lender nonce invalid, lender has either cancelled/begun this loan, or reused this nonce when signing"
        );
        _nonceHasBeenUsedForUser[msg.sender][
            _borrowerAndLenderNonces[1]
        ] = true;

        // Check that both signatures are valid.
        require(
            isValidLenderSignature(
                loan.loanPrincipalAmount,
                loan.nftCollateralId,
                loan.loanDuration,
                loan.loanInterestRate,
                loan.loanAdminFee,
                _borrowerAndLenderNonces[0], //_borrowerNonce,
                loan.nftCollateralContract,
                loan.loanCurrency,
                _borrower,
                _borrowerSignature
            ),
            "Borrower signature is invalid"
        );
        require(
            isValidLenderSignature(
                loan.loanPrincipalAmount,
                loan.nftCollateralId,
                loan.loanDuration,
                loan.loanInterestRate,
                loan.loanAdminFee,
                _borrowerAndLenderNonces[1], //_lenderNonce,
                loan.nftCollateralContract,
                loan.loanCurrency,
                msg.sender,
                _lenderSignature
            ),
            "Lender signature is invalid"
        );

        // Add the loan to storage before moving collateral/principal to follow
        // the Checks-Effects-Interactions pattern.
        loanIdToLoan[totalNumLoans] = loan;
        totalNumLoans = totalNumLoans.add(1);

        // Update number of active loans.
        totalActiveLoans = totalActiveLoans.add(1);
        require(
            totalActiveLoans <= maximumNumberOfActiveLoans,
            "Contract has reached the maximum number of active loans allowed by admins"
        );

        // Transfer collateral from borrower to this contract to be held until
        // loan completion.
        IERC721(loan.nftCollateralContract).transferFrom(
            _borrower,
            address(this),
            loan.nftCollateralId
        );

        // Transfer principal from lender to borrower.
        IERC20(loan.loanCurrency).transferFrom(
            msg.sender,
            _borrower,
            loan.loanPrincipalAmount
        );

        // Issue an ERC721 promissory note to the lender that gives them the
        // right to either the principal-plus-interest or the collateral.
        // _mint(_lender, loan.loanId);

        // Emit an event with all relevant details from this transaction.
        emit OfferNow(
            loan.loanId,
            loan.borrower,
            loan.lender,
            _borrowerAndLenderNonces[0],
            _borrowerAndLenderNonces[1]
        );
    }

    function payBackLoan(uint256 _loanId) external nonReentrant {
        // Sanity check that payBackLoan() and liquidateOverdueLoan() have
        // never been called on this loanId. Depending on how the rest of the
        // code turns out, this check may be unnecessary.
        require(
            !loanRepaidOrLiquidated[_loanId],
            "Loan has already been repaid or liquidated"
        );

        // Fetch loan details from storage, but store them in memory for the
        // sake of saving gas.
        Loan memory loan = loanIdToLoan[_loanId];

        // Check that the borrower is the caller, only the borrower is entitled
        // to the collateral.
        require(
            msg.sender == loan.borrower,
            "Only the borrower can pay back a loan and reclaim the underlying NFT"
        );

        address lender = loan.lender;

        uint256 interestDue = _computeInterestDue(
            loan.loanPrincipalAmount,
            loan.loanInterestRate,
            (uint256(block.timestamp)).sub(loan.loanStartTime),
            loan.loanDuration
        );

        uint256 adminFee = _computeAdminFee(
            interestDue,
            uint256(loan.loanAdminFee)
        );
        uint256 payoffAmount = ((loan.loanPrincipalAmount).add(interestDue))
            .sub(adminFee);

        // Mark loan as repaid before doing any external transfers to follow
        // the Checks-Effects-Interactions design pattern.
        loanRepaidOrLiquidated[_loanId] = true;

        // Update number of active loans.
        totalActiveLoans = totalActiveLoans.sub(1);

        // Transfer principal-plus-interest-minus-fees from borrower to lender
        IERC20(loan.loanCurrency).transferFrom(
            loan.borrower,
            lender,
            payoffAmount
        );

        // Transfer fees from borrower to admins
        IERC20(loan.loanCurrency).transferFrom(
            loan.borrower,
            owner(),
            adminFee
        );

        // Transfer collateral from this contract to borrower.
        require(
            _transferNftToAddress(
                loan.nftCollateralContract,
                loan.nftCollateralId,
                loan.borrower
            ),
            "NFT was not successfully transferred"
        );

        // Destroy the lender's promissory note.
        // _burn(_loanId);

        // Emit an event with all relevant details from this transaction.
        emit LoanRepaid(
            _loanId,
            loan.borrower,
            lender,
            loan.loanPrincipalAmount,
            loan.nftCollateralId,
            payoffAmount,
            adminFee,
            loan.nftCollateralContract,
            loan.loanCurrency
        );

        delete loanIdToLoan[_loanId];
    }

    function liquidateOverdueLoan(uint256 _loanId) external nonReentrant {
        // Sanity check that payBackLoan() and liquidateOverdueLoan() have
        // never been called on this loanId. Depending on how the rest of the
        // code turns out, this check may be unnecessary.
        require(
            !loanRepaidOrLiquidated[_loanId],
            "Loan has already been repaid or liquidated"
        );

        // Fetch loan details from storage, but store them in memory for the
        // sake of saving gas.
        Loan memory loan = loanIdToLoan[_loanId];

        // Ensure that the loan is indeed overdue, since we can only liquidate
        // overdue loans.
        uint256 loanMaturityDate = (uint256(loan.loanStartTime)).add(
            uint256(loan.loanDuration)
        );
        require(block.timestamp > loanMaturityDate, "Loan is not overdue yet");

        address lender = loan.lender;

        // Mark loan as liquidated before doing any external transfers to
        // follow the Checks-Effects-Interactions design pattern.
        loanRepaidOrLiquidated[_loanId] = true;

        // Update number of active loans.
        totalActiveLoans = totalActiveLoans.sub(1);

        // Transfer collateral from this contract to the lender, since the
        // lender is seizing collateral for an overdue loan.
        require(
            _transferNftToAddress(
                loan.nftCollateralContract,
                loan.nftCollateralId,
                lender
            ),
            "NFT was not successfully transferred"
        );

        // Destroy the lender's promissory note for this loan, since by seizing
        // the collateral, the lender has forfeit the rights to the loan
        // principal-plus-interest.
        // _burn(_loanId);

        // Emit an event with all relevant details from this transaction.
        emit LoanLiquidated(
            _loanId,
            loan.borrower,
            lender,
            loan.loanPrincipalAmount,
            loan.nftCollateralId,
            loanMaturityDate,
            block.timestamp,
            loan.nftCollateralContract
        );

        // Delete the loan from storage in order to achieve a substantial gas
        // savings and to lessen the burden of storage on Ethereum nodes, since
        // we will never access this loan's details again, and the details are
        // still available through event data.
        delete loanIdToLoan[_loanId];
    }

    function cancelLoanCommitmentBeforeLoanHasBegun(uint256 _nonce) external {
        require(
            !_nonceHasBeenUsedForUser[msg.sender][_nonce],
            "Nonce invalid, user has either cancelled/begun this loan, or reused a nonce when signing"
        );
        _nonceHasBeenUsedForUser[msg.sender][_nonce] = true;
        emit CancelNonce(msg.sender, _nonce);
    }

    /* ******************* */
    /* READ-ONLY FUNCTIONS */
    /* ******************* */
    function getWhetherNonceHasBeenUsedForUser(address _user, uint256 _nonce)
        public
        view
        returns (bool)
    {
        return _nonceHasBeenUsedForUser[_user][_nonce];
    }

    function getAdminFee() public view returns (uint256) {
        return adminFeeInBasisPoints;
    }

    /* ****************** */
    /* INTERNAL FUNCTIONS */
    /* ****************** */

    function _computeInterestDue(
        uint256 _loanPrincipalAmount,
        uint256 _loanInterestRate,
        uint256 _loanDurationSoFarInSeconds,
        uint256 _loanTotalDurationAgreedTo
    ) internal pure returns (uint256) {
        if (_loanDurationSoFarInSeconds > _loanTotalDurationAgreedTo) {
            _loanDurationSoFarInSeconds = _loanTotalDurationAgreedTo;
        }
        uint256 totalLoanDay = _loanTotalDurationAgreedTo.div(86400);
        if (totalLoanDay == 0) {
            totalLoanDay = 1;
        }
        uint256 sofarLoanDay = (_loanDurationSoFarInSeconds.div(86400)).add(1);
        if (sofarLoanDay > totalLoanDay) {
            sofarLoanDay = totalLoanDay;
        }
        uint256 interestDueAfterEntireDuration = (
            _loanPrincipalAmount.mul(_loanInterestRate)
        ).div(uint256(10000));
        uint256 interestDueAfterElapsedDuration = (
            interestDueAfterEntireDuration.mul(sofarLoanDay)
        ).div(totalLoanDay);

        uint256 remainInterestDueAfterElapsedDuration = interestDueAfterEntireDuration
                .sub(interestDueAfterElapsedDuration);

        return
            interestDueAfterElapsedDuration.add(
                remainInterestDueAfterElapsedDuration.div(2)
            );
    }

    function _computeAdminFee(
        uint256 _interestDue,
        uint256 _adminFeeInBasisPoints
    ) internal pure returns (uint256) {
        return (_interestDue.mul(_adminFeeInBasisPoints)).div(10000);
    }

    function _transferNftToAddress(
        address _nftContract,
        uint256 _nftId,
        address _recipient
    ) internal returns (bool) {
        // Try to call transferFrom()
        bool transferFromSucceeded = _attemptTransferFrom(
            _nftContract,
            _nftId,
            _recipient
        );
        if (transferFromSucceeded) {
            return true;
        } else {
            return false;
        }
    }

    function _attemptTransferFrom(
        address _nftContract,
        uint256 _nftId,
        address _recipient
    ) internal returns (bool) {
        // @notice Some NFT contracts will not allow you to approve an NFT that
        //         you own, so we cannot simply call approve() here, we have to
        //         try to call it in a manner that allows the call to fail.
        _nftContract.call(
            abi.encodeWithSelector(
                IERC721(_nftContract).approve.selector,
                address(this),
                _nftId
            )
        );

        // @notice Some NFT contracts will not allow you to call transferFrom()
        //         for an NFT that you own but that is not approved, so we
        //         cannot simply call transferFrom() here, we have to try to
        //         call it in a manner that allows the call to fail.
        (bool success, ) = _nftContract.call(
            abi.encodeWithSelector(
                IERC721(_nftContract).transferFrom.selector,
                address(this),
                _recipient,
                _nftId
            )
        );
        return success;
    }

    //     /* ***************** */
    //     /* FALLBACK FUNCTION */
    //     /* ***************** */

    // @notice By calling 'revert' in the fallback function, we prevent anyone
    //         from accidentally sending funds directly to this contract.
    fallback() external payable {
        revert();
    }
}
