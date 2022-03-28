pragma solidity ^0.8.0;
import "./openzeppelin/ECDSA.sol";

contract NFTPawnSigningUtils {
    /* *********** */
    /* CONSTRUCTOR */
    /* *********** */

    constructor() {}

    /* ********* */
    /* FUNCTIONS */
    /* ********* */

    // @notice OpenZeppelin's ECDSA library is used to call all ECDSA functions
    //         directly on the bytes32 variables themselves.
    using ECDSA for bytes32;

    // @notice This function gets the current chain ID.
    function getChainID() public view returns (uint256) {
        uint256 id;
        assembly {
            id := chainid()
        }
        return id;
    }

    function isValidBorrowerSignature(
        uint256 _nftCollateralId,
        uint256 _borrowerNonce,
        address _nftCollateralContract,
        address _borrower,
        bytes memory _borrowerSignature
    ) public view returns (bool) {
        if (_borrower == address(0)) {
            return false;
        } else {
            uint256 chainId;
            chainId = getChainID();
            bytes32 message = keccak256(
                abi.encodePacked(
                    _nftCollateralId,
                    _borrowerNonce,
                    _nftCollateralContract,
                    _borrower,
                    chainId
                )
            );

            bytes32 messageWithEthSignPrefix = message.toEthSignedMessageHash();

            return (messageWithEthSignPrefix.recover(_borrowerSignature) ==
                _borrower);
        }
    }

    function getValidBorrower(
        bytes32 _messageWithEthSignPrefix,
        bytes memory _borrowerSignature
    ) public view returns (address) {
        return _messageWithEthSignPrefix.recover(_borrowerSignature);
    }

    function getValidBorrowerSignature(
        uint256 _nftCollateralId,
        uint256 _borrowerNonce,
        address _nftCollateralContract,
        address _borrower
    ) public view returns (bytes32) {
        bytes32 messageWithEthSignPrefix;
        if (_borrower == address(0)) {
            return messageWithEthSignPrefix;
        } else {
            uint256 chainId;
            chainId = getChainID();
            bytes32 message = keccak256(
                abi.encodePacked(
                    _nftCollateralId,
                    _borrowerNonce,
                    _nftCollateralContract,
                    _borrower,
                    chainId
                )
            );

            messageWithEthSignPrefix = message.toEthSignedMessageHash();

            return messageWithEthSignPrefix;
        }
    }

    function isValidLenderSignature(
        uint256 _loanPrincipalAmount,
        uint256 _nftCollateralId,
        uint256 _loanDuration,
        uint256 _loanInterestRate,
        uint256 _adminFee,
        uint256 _lenderNonce,
        address _nftCollateralContract,
        address _loanCurrency,
        address _lender,
        bytes memory _lenderSignature
    ) public view returns (bool) {
        if (_lender == address(0)) {
            return false;
        } else {
            uint256 chainId;
            chainId = getChainID();
            bytes32 message = keccak256(
                abi.encodePacked(
                    _loanPrincipalAmount,
                    _nftCollateralId,
                    _loanDuration,
                    _loanInterestRate,
                    _adminFee,
                    _lenderNonce,
                    _nftCollateralContract,
                    _loanCurrency,
                    _lender,
                    chainId
                )
            );

            bytes32 messageWithEthSignPrefix = message.toEthSignedMessageHash();

            return (messageWithEthSignPrefix.recover(_lenderSignature) ==
                _lender);
        }
    }
}
