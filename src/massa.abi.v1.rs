/// Error message used in abi Response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    /// a string representing the error
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
/// Create SC request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateScRequest {
    /// Bytecode is the compiled code of the smart contract
    #[prost(bytes = "vec", tag = "1")]
    pub bytecode: ::prost::alloc::vec::Vec<u8>,
}
/// Create SC result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateScResult {
    /// Address of the just created smart contract
    #[prost(string, tag = "1")]
    pub sc_address: ::prost::alloc::string::String,
}
/// Call SC request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallRequest {
    /// Address of the smart contract to call
    #[prost(string, tag = "1")]
    pub target_sc_address: ::prost::alloc::string::String,
    /// Function to call in the targeted smart contract
    #[prost(string, tag = "2")]
    pub target_function_name: ::prost::alloc::string::String,
    /// Argument to the function serialized in a byte array.
    #[prost(bytes = "vec", tag = "3")]
    pub function_arg: ::prost::alloc::vec::Vec<u8>,
    /// call_coins is the amount of coins to pay for the call
    #[prost(message, optional, tag = "4")]
    pub call_coins: ::core::option::Option<super::super::model::v1::NativeAmount>,
}
/// Call SC response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallResponse {
    /// Return_data is the return value of the function
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Local execution request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalExecutionRequest {
    /// Bytecode is the compiled code of the smart contract
    #[prost(bytes = "vec", tag = "1")]
    pub bytecode: ::prost::alloc::vec::Vec<u8>,
    /// Function to call in the targeted smart contract
    #[prost(string, tag = "2")]
    pub target_function_name: ::prost::alloc::string::String,
    /// Argument to the function serialized in a byte array.
    #[prost(bytes = "vec", tag = "3")]
    pub function_arg: ::prost::alloc::vec::Vec<u8>,
}
/// Local call response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalExecutionResponse {
    /// Return_data is the return value of the function
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Generate event request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateEventRequest {
    /// Event
    #[prost(bytes = "vec", tag = "1")]
    pub event: ::prost::alloc::vec::Vec<u8>,
}
/// Generate event resulst
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateEventResult {}
/// Transfer coins request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferCoinsRequest {
    /// The address of the recipient
    #[prost(string, tag = "1")]
    pub target_address: ::prost::alloc::string::String,
    /// The amount of coins to transfer
    #[prost(message, optional, tag = "2")]
    pub amount_to_transfer: ::core::option::Option<
        super::super::model::v1::NativeAmount,
    >,
    /// The address of the sender, if none, use current address
    #[prost(message, optional, tag = "3")]
    pub sender_address: ::core::option::Option<::prost::alloc::string::String>,
}
/// Transfer coin result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferCoinsResult {}
/// Function exists request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionExistsRequest {
    /// Address of the smart contract to call
    #[prost(string, tag = "1")]
    pub target_sc_address: ::prost::alloc::string::String,
    /// The name of the function to check the existance of
    #[prost(string, tag = "2")]
    pub function_name: ::prost::alloc::string::String,
}
/// Function exists result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionExistsResult {
    /// Exists is true if the function exists
    #[prost(bool, tag = "1")]
    pub exists: bool,
}
/// Tips to check for completeness exec:
/// `rg message | rg "\{" | rg "Result" | wc -l`
/// the given count should be equal to the number of messages in RespResult + 1
/// the +1 comes from ResResult itself which is counted above
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RespResult {
    #[prost(
        oneof = "resp_result::Res",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67"
    )]
    pub res: ::core::option::Option<resp_result::Res>,
}
/// Nested message and enum types in `RespResult`.
pub mod resp_result {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Res {
        #[prost(message, tag = "1")]
        AddNativeAmountResult(super::AddNativeAmountResult),
        #[prost(message, tag = "2")]
        AddressFromPubKeyResult(super::AddressFromPubKeyResult),
        #[prost(message, tag = "3")]
        AppendDsValueResult(super::AppendDsValueResult),
        #[prost(message, tag = "4")]
        Base58CheckToBytesResult(super::Base58CheckToBytesResult),
        #[prost(message, tag = "5")]
        BytesToBase58CheckResult(super::BytesToBase58CheckResult),
        #[prost(message, tag = "6")]
        CallerHasWriteAccessResult(super::CallerHasWriteAccessResult),
        #[prost(message, tag = "7")]
        CheckAddressResult(super::CheckAddressResult),
        #[prost(message, tag = "8")]
        CheckNativeAmountResult(super::CheckNativeAmountResult),
        #[prost(message, tag = "9")]
        CheckPubKeyResult(super::CheckPubKeyResult),
        #[prost(message, tag = "10")]
        CheckSigResult(super::CheckSigResult),
        #[prost(message, tag = "11")]
        CheckedAddNativeTimeResult(super::CheckedAddNativeTimeResult),
        #[prost(message, tag = "12")]
        CheckedDivRemNativeTimeResult(super::CheckedDivRemNativeTimeResult),
        #[prost(message, tag = "13")]
        CheckedScalarDivRemNativeTimeResult(super::CheckedScalarDivRemNativeTimeResult),
        #[prost(message, tag = "14")]
        CheckedScalarMulNativeTimeResult(super::CheckedScalarMulNativeTimeResult),
        #[prost(message, tag = "15")]
        CheckedSubNativeTimeResult(super::CheckedSubNativeTimeResult),
        #[prost(message, tag = "16")]
        CompareAddressResult(super::CompareAddressResult),
        #[prost(message, tag = "17")]
        CompareNativeAmountResult(super::CompareNativeAmountResult),
        #[prost(message, tag = "18")]
        CompareNativeTimeResult(super::CompareNativeTimeResult),
        #[prost(message, tag = "19")]
        ComparePubKeyResult(super::ComparePubKeyResult),
        #[prost(message, tag = "20")]
        CreateScResult(super::CreateScResult),
        #[prost(message, tag = "21")]
        DeleteDsEntryResult(super::DeleteDsEntryResult),
        #[prost(message, tag = "22")]
        DivRemNativeAmountResult(super::DivRemNativeAmountResult),
        #[prost(message, tag = "23")]
        DsEntryExistsResult(super::DsEntryExistsResult),
        #[prost(message, tag = "24")]
        FunctionExistsResult(super::FunctionExistsResult),
        #[prost(message, tag = "25")]
        GenerateEventResult(super::GenerateEventResult),
        #[prost(message, tag = "26")]
        GetAddressCategoryResult(super::GetAddressCategoryResult),
        #[prost(message, tag = "27")]
        GetAddressVersionResult(super::GetAddressVersionResult),
        #[prost(message, tag = "28")]
        GetBalanceResult(super::GetBalanceResult),
        #[prost(message, tag = "29")]
        GetBytecodeResult(super::GetBytecodeResult),
        #[prost(message, tag = "30")]
        GetCallCoinsResult(super::GetCallCoinsResult),
        #[prost(message, tag = "31")]
        GetCallStackResult(super::GetCallStackResult),
        #[prost(message, tag = "32")]
        GetCurrentSlotResult(super::GetCurrentSlotResult),
        #[prost(message, tag = "33")]
        GetDsKeysResult(super::GetDsKeysResult),
        #[prost(message, tag = "34")]
        GetDsValueResult(super::GetDsValueResult),
        #[prost(message, tag = "35")]
        GetNativeTimeResult(super::GetNativeTimeResult),
        #[prost(message, tag = "36")]
        GetOpDataResult(super::GetOpDataResult),
        #[prost(message, tag = "37")]
        GetOpKeysResult(super::GetOpKeysResult),
        #[prost(message, tag = "38")]
        GetOriginOperationIdResult(super::GetOriginOperationIdResult),
        #[prost(message, tag = "39")]
        GetOwnedAddressesResult(super::GetOwnedAddressesResult),
        #[prost(message, tag = "40")]
        GetPubKeyVersionResult(super::GetPubKeyVersionResult),
        #[prost(message, tag = "41")]
        GetRemainingGasResult(super::GetRemainingGasResult),
        #[prost(message, tag = "42")]
        GetSignatureVersionResult(super::GetSignatureVersionResult),
        #[prost(message, tag = "43")]
        HashBlake3Result(super::HashBlake3Result),
        #[prost(message, tag = "44")]
        HashSha256Result(super::HashSha256Result),
        #[prost(message, tag = "45")]
        OpEntryExistsResult(super::OpEntryExistsResult),
        #[prost(message, tag = "46")]
        Keccak256Result(super::Keccak256Result),
        #[prost(message, tag = "47")]
        LocalExecutionResponse(super::LocalExecutionResponse),
        #[prost(message, tag = "48")]
        NativeAmountFromStringResult(super::NativeAmountFromStringResult),
        #[prost(message, tag = "49")]
        NativeAmountToStringResult(super::NativeAmountToStringResult),
        #[prost(message, tag = "50")]
        ScalarDivRemNativeAmountResult(super::ScalarDivRemNativeAmountResult),
        #[prost(message, tag = "51")]
        ScalarMulNativeAmountResult(super::ScalarMulNativeAmountResult),
        #[prost(message, tag = "52")]
        SendAsyncMessageResult(super::SendAsyncMessageResult),
        #[prost(message, tag = "53")]
        SetBytecodeResult(super::SetBytecodeResult),
        #[prost(message, tag = "54")]
        SetDsValueResult(super::SetDsValueResult),
        #[prost(message, tag = "55")]
        SubNativeAmountResult(super::SubNativeAmountResult),
        #[prost(message, tag = "56")]
        TransferCoinsResult(super::TransferCoinsResult),
        #[prost(message, tag = "57")]
        UnsafeRandomResult(super::UnsafeRandomResult),
        #[prost(message, tag = "58")]
        EvmVerifySigResult(super::EvmVerifySigResult),
        #[prost(message, tag = "59")]
        VerifySigResult(super::VerifySigResult),
        #[prost(message, tag = "60")]
        EvmGetAddressFromPubkeyResult(super::EvmGetAddressFromPubkeyResult),
        #[prost(message, tag = "61")]
        EvmGetPubkeyFromSignatureResult(super::EvmGetPubkeyFromSignatureResult),
        #[prost(message, tag = "62")]
        IsAddressEoaResult(super::IsAddressEoaResult),
        #[prost(message, tag = "63")]
        ChainIdResult(super::ChainIdResult),
        #[prost(message, tag = "64")]
        DeferredCallQuoteResult(super::DeferredCallQuoteResult),
        #[prost(message, tag = "65")]
        DeferredCallExistsResult(super::DeferredCallExistsResult),
        #[prost(message, tag = "66")]
        DeferredCallRegisterResult(super::DeferredCallRegisterResult),
        #[prost(message, tag = "67")]
        DeferredCallCancelResult(super::DeferredCallCancelResult),
    }
}
/// Generic message that encapsulate response from ABI calls.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbiResponse {
    #[prost(oneof = "abi_response::Resp", tags = "1, 2")]
    pub resp: ::core::option::Option<abi_response::Resp>,
}
/// Nested message and enum types in `AbiResponse`.
pub mod abi_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Resp {
        /// variant for success
        #[prost(message, tag = "1")]
        Res(super::RespResult),
        /// variant for error
        #[prost(message, tag = "2")]
        Error(super::Error),
    }
}
/// Amount to string request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeAmountToStringRequest {
    /// Amount to convert to string
    #[prost(message, optional, tag = "1")]
    pub to_convert: ::core::option::Option<super::super::model::v1::NativeAmount>,
}
/// Amount to string result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeAmountToStringResult {
    /// Converted Amount
    #[prost(string, tag = "1")]
    pub converted_amount: ::prost::alloc::string::String,
}
/// Amount from string request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeAmountFromStringRequest {
    /// String to convert to Amount
    #[prost(string, tag = "1")]
    pub to_convert: ::prost::alloc::string::String,
}
/// Amount from string result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeAmountFromStringResult {
    /// Converted Amount
    #[prost(message, optional, tag = "1")]
    pub converted_amount: ::core::option::Option<super::super::model::v1::NativeAmount>,
}
/// Check address request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckAddressRequest {
    /// Address to check
    #[prost(string, tag = "1")]
    pub to_check: ::prost::alloc::string::String,
}
/// Check address result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckAddressResult {
    /// Is address valid
    #[prost(bool, tag = "1")]
    pub is_valid: bool,
}
/// Check PubKey request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckPubKeyRequest {
    /// PubKey to check
    #[prost(string, tag = "1")]
    pub to_check: ::prost::alloc::string::String,
}
/// Check PubKey result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckPubKeyResult {
    /// Is PubKey valid
    #[prost(bool, tag = "1")]
    pub is_valid: bool,
}
/// Check Sig request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckSigRequest {
    /// Sig to check
    #[prost(string, tag = "1")]
    pub to_check: ::prost::alloc::string::String,
}
/// Check Sig result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckSigResult {
    /// Is Sig valid
    #[prost(bool, tag = "1")]
    pub is_valid: bool,
}
/// Check Amount request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckNativeAmountRequest {
    /// Amount to check
    #[prost(message, optional, tag = "1")]
    pub to_check: ::core::option::Option<super::super::model::v1::NativeAmount>,
}
/// Check Amount result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckNativeAmountResult {
    /// Is Amount valid
    #[prost(bool, tag = "1")]
    pub is_valid: bool,
}
/// Amount addition request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddNativeAmountRequest {
    /// First amount to add
    #[prost(message, optional, tag = "1")]
    pub amount1: ::core::option::Option<super::super::model::v1::NativeAmount>,
    /// Second amount to add
    #[prost(message, optional, tag = "2")]
    pub amount2: ::core::option::Option<super::super::model::v1::NativeAmount>,
}
/// Amount addition result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddNativeAmountResult {
    /// Sum of amounts
    #[prost(message, optional, tag = "1")]
    pub sum: ::core::option::Option<super::super::model::v1::NativeAmount>,
}
/// Amount subtraction request
/// try to compute difference = left - right
/// fails if right > left
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubNativeAmountRequest {
    /// First amount to subtract from
    #[prost(message, optional, tag = "1")]
    pub left: ::core::option::Option<super::super::model::v1::NativeAmount>,
    /// Second amount to subtract
    #[prost(message, optional, tag = "2")]
    pub right: ::core::option::Option<super::super::model::v1::NativeAmount>,
}
/// Amount subtraction result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubNativeAmountResult {
    /// Difference of amounts (left - right)
    #[prost(message, optional, tag = "1")]
    pub difference: ::core::option::Option<super::super::model::v1::NativeAmount>,
}
/// Amount multiplication request
/// Try to compute product = amount * coefficient (fail if overflow)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScalarMulNativeAmountRequest {
    /// Amount to multiply
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::super::model::v1::NativeAmount>,
    /// Coefficient to multiply by
    #[prost(uint64, tag = "2")]
    pub coefficient: u64,
}
/// Amount multiplication result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScalarMulNativeAmountResult {
    /// Product of amount and coefficient
    #[prost(message, optional, tag = "1")]
    pub product: ::core::option::Option<super::super::model::v1::NativeAmount>,
}
/// Amount division by scalar request
/// Try to compute quotient = dividend / divisor
/// Fails if divisor == 0
/// Fails if underflow
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScalarDivRemNativeAmountRequest {
    /// Amount to divide
    #[prost(message, optional, tag = "1")]
    pub dividend: ::core::option::Option<super::super::model::v1::NativeAmount>,
    /// Divisor to divide by
    #[prost(uint64, tag = "2")]
    pub divisor: u64,
}
/// Amount division by scalar result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScalarDivRemNativeAmountResult {
    /// Quotient of amount and divisor
    #[prost(message, optional, tag = "1")]
    pub quotient: ::core::option::Option<super::super::model::v1::NativeAmount>,
    /// Remainder of amount and divisor
    #[prost(message, optional, tag = "2")]
    pub remainder: ::core::option::Option<super::super::model::v1::NativeAmount>,
}
/// Amount division request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DivRemNativeAmountRequest {
    /// Amount to divide
    #[prost(message, optional, tag = "1")]
    pub dividend: ::core::option::Option<super::super::model::v1::NativeAmount>,
    /// Divisor to divide by
    #[prost(message, optional, tag = "2")]
    pub divisor: ::core::option::Option<super::super::model::v1::NativeAmount>,
}
/// Amount division result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DivRemNativeAmountResult {
    /// Quotient of amount and divisor
    #[prost(uint64, tag = "1")]
    pub quotient: u64,
    /// Remainder of amount and divisor
    #[prost(message, optional, tag = "2")]
    pub remainder: ::core::option::Option<super::super::model::v1::NativeAmount>,
}
/// Time addition checked request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckedAddNativeTimeRequest {
    /// First time to add
    #[prost(message, optional, tag = "1")]
    pub left: ::core::option::Option<super::super::model::v1::NativeTime>,
    /// Second time to add
    #[prost(message, optional, tag = "2")]
    pub right: ::core::option::Option<super::super::model::v1::NativeTime>,
}
/// Time addition checked result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckedAddNativeTimeResult {
    /// Sum of times
    #[prost(message, optional, tag = "1")]
    pub sum: ::core::option::Option<super::super::model::v1::NativeTime>,
}
/// Time subtraction checked request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckedSubNativeTimeRequest {
    /// First time to subtract from
    #[prost(message, optional, tag = "1")]
    pub left: ::core::option::Option<super::super::model::v1::NativeTime>,
    /// Second time to subtract
    #[prost(message, optional, tag = "2")]
    pub right: ::core::option::Option<super::super::model::v1::NativeTime>,
}
/// Time subtraction checked result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckedSubNativeTimeResult {
    /// Difference of times (left - right)
    #[prost(message, optional, tag = "1")]
    pub difference: ::core::option::Option<super::super::model::v1::NativeTime>,
}
/// Time scalar mult checked request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckedScalarMulNativeTimeRequest {
    /// Time to multiply
    #[prost(message, optional, tag = "1")]
    pub time: ::core::option::Option<super::super::model::v1::NativeTime>,
    /// Coefficient to multiply by
    #[prost(uint64, tag = "2")]
    pub coefficient: u64,
}
/// Time scalar mult checked result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckedScalarMulNativeTimeResult {
    /// Product of time and coefficient
    #[prost(message, optional, tag = "1")]
    pub product: ::core::option::Option<super::super::model::v1::NativeTime>,
}
/// Time scalar divrem checked request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckedScalarDivRemNativeTimeRequest {
    /// Time to divide
    #[prost(message, optional, tag = "1")]
    pub dividend: ::core::option::Option<super::super::model::v1::NativeTime>,
    /// Divisor to divide by
    #[prost(uint64, tag = "2")]
    pub divisor: u64,
}
/// Time scalar divrem checked result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckedScalarDivRemNativeTimeResult {
    /// Quotient of time and divisor
    #[prost(message, optional, tag = "1")]
    pub quotient: ::core::option::Option<super::super::model::v1::NativeTime>,
    /// Remainder of time and divisor
    #[prost(message, optional, tag = "2")]
    pub remainder: ::core::option::Option<super::super::model::v1::NativeTime>,
}
/// Time division checked request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckedDivRemNativeTimeRequest {
    /// Time to divide
    #[prost(message, optional, tag = "1")]
    pub dividend: ::core::option::Option<super::super::model::v1::NativeTime>,
    /// Divisor to divide by
    #[prost(message, optional, tag = "2")]
    pub divisor: ::core::option::Option<super::super::model::v1::NativeTime>,
}
/// Time division checked result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckedDivRemNativeTimeResult {
    /// Quotient of time and divisor
    #[prost(uint64, tag = "1")]
    pub quotient: u64,
    /// Remainder of time and divisor
    #[prost(message, optional, tag = "2")]
    pub remainder: ::core::option::Option<super::super::model::v1::NativeTime>,
}
/// Time comparison request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompareNativeTimeRequest {
    /// First time to compare
    #[prost(message, optional, tag = "1")]
    pub left: ::core::option::Option<super::super::model::v1::NativeTime>,
    /// Second time to compare
    #[prost(message, optional, tag = "2")]
    pub right: ::core::option::Option<super::super::model::v1::NativeTime>,
}
/// Time comparison result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompareNativeTimeResult {
    /// Comparison result
    #[prost(enumeration = "super::super::model::v1::ComparisonResult", tag = "1")]
    pub result: i32,
}
/// Compare Address request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompareAddressRequest {
    /// First address to compare
    #[prost(string, tag = "1")]
    pub left: ::prost::alloc::string::String,
    /// Second address to compare
    #[prost(string, tag = "2")]
    pub right: ::prost::alloc::string::String,
}
/// Compare Address result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompareAddressResult {
    /// Comparison result
    #[prost(enumeration = "super::super::model::v1::ComparisonResult", tag = "1")]
    pub result: i32,
}
/// Compare PubKey request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComparePubKeyRequest {
    /// First public key to compare
    #[prost(string, tag = "1")]
    pub left: ::prost::alloc::string::String,
    /// Second public key to compare
    #[prost(string, tag = "2")]
    pub right: ::prost::alloc::string::String,
}
/// Compare PubKey result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComparePubKeyResult {
    /// Comparison result
    #[prost(enumeration = "super::super::model::v1::ComparisonResult", tag = "1")]
    pub result: i32,
}
/// Verify Sig request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifySigRequest {
    /// Signature to verify
    #[prost(string, tag = "1")]
    pub sig: ::prost::alloc::string::String,
    /// Message to verify
    #[prost(bytes = "vec", tag = "2")]
    pub message: ::prost::alloc::vec::Vec<u8>,
    /// Public key to verify with
    #[prost(string, tag = "3")]
    pub pub_key: ::prost::alloc::string::String,
}
/// Verify Sig result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifySigResult {
    /// Verification result
    #[prost(bool, tag = "1")]
    pub is_verified: bool,
}
/// Compare NativeAmount request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompareNativeAmountRequest {
    /// First amount to compare
    #[prost(message, optional, tag = "1")]
    pub left: ::core::option::Option<super::super::model::v1::NativeAmount>,
    /// Second amount to compare
    #[prost(message, optional, tag = "2")]
    pub right: ::core::option::Option<super::super::model::v1::NativeAmount>,
}
/// Compare NativeAmount result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompareNativeAmountResult {
    /// Comparison result
    #[prost(enumeration = "super::super::model::v1::ComparisonResult", tag = "1")]
    pub result: i32,
}
/// Keccak256 hash request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Keccak256Request {
    /// Data to hash
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Keccak256 hash result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Keccak256Result {
    /// Hash of data
    #[prost(bytes = "vec", tag = "1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
/// EVM signature verification request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvmVerifySigRequest {
    /// Signature to verify
    #[prost(bytes = "vec", tag = "1")]
    pub sig: ::prost::alloc::vec::Vec<u8>,
    /// Message to verify
    #[prost(bytes = "vec", tag = "2")]
    pub message: ::prost::alloc::vec::Vec<u8>,
    /// Public key to verify with
    #[prost(bytes = "vec", tag = "3")]
    pub pub_key: ::prost::alloc::vec::Vec<u8>,
}
/// EVM signature verification result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvmVerifySigResult {
    /// Verification result
    #[prost(bool, tag = "1")]
    pub is_verified: bool,
}
/// EVM get address from public key request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvmGetAddressFromPubkeyRequest {
    /// Public key to get address from
    #[prost(bytes = "vec", tag = "1")]
    pub pub_key: ::prost::alloc::vec::Vec<u8>,
}
/// EVM get address from public key result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvmGetAddressFromPubkeyResult {
    /// Address
    #[prost(bytes = "vec", tag = "1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
}
/// EVM get public key from signature request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvmGetPubkeyFromSignatureRequest {
    /// Signed hash to verify
    #[prost(bytes = "vec", tag = "1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    /// Signature to verify
    #[prost(bytes = "vec", tag = "2")]
    pub sig: ::prost::alloc::vec::Vec<u8>,
}
/// EVM get public key from signature result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvmGetPubkeyFromSignatureResult {
    /// Public key
    #[prost(bytes = "vec", tag = "1")]
    pub pub_key: ::prost::alloc::vec::Vec<u8>,
}
/// Is address EOA request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsAddressEoaRequest {
    /// Address to check
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// Is address EOA result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsAddressEoaResult {
    /// Is address EOA
    #[prost(bool, tag = "1")]
    pub is_eoa: bool,
}
/// Blake3 hash request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HashBlake3Request {
    /// Data to hash
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Blake3 hash result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HashBlake3Result {
    /// Hash of data
    #[prost(bytes = "vec", tag = "1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
/// Get keys request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDsKeysRequest {
    /// keys prefix
    #[prost(bytes = "vec", tag = "1")]
    pub prefix: ::prost::alloc::vec::Vec<u8>,
    /// Address to get keys for
    #[prost(message, optional, tag = "2")]
    pub address: ::core::option::Option<::prost::alloc::string::String>,
}
/// Get keys result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDsKeysResult {
    /// keys
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// Set data store value request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDsValueRequest {
    /// key
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// value
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    /// Address to set data for, if none, use current address
    #[prost(message, optional, tag = "3")]
    pub address: ::core::option::Option<::prost::alloc::string::String>,
}
/// Set data store value result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDsValueResult {}
/// Append data request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppendDsValueRequest {
    /// key
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// value
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    /// Address to append data for, if none, use current address
    #[prost(message, optional, tag = "3")]
    pub address: ::core::option::Option<::prost::alloc::string::String>,
}
/// Append data result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppendDsValueResult {}
/// Get data request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDsValueRequest {
    /// key
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// Address to get data for, if none, use current address
    #[prost(message, optional, tag = "2")]
    pub address: ::core::option::Option<::prost::alloc::string::String>,
}
/// Get data result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDsValueResult {
    /// value
    #[prost(bytes = "vec", tag = "1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// Delete data store entry request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDsEntryRequest {
    /// key
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// Address to delete data for, if none, use current address
    #[prost(message, optional, tag = "2")]
    pub address: ::core::option::Option<::prost::alloc::string::String>,
}
/// Delete data store entry result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDsEntryResult {}
/// Data store entry exists request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DsEntryExistsRequest {
    /// key
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// Address to check data for, if none, use current address
    #[prost(message, optional, tag = "2")]
    pub address: ::core::option::Option<::prost::alloc::string::String>,
}
/// Data store entry exists result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DsEntryExistsResult {
    /// has data
    #[prost(bool, tag = "1")]
    pub has_data: bool,
}
/// Get owned addresses request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOwnedAddressesRequest {}
/// Get owned addresses result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOwnedAddressesResult {
    /// owned addresses
    #[prost(string, repeated, tag = "1")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Get call stack request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCallStackRequest {}
/// Get call stack result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCallStackResult {
    /// call stack
    #[prost(string, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Address from public key request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressFromPubKeyRequest {
    /// Native public key
    #[prost(string, tag = "1")]
    pub pub_key: ::prost::alloc::string::String,
}
/// Address from public key result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressFromPubKeyResult {
    /// Address
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// Unsafe random request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnsafeRandomRequest {
    #[prost(uint32, tag = "2")]
    pub num_bytes: u32,
}
/// Unsafe random result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnsafeRandomResult {
    /// Random bytes generated
    #[prost(bytes = "vec", tag = "1")]
    pub random_bytes: ::prost::alloc::vec::Vec<u8>,
}
/// / Deferred call cancel request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeferredCallCancelRequest {
    /// / call id
    #[prost(message, optional, tag = "1")]
    pub call_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// / Deferred call cancel result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeferredCallCancelResult {}
/// Deferred call quote
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeferredCallQuoteRequest {
    /// / target slot
    #[prost(message, optional, tag = "1")]
    pub target_slot: ::core::option::Option<super::super::model::v1::Slot>,
    /// / max gas requested
    #[prost(uint64, tag = "2")]
    pub max_gas: u64,
    /// / params size in bytes
    #[prost(uint64, tag = "3")]
    pub params_size: u64,
}
/// Deferred call quote result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeferredCallQuoteResult {
    /// / available
    #[prost(bool, tag = "1")]
    pub available: bool,
    /// / cost
    #[prost(uint64, tag = "2")]
    pub cost: u64,
}
/// deferred call exists request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeferredCallExistsRequest {
    /// / deferred call id
    #[prost(message, optional, tag = "1")]
    pub call_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// deferred call exists result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeferredCallExistsResult {
    /// / call exists
    #[prost(bool, tag = "1")]
    pub call_exists: bool,
}
/// deferred call register request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeferredCallRegisterRequest {
    /// / target address
    #[prost(string, tag = "1")]
    pub target_address: ::prost::alloc::string::String,
    /// / target function
    #[prost(string, tag = "2")]
    pub target_function: ::prost::alloc::string::String,
    /// / start slot
    #[prost(message, optional, tag = "3")]
    pub target_slot: ::core::option::Option<super::super::model::v1::Slot>,
    /// / gas requested
    #[prost(uint64, tag = "4")]
    pub max_gas: u64,
    /// / params of the call
    #[prost(bytes = "vec", tag = "5")]
    pub params: ::prost::alloc::vec::Vec<u8>,
    /// / coins
    #[prost(uint64, tag = "6")]
    pub coins: u64,
}
/// / deferred call register result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeferredCallRegisterResult {
    /// / deferred call id
    #[prost(message, optional, tag = "1")]
    pub call_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// Send async message request filter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendAsyncMessageFilter {
    /// Target address
    #[prost(string, tag = "1")]
    pub target_address: ::prost::alloc::string::String,
    /// Target key
    #[prost(message, optional, tag = "2")]
    pub target_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Send async message request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendAsyncMessageRequest {
    /// Target address
    #[prost(string, tag = "1")]
    pub target_address: ::prost::alloc::string::String,
    /// Target handler (function name)
    #[prost(string, tag = "2")]
    pub target_handler: ::prost::alloc::string::String,
    /// Start slot for the message execution
    #[prost(message, optional, tag = "3")]
    pub validity_start: ::core::option::Option<super::super::model::v1::Slot>,
    /// End slot for the message execution
    #[prost(message, optional, tag = "4")]
    pub validity_end: ::core::option::Option<super::super::model::v1::Slot>,
    /// Gas given for the execution
    #[prost(uint64, tag = "5")]
    pub execution_gas: u64,
    /// Message fee
    #[prost(uint64, tag = "6")]
    pub raw_fee: u64,
    /// Coins sent to the execution context
    #[prost(uint64, tag = "7")]
    pub raw_coins: u64,
    /// Message data
    #[prost(bytes = "vec", tag = "8")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// Filter for the message
    #[prost(message, optional, tag = "9")]
    pub filter: ::core::option::Option<SendAsyncMessageFilter>,
}
/// Send async message result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendAsyncMessageResult {}
/// Get origin operation id request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOriginOperationIdRequest {}
/// Get origin operation id result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOriginOperationIdResult {
    #[prost(message, optional, tag = "1")]
    pub operation_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// Get native time request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNativeTimeRequest {}
/// Get native time result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNativeTimeResult {
    /// Native time
    #[prost(message, optional, tag = "1")]
    pub time: ::core::option::Option<super::super::model::v1::NativeTime>,
}
/// Get current slot request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCurrentSlotRequest {}
/// Get current slot result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCurrentSlotResult {
    /// Current slot
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<super::super::model::v1::Slot>,
}
/// Set bytecode request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetBytecodeRequest {
    /// Bytecode
    #[prost(bytes = "vec", tag = "1")]
    pub bytecode: ::prost::alloc::vec::Vec<u8>,
    /// Address to set bytecode for, if none, use current address
    #[prost(message, optional, tag = "2")]
    pub address: ::core::option::Option<::prost::alloc::string::String>,
}
/// Set bytecode result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetBytecodeResult {}
/// Get bytecode request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBytecodeRequest {
    /// Address to get bytecode for, if none, use current address
    #[prost(message, optional, tag = "1")]
    pub address: ::core::option::Option<::prost::alloc::string::String>,
}
/// Get bytecode result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBytecodeResult {
    /// Bytecode
    #[prost(bytes = "vec", tag = "1")]
    pub bytecode: ::prost::alloc::vec::Vec<u8>,
}
/// Caller has write access request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallerHasWriteAccessRequest {}
/// Caller has write access result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallerHasWriteAccessResult {
    /// Caller has write access
    #[prost(bool, tag = "1")]
    pub has_write_access: bool,
}
/// Hash sha256 request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HashSha256Request {
    /// data
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Hash sha256 result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HashSha256Result {
    /// hash
    #[prost(bytes = "vec", tag = "1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
/// Get op data request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOpDataRequest {
    /// key
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
/// Get op data result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOpDataResult {
    /// value
    #[prost(bytes = "vec", tag = "1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// Op entry exists request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpEntryExistsRequest {
    /// key
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
/// Op entry exists result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpEntryExistsResult {
    /// has key
    #[prost(bool, tag = "1")]
    pub has_key: bool,
}
/// Get op keys request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOpKeysRequest {
    /// keys prefix
    #[prost(bytes = "vec", tag = "1")]
    pub prefix: ::prost::alloc::vec::Vec<u8>,
}
/// Get op keys result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOpKeysResult {
    /// keys
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// Get remaining gas request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRemainingGasRequest {}
/// Get remaining gas result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRemainingGasResult {
    /// remaining gas
    #[prost(uint64, tag = "1")]
    pub remaining_gas: u64,
}
/// Get balance request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBalanceRequest {
    /// Address to get balance for, if none, use current address
    #[prost(message, optional, tag = "1")]
    pub address: ::core::option::Option<::prost::alloc::string::String>,
}
/// Get balance result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBalanceResult {
    /// balance
    #[prost(message, optional, tag = "1")]
    pub balance: ::core::option::Option<super::super::model::v1::NativeAmount>,
}
/// Get call coins request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCallCoinsRequest {}
/// Get call coins result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCallCoinsResult {
    /// coins
    #[prost(message, optional, tag = "1")]
    pub coins: ::core::option::Option<super::super::model::v1::NativeAmount>,
}
/// Get the version of the address request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAddressVersionRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// Get the version of the address result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAddressVersionResult {
    #[prost(uint64, tag = "1")]
    pub version: u64,
}
/// Get the category of the address request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAddressCategoryRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// Get the category of the address result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAddressCategoryResult {
    #[prost(enumeration = "super::super::model::v1::AddressCategory", tag = "1")]
    pub category: i32,
}
/// Get the version of the public key request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPubKeyVersionRequest {
    #[prost(string, tag = "1")]
    pub pub_key: ::prost::alloc::string::String,
}
/// Get the version of the public key result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPubKeyVersionResult {
    #[prost(uint64, tag = "1")]
    pub version: u64,
}
/// Get the version of the signature request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSignatureVersionRequest {
    #[prost(string, tag = "1")]
    pub signature: ::prost::alloc::string::String,
}
/// Get the version of the signature result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSignatureVersionResult {
    #[prost(uint64, tag = "1")]
    pub version: u64,
}
/// bytes to base58 request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BytesToBase58CheckRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub bytes: ::prost::alloc::vec::Vec<u8>,
}
/// bytes to base58 result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BytesToBase58CheckResult {
    #[prost(string, tag = "1")]
    pub base58_check: ::prost::alloc::string::String,
}
/// base58 to bytes request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Base58CheckToBytesRequest {
    #[prost(string, tag = "1")]
    pub base58_check: ::prost::alloc::string::String,
}
/// base58 to bytes result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Base58CheckToBytesResult {
    #[prost(bytes = "vec", tag = "1")]
    pub bytes: ::prost::alloc::vec::Vec<u8>,
}
/// Chain id request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChainIdRequest {}
/// Chain id result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChainIdResult {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
