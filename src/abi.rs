// Codegen from ABI file to interact with the contract.
use alloy::sol;
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IWETH9,
    "src/abi/IWETH9.json"
);

