/// Massa NativeAddress
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeAddress {
    /// Address category
    #[prost(enumeration = "AddressCategory", tag = "1")]
    pub category: i32,
    /// Address version
    #[prost(uint64, tag = "2")]
    pub version: u64,
    /// Address content
    #[prost(bytes = "vec", tag = "3")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
/// Addresses holds addresses
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Addresses {
    /// Addresses
    #[prost(string, repeated, tag = "1")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Address category discriminant
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AddressCategory {
    /// Unspecified address category
    Unspecified = 0,
    /// User address
    UserAddress = 1,
    /// Smart contract address
    ScAddress = 2,
}
impl AddressCategory {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AddressCategory::Unspecified => "ADDRESS_CATEGORY_UNSPECIFIED",
            AddressCategory::UserAddress => "ADDRESS_CATEGORY_USER_ADDRESS",
            AddressCategory::ScAddress => "ADDRESS_CATEGORY_SC_ADDRESS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ADDRESS_CATEGORY_UNSPECIFIED" => Some(Self::Unspecified),
            "ADDRESS_CATEGORY_USER_ADDRESS" => Some(Self::UserAddress),
            "ADDRESS_CATEGORY_SC_ADDRESS" => Some(Self::ScAddress),
            _ => None,
        }
    }
}
/// NativeAmount is represented as a fraction so precision can be adjusted in
/// the future. value = mantissa / (10^scale)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeAmount {
    /// Mantissa
    #[prost(uint64, tag = "1")]
    pub mantissa: u64,
    /// Scale
    #[prost(uint32, tag = "2")]
    pub scale: u32,
}
/// When an address is drawn to create an endorsement it is selected for a specific index
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexedSlot {
    /// Slot
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<Slot>,
    /// Endorsement index in the slot
    #[prost(uint64, tag = "2")]
    pub index: u64,
}
/// A point in time where a block is expected
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Slot {
    /// Period
    #[prost(uint64, tag = "1")]
    pub period: u64,
    /// Thread
    #[prost(uint32, tag = "2")]
    pub thread: u32,
}
/// Slots
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Slots {
    /// Slots
    #[prost(message, repeated, tag = "1")]
    pub slots: ::prost::alloc::vec::Vec<Slot>,
}
/// SlotRange
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SlotRange {
    /// Start lot (Optional)
    #[prost(message, optional, tag = "1")]
    pub start_slot: ::core::option::Option<Slot>,
    /// End slot (Optional)
    #[prost(message, optional, tag = "2")]
    pub end_slot: ::core::option::Option<Slot>,
}
/// An endorsement, as sent in the network
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Endorsement {
    /// Slot in which the endorsement can be included
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<Slot>,
    /// Endorsement index inside the including block
    #[prost(uint32, tag = "2")]
    pub index: u32,
    /// Hash of endorsed block
    /// This is the parent in thread `self.slot.thread` of the block in which the endorsement is included
    #[prost(string, tag = "3")]
    pub endorsed_block: ::prost::alloc::string::String,
}
/// Signed endorsement
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedEndorsement {
    /// Endorsement
    #[prost(message, optional, tag = "1")]
    pub content: ::core::option::Option<Endorsement>,
    /// A cryptographically generated value using `serialized_data` and a public key.
    #[prost(string, tag = "2")]
    pub signature: ::prost::alloc::string::String,
    /// The public-key component used in the generation of the signature
    #[prost(string, tag = "3")]
    pub content_creator_pub_key: ::prost::alloc::string::String,
    /// Derived from the same public key used to generate the signature
    #[prost(string, tag = "4")]
    pub content_creator_address: ::prost::alloc::string::String,
    /// A secure hash of the non-malleable contents of a deterministic binary representation of the block header
    #[prost(string, tag = "5")]
    pub secure_hash: ::prost::alloc::string::String,
    /// The size of the serialized endorsement in bytes
    #[prost(uint64, tag = "6")]
    pub serialized_size: u64,
}
/// EndorsementIds holds endorsements ids
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndorsementIds {
    /// Endorsements ids
    #[prost(string, repeated, tag = "1")]
    pub endorsement_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A wrapper around an endorsement with its metadata
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndorsementWrapper {
    /// Whether the endorsement is still in pool
    #[prost(bool, tag = "1")]
    pub in_pool: bool,
    /// The endorsement appears in `in_blocks`
    /// If it appears in multiple blocks, these blocks are in different cliques
    #[prost(string, repeated, tag = "2")]
    pub in_blocks: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Whether the the endorsement is final (for example in a final block)
    #[prost(bool, tag = "3")]
    pub is_final: bool,
    /// The endorsement itself
    #[prost(message, optional, tag = "4")]
    pub endorsement: ::core::option::Option<SignedEndorsement>,
}
/// Informations about an endorsement with its metadata
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndorsementInfo {
    /// The endorsement id
    #[prost(string, tag = "1")]
    pub endorsement_id: ::prost::alloc::string::String,
    /// Whether the endorsement is still in pool
    #[prost(bool, tag = "2")]
    pub in_pool: bool,
    /// The endorsement appears in `in_blocks`
    /// If it appears in multiple blocks, these blocks are in different cliques
    #[prost(string, repeated, tag = "3")]
    pub in_blocks: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Whether the the endorsement is final (for example in a final block)
    #[prost(bool, tag = "4")]
    pub is_final: bool,
}
/// Massa error
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    /// The error code
    #[prost(int32, tag = "1")]
    pub code: i32,
    /// A developer-facing error message, which should be in English
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
/// Empty
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {}
/// BytesMapFieldEntry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BytesMapFieldEntry {
    /// bytes key
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// bytes key
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// Array of bytes wrapper
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArrayOfBytesWrapper {
    /// Repeated bytes
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// KeyPair
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyPair {
    /// Public key
    #[prost(string, tag = "1")]
    pub public_key: ::prost::alloc::string::String,
    /// Secret key
    #[prost(string, tag = "2")]
    pub secret_key: ::prost::alloc::string::String,
}
/// Comparison result
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ComparisonResult {
    Unspecified = 0,
    /// left is lower
    Lower = 1,
    /// left and right are equal
    Equal = 2,
    /// left is greater
    Greater = 3,
}
impl ComparisonResult {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ComparisonResult::Unspecified => "COMPARISON_RESULT_UNSPECIFIED",
            ComparisonResult::Lower => "COMPARISON_RESULT_LOWER",
            ComparisonResult::Equal => "COMPARISON_RESULT_EQUAL",
            ComparisonResult::Greater => "COMPARISON_RESULT_GREATER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "COMPARISON_RESULT_UNSPECIFIED" => Some(Self::Unspecified),
            "COMPARISON_RESULT_LOWER" => Some(Self::Lower),
            "COMPARISON_RESULT_EQUAL" => Some(Self::Equal),
            "COMPARISON_RESULT_GREATER" => Some(Self::Greater),
            _ => None,
        }
    }
}
/// The operation as sent in the network
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
    /// The fee they have decided for this operation
    #[prost(message, optional, tag = "1")]
    pub fee: ::core::option::Option<NativeAmount>,
    /// After `expire_period` slot the operation won't be included in a block
    #[prost(uint64, tag = "2")]
    pub expire_period: u64,
    /// The type specific operation part
    #[prost(message, optional, tag = "3")]
    pub op: ::core::option::Option<OperationType>,
}
/// Type specific operation content
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationType {
    #[prost(oneof = "operation_type::Type", tags = "1, 2, 3, 4, 5")]
    pub r#type: ::core::option::Option<operation_type::Type>,
}
/// Nested message and enum types in `OperationType`.
pub mod operation_type {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// Transfer coins from sender to recipient
        #[prost(message, tag = "1")]
        Transaction(super::Transaction),
        /// The sender buys `roll_count` rolls. Roll price is defined in configuration
        #[prost(message, tag = "2")]
        RollBuy(super::RollBuy),
        /// The sender sells `roll_count` rolls. Roll price is defined in configuration
        #[prost(message, tag = "3")]
        RollSell(super::RollSell),
        /// Execute a smart contract
        #[prost(message, tag = "4")]
        ExecutSc(super::ExecuteSc),
        /// Calls an exported function from a stored smart contract
        #[prost(message, tag = "5")]
        CallSc(super::CallSc),
    }
}
/// Transfer coins from sender to recipient
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    /// Recipient address
    #[prost(string, tag = "1")]
    pub recipient_address: ::prost::alloc::string::String,
    /// Amount
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<NativeAmount>,
}
/// The sender buys `roll_count` rolls. Roll price is defined in configuration
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollBuy {
    /// Roll count
    #[prost(uint64, tag = "1")]
    pub roll_count: u64,
}
/// The sender sells `roll_count` rolls. Roll price is defined in configuration
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollSell {
    /// Roll count
    #[prost(uint64, tag = "1")]
    pub roll_count: u64,
}
/// Execute a smart contract
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteSc {
    /// Smart contract bytecode.
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// The maximum of coins that could be spent by the operation sender
    #[prost(uint64, tag = "2")]
    pub max_coins: u64,
    /// The maximum amount of gas that the execution of the contract is allowed to cost
    #[prost(uint64, tag = "3")]
    pub max_gas: u64,
    /// A key-value store associating a hash to arbitrary bytes
    #[prost(message, repeated, tag = "4")]
    pub datastore: ::prost::alloc::vec::Vec<BytesMapFieldEntry>,
}
/// Calls an exported function from a stored smart contract
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallSc {
    /// Target smart contract address
    #[prost(string, tag = "1")]
    pub target_address: ::prost::alloc::string::String,
    /// Target function name. No function is called if empty
    #[prost(string, tag = "2")]
    pub target_function: ::prost::alloc::string::String,
    /// Parameter to pass to the target function
    #[prost(bytes = "vec", tag = "3")]
    pub parameter: ::prost::alloc::vec::Vec<u8>,
    /// The maximum amount of gas that the execution of the contract is allowed to cost
    #[prost(uint64, tag = "4")]
    pub max_gas: u64,
    /// Extra coins that are spent from the caller's balance and transferred to the target
    #[prost(message, optional, tag = "5")]
    pub coins: ::core::option::Option<NativeAmount>,
}
/// Signed operation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedOperation {
    /// Operation
    #[prost(message, optional, tag = "1")]
    pub content: ::core::option::Option<Operation>,
    /// A cryptographically generated value using `serialized_data` and a public key.
    #[prost(string, tag = "2")]
    pub signature: ::prost::alloc::string::String,
    /// The public-key component used in the generation of the signature
    #[prost(string, tag = "3")]
    pub content_creator_pub_key: ::prost::alloc::string::String,
    /// Derived from the same public key used to generate the signature
    #[prost(string, tag = "4")]
    pub content_creator_address: ::prost::alloc::string::String,
    /// A secure hash of the non-malleable contents of a deterministic binary representation of the block header
    #[prost(string, tag = "5")]
    pub secure_hash: ::prost::alloc::string::String,
    /// The size of the serialized operation in bytes
    #[prost(uint64, tag = "6")]
    pub serialized_size: u64,
}
/// A wrapper around an operation with its metadata
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationWrapper {
    /// The IDs of the blocks in which the operation appears
    #[prost(string, repeated, tag = "1")]
    pub block_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The thread in which the operation can be included
    #[prost(uint32, tag = "2")]
    pub thread: u32,
    /// The operation object itself
    #[prost(message, optional, tag = "3")]
    pub operation: ::core::option::Option<SignedOperation>,
}
/// Information about an operation with its metadata
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationInfo {
    /// The unique ID of the operation.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The IDs of the blocks in which the operation appears
    #[prost(string, repeated, tag = "2")]
    pub block_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The thread in which the operation can be included
    #[prost(uint32, tag = "3")]
    pub thread: u32,
}
/// OperationIds
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationIds {
    /// Operations ids
    #[prost(string, repeated, tag = "1")]
    pub operation_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// OpTypes
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpTypes {
    /// Operations types
    #[prost(enumeration = "OpType", repeated, tag = "1")]
    pub op_types: ::prost::alloc::vec::Vec<i32>,
}
/// Operation type enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OpType {
    /// Default enum value
    Unspecified = 0,
    /// Transaction
    Transaction = 1,
    /// Roll buy
    RollBuy = 2,
    /// Roll sell
    RollSell = 3,
    /// Execute smart contract
    ExecuteSc = 4,
    /// Call smart contract
    CallSc = 5,
}
impl OpType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OpType::Unspecified => "OP_TYPE_UNSPECIFIED",
            OpType::Transaction => "OP_TYPE_TRANSACTION",
            OpType::RollBuy => "OP_TYPE_ROLL_BUY",
            OpType::RollSell => "OP_TYPE_ROLL_SELL",
            OpType::ExecuteSc => "OP_TYPE_EXECUTE_SC",
            OpType::CallSc => "OP_TYPE_CALL_SC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OP_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "OP_TYPE_TRANSACTION" => Some(Self::Transaction),
            "OP_TYPE_ROLL_BUY" => Some(Self::RollBuy),
            "OP_TYPE_ROLL_SELL" => Some(Self::RollSell),
            "OP_TYPE_EXECUTE_SC" => Some(Self::ExecuteSc),
            "OP_TYPE_CALL_SC" => Some(Self::CallSc),
            _ => None,
        }
    }
}
/// Block
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    /// Signed header
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<SignedBlockHeader>,
    /// Operations ids
    #[prost(string, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Filled block
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilledBlock {
    /// Signed header
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<SignedBlockHeader>,
    /// Operations
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<FilledOperationEntry>,
}
/// Block header
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockHeader {
    /// Current network version
    #[prost(uint32, tag = "1")]
    pub current_version: u32,
    /// Announced network version(Optional)
    #[prost(message, optional, tag = "2")]
    pub announced_version: ::core::option::Option<u32>,
    /// Slot
    #[prost(message, optional, tag = "3")]
    pub slot: ::core::option::Option<Slot>,
    /// parents
    #[prost(string, repeated, tag = "4")]
    pub parents: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// All operations hash
    #[prost(string, tag = "5")]
    pub operations_hash: ::prost::alloc::string::String,
    /// Signed endorsements
    #[prost(message, repeated, tag = "6")]
    pub endorsements: ::prost::alloc::vec::Vec<SignedEndorsement>,
}
/// Filled Operation Tuple
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilledOperationEntry {
    /// Operation id
    #[prost(string, tag = "1")]
    pub operation_id: ::prost::alloc::string::String,
    /// Signed operation
    #[prost(message, optional, tag = "2")]
    pub operation: ::core::option::Option<SignedOperation>,
}
/// Signed block
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedBlock {
    /// Block
    #[prost(message, optional, tag = "1")]
    pub content: ::core::option::Option<Block>,
    /// A cryptographically generated value using `serialized_data` and a public key.
    #[prost(string, tag = "2")]
    pub signature: ::prost::alloc::string::String,
    /// The public-key component used in the generation of the signature
    #[prost(string, tag = "3")]
    pub content_creator_pub_key: ::prost::alloc::string::String,
    /// Derived from the same public key used to generate the signature
    #[prost(string, tag = "4")]
    pub content_creator_address: ::prost::alloc::string::String,
    /// A secure hash of the non-malleable contents of a deterministic binary representation of the block header
    #[prost(string, tag = "5")]
    pub secure_hash: ::prost::alloc::string::String,
    /// The size of the serialized block in bytes
    #[prost(uint64, tag = "6")]
    pub serialized_size: u64,
}
/// Signed block header
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedBlockHeader {
    /// BlockHeader
    #[prost(message, optional, tag = "1")]
    pub content: ::core::option::Option<BlockHeader>,
    /// A cryptographically generated value using `serialized_data` and a public key.
    #[prost(string, tag = "2")]
    pub signature: ::prost::alloc::string::String,
    /// The public-key component used in the generation of the signature
    #[prost(string, tag = "3")]
    pub content_creator_pub_key: ::prost::alloc::string::String,
    /// Derived from the same public key used to generate the signature
    #[prost(string, tag = "4")]
    pub content_creator_address: ::prost::alloc::string::String,
    /// A secure hash of the non-malleable contents of a deterministic binary representation of the block header
    #[prost(string, tag = "5")]
    pub secure_hash: ::prost::alloc::string::String,
    /// The size of the serialized block header in bytes
    #[prost(uint64, tag = "6")]
    pub serialized_size: u64,
}
/// A wrapper around a block with its metadata
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockWrapper {
    /// The execution status of the block
    #[prost(enumeration = "BlockStatus", tag = "1")]
    pub status: i32,
    /// The block object itself
    #[prost(message, optional, tag = "2")]
    pub block: ::core::option::Option<Block>,
}
/// Informations about a block with its metadata
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockInfo {
    /// The unique ID of the block.
    #[prost(string, tag = "1")]
    pub block_id: ::prost::alloc::string::String,
    /// The execution status of the block
    #[prost(enumeration = "BlockStatus", tag = "2")]
    pub status: i32,
}
/// BlockIds holds block ids
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockIds {
    /// Block ids
    #[prost(string, repeated, tag = "1")]
    pub block_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Block parent tuple
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockParent {
    /// Block id
    #[prost(string, tag = "1")]
    pub block_id: ::prost::alloc::string::String,
    /// Period
    #[prost(uint64, tag = "2")]
    pub period: u64,
}
/// Possible statuses for a block
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BlockStatus {
    /// Default enum value
    Unspecified = 0,
    /// The block is in the greatest clique (and not final)
    NonFinalBlockclique = 1,
    /// The block is final
    Final = 2,
    /// The block is candidate (active any clique but not final)
    NonFinalAlternateClique = 3,
    /// The block is discarded
    Discarded = 4,
}
impl BlockStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BlockStatus::Unspecified => "BLOCK_STATUS_UNSPECIFIED",
            BlockStatus::NonFinalBlockclique => "BLOCK_STATUS_NON_FINAL_BLOCKCLIQUE",
            BlockStatus::Final => "BLOCK_STATUS_FINAL",
            BlockStatus::NonFinalAlternateClique => {
                "BLOCK_STATUS_NON_FINAL_ALTERNATE_CLIQUE"
            }
            BlockStatus::Discarded => "BLOCK_STATUS_DISCARDED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BLOCK_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "BLOCK_STATUS_NON_FINAL_BLOCKCLIQUE" => Some(Self::NonFinalBlockclique),
            "BLOCK_STATUS_FINAL" => Some(Self::Final),
            "BLOCK_STATUS_NON_FINAL_ALTERNATE_CLIQUE" => {
                Some(Self::NonFinalAlternateClique)
            }
            "BLOCK_STATUS_DISCARDED" => Some(Self::Discarded),
            _ => None,
        }
    }
}
/// AddressKeys holds a list of addresses - keys
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressKeysEntries {
    /// List of address- key entries
    #[prost(message, repeated, tag = "1")]
    pub address_key_entries: ::prost::alloc::vec::Vec<AddressKeyEntry>,
}
/// AddressKeyEntry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressKeyEntry {
    /// Associated address of the entry
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Datastore key
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
/// DatastoreEntry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatastoreEntry {
    /// final datastore entry value
    #[prost(bytes = "vec", tag = "1")]
    pub final_value: ::prost::alloc::vec::Vec<u8>,
    /// candidate_value datastore entry value
    #[prost(bytes = "vec", tag = "2")]
    pub candidate_value: ::prost::alloc::vec::Vec<u8>,
}
/// Index for Denunciations in collections (e.g. like a HashMap...)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenunciationIndex {
    /// DenunciationBlockHeader or DenunciationEndorsement
    #[prost(oneof = "denunciation_index::Entry", tags = "1, 2")]
    pub entry: ::core::option::Option<denunciation_index::Entry>,
}
/// Nested message and enum types in `DenunciationIndex`.
pub mod denunciation_index {
    /// DenunciationBlockHeader or DenunciationEndorsement
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Entry {
        /// Denunciation block header
        #[prost(message, tag = "1")]
        BlockHeader(super::DenunciationBlockHeader),
        /// Denunciation endorsement
        #[prost(message, tag = "2")]
        Endorsement(super::DenunciationEndorsement),
    }
}
/// Variant for Block header denunciation index
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenunciationBlockHeader {
    /// Denounciation slot
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<Slot>,
}
/// Variant for Endorsement denunciation index
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenunciationEndorsement {
    /// Denounciation slot
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<Slot>,
    /// Denounciation index
    #[prost(uint32, tag = "2")]
    pub index: u32,
}
/// Slot draw
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SlotDraw {
    /// Slot
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<Slot>,
    /// Block producer address (Optional)
    #[prost(message, optional, tag = "2")]
    pub block_producer: ::core::option::Option<::prost::alloc::string::String>,
    /// Endorsement draws
    #[prost(message, repeated, tag = "3")]
    pub endorsement_draws: ::prost::alloc::vec::Vec<EndorsementDraw>,
}
/// Endorsement draw
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndorsementDraw {
    /// Endorsement index
    #[prost(uint64, tag = "1")]
    pub index: u64,
    /// Producer address
    #[prost(string, tag = "2")]
    pub producer: ::prost::alloc::string::String,
}
/// SlotExecutionOutput
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SlotExecutionOutput {
    /// Status
    #[prost(enumeration = "ExecutionOutputStatus", tag = "1")]
    pub status: i32,
    /// Executed slot output
    #[prost(message, optional, tag = "2")]
    pub execution_output: ::core::option::Option<ExecutionOutput>,
}
/// FinalizedExecutionOutput
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizedExecutionOutput {
    /// Slot
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<Slot>,
}
/// ExecutionOutput
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionOutput {
    /// Slot
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<Slot>,
    /// Block id at that slot (Optional)
    #[prost(message, optional, tag = "2")]
    pub block_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Events emitted by the execution step
    #[prost(message, repeated, tag = "3")]
    pub events: ::prost::alloc::vec::Vec<ScExecutionEvent>,
    /// State changes caused by the execution step
    #[prost(message, optional, tag = "4")]
    pub state_changes: ::core::option::Option<StateChanges>,
}
/// ScExecutionEvent
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScExecutionEvent {
    /// Sc execution context
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<ScExecutionEventContext>,
    /// Generated data of the event
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// ScExecutionEvent context
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScExecutionEventContext {
    /// When was it generated
    #[prost(message, optional, tag = "1")]
    pub origin_slot: ::core::option::Option<Slot>,
    /// Block id if there was a block at that slot (Optional)
    #[prost(message, optional, tag = "2")]
    pub block_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Index of the event in the slot
    #[prost(uint64, tag = "3")]
    pub index_in_slot: u64,
    /// Call stack addresses. most recent at the end
    #[prost(string, repeated, tag = "4")]
    pub call_stack: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Origin operation id (Optional)
    #[prost(message, optional, tag = "5")]
    pub origin_operation_id: ::core::option::Option<::prost::alloc::string::String>,
    /// If a failure occurred
    #[prost(bool, tag = "6")]
    pub is_failure: bool,
    /// Status
    #[prost(enumeration = "ScExecutionEventStatus", tag = "7")]
    pub status: i32,
}
/// ScExecutionEventsStatus
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScExecutionEventsStatus {
    /// Status
    #[prost(enumeration = "ScExecutionEventStatus", repeated, tag = "1")]
    pub status: ::prost::alloc::vec::Vec<i32>,
}
/// StateChanges
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateChanges {
    /// Ledger changes
    #[prost(message, repeated, tag = "1")]
    pub ledger_changes: ::prost::alloc::vec::Vec<LedgerChangeEntry>,
    /// Asynchronous pool changes
    #[prost(message, repeated, tag = "2")]
    pub async_pool_changes: ::prost::alloc::vec::Vec<AsyncPoolChangeEntry>,
    /// Executed operations changes
    #[prost(message, repeated, tag = "4")]
    pub executed_ops_changes: ::prost::alloc::vec::Vec<ExecutedOpsChangeEntry>,
    /// Executed denunciations changes
    #[prost(message, repeated, tag = "5")]
    pub executed_denunciations_changes: ::prost::alloc::vec::Vec<DenunciationIndex>,
    /// Execution trail hash change
    #[prost(message, optional, tag = "6")]
    pub execution_trail_hash_change: ::core::option::Option<SetOrKeepString>,
}
/// ExecutedOpsChangeEntry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutedOpsChangeEntry {
    /// string
    #[prost(string, tag = "1")]
    pub operation_id: ::prost::alloc::string::String,
    /// ExecutedOpsChangeValue
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<ExecutedOpsChangeValue>,
}
/// ExecutedOpsChangeValue
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutedOpsChangeValue {
    /// The status of the execution of the operation
    #[prost(enumeration = "OperationExecutionStatus", tag = "1")]
    pub status: i32,
    /// Slot until which the operation remains valid (included)
    #[prost(message, optional, tag = "2")]
    pub slot: ::core::option::Option<Slot>,
}
/// AsyncPoolChange Entry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncPoolChangeEntry {
    /// Async message id
    #[prost(string, tag = "1")]
    pub async_message_id: ::prost::alloc::string::String,
    /// AsyncPool message
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<AsyncPoolChangeValue>,
}
/// AsyncPoolChangeValue
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncPoolChangeValue {
    /// The type of the change
    #[prost(enumeration = "AsyncPoolChangeType", tag = "1")]
    pub r#type: i32,
    /// AsyncPool message
    #[prost(oneof = "async_pool_change_value::Message", tags = "2, 3")]
    pub message: ::core::option::Option<async_pool_change_value::Message>,
}
/// Nested message and enum types in `AsyncPoolChangeValue`.
pub mod async_pool_change_value {
    /// AsyncPool message
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// Created ledger entry
        #[prost(message, tag = "2")]
        CreatedMessage(super::AsyncMessage),
        /// Update ledger entry
        #[prost(message, tag = "3")]
        UpdatedMessage(super::AsyncMessageUpdate),
    }
}
/// Asynchronous smart contract message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncMessage {
    /// Slot at which the message was emitted
    #[prost(message, optional, tag = "1")]
    pub emission_slot: ::core::option::Option<Slot>,
    /// Index of the emitted message within the `emission_slot`.
    /// This is used for disambiguate the emission of multiple messages at the same slot.
    #[prost(uint64, tag = "2")]
    pub emission_index: u64,
    /// The address that sent the message
    #[prost(string, tag = "3")]
    pub sender: ::prost::alloc::string::String,
    /// The address towards which the message is being sent
    #[prost(string, tag = "4")]
    pub destination: ::prost::alloc::string::String,
    /// the handler function name within the destination address' bytecode
    #[prost(string, tag = "5")]
    pub handler: ::prost::alloc::string::String,
    /// Maximum gas to use when processing the message
    #[prost(uint64, tag = "6")]
    pub max_gas: u64,
    /// Fee paid by the sender when the message is processed.
    #[prost(message, optional, tag = "7")]
    pub fee: ::core::option::Option<NativeAmount>,
    /// Coins sent from the sender to the target address of the message.
    /// Those coins are spent by the sender address when the message is sent,
    /// and credited to the destination address when receiving the message.
    /// In case of failure or discard, those coins are reimbursed to the sender.
    #[prost(message, optional, tag = "8")]
    pub coins: ::core::option::Option<NativeAmount>,
    /// Slot at which the message starts being valid (bound included in the validity range)
    #[prost(message, optional, tag = "9")]
    pub validity_start: ::core::option::Option<Slot>,
    /// Slot at which the message stops being valid (bound not included in the validity range)
    #[prost(message, optional, tag = "10")]
    pub validity_end: ::core::option::Option<Slot>,
    /// Raw payload data of the message
    #[prost(bytes = "vec", tag = "11")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// Trigger that define whenever a message can be executed
    #[prost(message, optional, tag = "12")]
    pub trigger: ::core::option::Option<AsyncMessageTrigger>,
    /// Boolean that determine if the message can be executed. For messages without filter this boolean is always true.
    /// For messages with filter, this boolean is true if the filter has been matched between `validity_start` and current slot.
    #[prost(bool, tag = "13")]
    pub can_be_executed: bool,
}
/// Asynchronous smart contract message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncMessageUpdate {
    /// Change the slot at which the message was emitted
    #[prost(message, optional, tag = "1")]
    pub emission_slot: ::core::option::Option<SetOrKeepSlot>,
    /// Change the index of the emitted message within the `emission_slot`.
    /// This is used for disambiguate the emission of multiple messages at the same slot.
    #[prost(message, optional, tag = "2")]
    pub emission_index: ::core::option::Option<SetOrKeepUint64>,
    /// Change the address that sent the message
    #[prost(message, optional, tag = "3")]
    pub sender: ::core::option::Option<SetOrKeepString>,
    /// Change the address towards which the message is being sent
    #[prost(message, optional, tag = "4")]
    pub destination: ::core::option::Option<SetOrKeepString>,
    /// Change the handler function name within the destination address' bytecode
    #[prost(message, optional, tag = "5")]
    pub handler: ::core::option::Option<SetOrKeepString>,
    /// Change the maximum gas to use when processing the message
    #[prost(message, optional, tag = "6")]
    pub max_gas: ::core::option::Option<SetOrKeepUint64>,
    /// Change the fee paid by the sender when the message is processed.
    #[prost(message, optional, tag = "7")]
    pub fee: ::core::option::Option<SetOrKeepUint64>,
    /// Change the coins sent from the sender to the target address of the message.
    /// Those coins are spent by the sender address when the message is sent,
    /// and credited to the destination address when receiving the message.
    /// In case of failure or discard, those coins are reimbursed to the sender.
    #[prost(message, optional, tag = "8")]
    pub coins: ::core::option::Option<SetOrKeepUint64>,
    /// Change the slot at which the message starts being valid (bound included in the validity range)
    #[prost(message, optional, tag = "9")]
    pub validity_start: ::core::option::Option<SetOrKeepSlot>,
    /// Change the slot at which the message stops being valid (bound not included in the validity range)
    #[prost(message, optional, tag = "10")]
    pub validity_end: ::core::option::Option<SetOrKeepSlot>,
    /// Change the raw payload data of the message
    #[prost(message, optional, tag = "11")]
    pub data: ::core::option::Option<SetOrKeepBytes>,
    /// Change the trigger that define whenever a message can be executed
    #[prost(message, optional, tag = "12")]
    pub trigger: ::core::option::Option<SetOrKeepAsyncMessageTrigger>,
    /// Change the boolean that determine if the message can be executed. For messages without filter this boolean is always true.
    /// For messages with filter, this boolean is true if the filter has been matched between `validity_start` and current slot.
    #[prost(message, optional, tag = "13")]
    pub can_be_executed: ::core::option::Option<SetOrKeepBool>,
}
/// Set or Keep Slot
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrKeepSlot {
    /// The type of the change
    #[prost(oneof = "set_or_keep_slot::Change", tags = "1, 2")]
    pub change: ::core::option::Option<set_or_keep_slot::Change>,
}
/// Nested message and enum types in `SetOrKeepSlot`.
pub mod set_or_keep_slot {
    /// The type of the change
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Change {
        /// The value of that entry (Optional)
        #[prost(message, tag = "1")]
        Set(super::Slot),
        /// Keep the existing value
        #[prost(message, tag = "2")]
        Keep(super::Empty),
    }
}
/// Set or Keep Uint64
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrKeepUint64 {
    /// The type of the change
    #[prost(oneof = "set_or_keep_uint64::Change", tags = "1, 2")]
    pub change: ::core::option::Option<set_or_keep_uint64::Change>,
}
/// Nested message and enum types in `SetOrKeepUint64`.
pub mod set_or_keep_uint64 {
    /// The type of the change
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Change {
        /// The value of that entry (Optional)
        #[prost(message, tag = "1")]
        Set(u64),
        /// Keep the existing value
        #[prost(message, tag = "2")]
        Keep(super::Empty),
    }
}
/// Set or Keep String
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrKeepString {
    /// The type of the change
    #[prost(oneof = "set_or_keep_string::Change", tags = "1, 2")]
    pub change: ::core::option::Option<set_or_keep_string::Change>,
}
/// Nested message and enum types in `SetOrKeepString`.
pub mod set_or_keep_string {
    /// The type of the change
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Change {
        /// The value of that entry (Optional)
        #[prost(message, tag = "1")]
        Set(::prost::alloc::string::String),
        /// Keep the existing value
        #[prost(message, tag = "2")]
        Keep(super::Empty),
    }
}
/// Set or Keep Bytes
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrKeepBytes {
    /// The type of the change
    #[prost(oneof = "set_or_keep_bytes::Change", tags = "1, 2")]
    pub change: ::core::option::Option<set_or_keep_bytes::Change>,
}
/// Nested message and enum types in `SetOrKeepBytes`.
pub mod set_or_keep_bytes {
    /// The type of the change
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Change {
        /// The value of that entry (Optional)
        #[prost(message, tag = "1")]
        Set(::prost::alloc::vec::Vec<u8>),
        /// Keep the existing value
        #[prost(message, tag = "2")]
        Keep(super::Empty),
    }
}
/// Set or Keep Bool
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrKeepBool {
    /// The type of the change
    #[prost(oneof = "set_or_keep_bool::Change", tags = "1, 2")]
    pub change: ::core::option::Option<set_or_keep_bool::Change>,
}
/// Nested message and enum types in `SetOrKeepBool`.
pub mod set_or_keep_bool {
    /// The type of the change
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Change {
        /// The value of that entry (Optional)
        #[prost(message, tag = "1")]
        Set(bool),
        /// Keep the existing value
        #[prost(message, tag = "2")]
        Keep(super::Empty),
    }
}
/// Set or Keep AsyncMessageTrigger
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrKeepAsyncMessageTrigger {
    /// The type of the change
    #[prost(oneof = "set_or_keep_async_message_trigger::Change", tags = "1, 2")]
    pub change: ::core::option::Option<set_or_keep_async_message_trigger::Change>,
}
/// Nested message and enum types in `SetOrKeepAsyncMessageTrigger`.
pub mod set_or_keep_async_message_trigger {
    /// The type of the change
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Change {
        /// The value of that entry (Optional)
        #[prost(message, tag = "1")]
        Set(super::AsyncMessageTrigger),
        /// Keep the existing value
        #[prost(message, tag = "2")]
        Keep(super::Empty),
    }
}
/// Structure defining a trigger for an asynchronous message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncMessageTrigger {
    /// Filter on the address
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Filter on the datastore key (Optional)
    #[prost(message, optional, tag = "2")]
    pub datastore_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// LedgerChangeEntry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LedgerChangeEntry {
    /// Address
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Ledger message
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<LedgerChangeValue>,
}
/// LedgerChangeValue
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LedgerChangeValue {
    /// The type of the change
    #[prost(enumeration = "LedgerChangeType", tag = "1")]
    pub r#type: i32,
    /// LedgerEntry or LedgerEntryUpdate
    #[prost(oneof = "ledger_change_value::Entry", tags = "2, 3")]
    pub entry: ::core::option::Option<ledger_change_value::Entry>,
}
/// Nested message and enum types in `LedgerChangeValue`.
pub mod ledger_change_value {
    /// LedgerEntry or LedgerEntryUpdate
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Entry {
        /// Created ledger entry
        #[prost(message, tag = "2")]
        CreatedEntry(super::LedgerEntry),
        /// Update ledger entry
        #[prost(message, tag = "3")]
        UpdatedEntry(super::LedgerEntryUpdate),
    }
}
/// An entry associated to an address in the `FinalLedger`
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LedgerEntry {
    /// The balance of that entry
    #[prost(message, optional, tag = "1")]
    pub balance: ::core::option::Option<NativeAmount>,
    /// Executable bytecode
    #[prost(bytes = "vec", tag = "2")]
    pub bytecode: ::prost::alloc::vec::Vec<u8>,
    /// A key-value store associating a hash to arbitrary bytes
    #[prost(message, repeated, tag = "3")]
    pub datastore: ::prost::alloc::vec::Vec<BytesMapFieldEntry>,
}
/// Represents an update to one or more fields of a `LedgerEntry`
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LedgerEntryUpdate {
    /// Change the balance
    #[prost(message, optional, tag = "1")]
    pub balance: ::core::option::Option<SetOrKeepBalance>,
    /// Change the executable bytecode
    #[prost(message, optional, tag = "2")]
    pub bytecode: ::core::option::Option<SetOrKeepBytes>,
    /// Change datastore entries
    #[prost(message, repeated, tag = "3")]
    pub datastore: ::prost::alloc::vec::Vec<SetOrDeleteDatastoreEntry>,
}
/// Set or Keep Balance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrKeepBalance {
    /// The type of the change
    #[prost(oneof = "set_or_keep_balance::Change", tags = "1, 2")]
    pub change: ::core::option::Option<set_or_keep_balance::Change>,
}
/// Nested message and enum types in `SetOrKeepBalance`.
pub mod set_or_keep_balance {
    /// The type of the change
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Change {
        /// The value of that entry (Optional)
        #[prost(message, tag = "1")]
        Set(super::NativeAmount),
        /// Keep the existing value
        #[prost(message, tag = "2")]
        Keep(super::Empty),
    }
}
/// Set or Delete DatastoreEntry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrDeleteDatastoreEntry {
    /// The type of the change
    #[prost(oneof = "set_or_delete_datastore_entry::Change", tags = "1, 2")]
    pub change: ::core::option::Option<set_or_delete_datastore_entry::Change>,
}
/// Nested message and enum types in `SetOrDeleteDatastoreEntry`.
pub mod set_or_delete_datastore_entry {
    /// The type of the change
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Change {
        /// Executable bytecode (Optional)
        #[prost(message, tag = "1")]
        Set(super::BytesMapFieldEntry),
        /// Delete the existing bytecode
        #[prost(message, tag = "2")]
        Delete(super::Empty),
    }
}
/// Read-only execution call
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadOnlyExecutionCall {
    /// Maximum gas to spend in the execution.
    #[prost(uint64, tag = "1")]
    pub max_gas: u64,
    /// Call stack to simulate, older caller first
    #[prost(message, repeated, tag = "2")]
    pub call_stack: ::prost::alloc::vec::Vec<ExecutionStackElement>,
    /// Caller's address, (Optional) if not set, an auto-generated address will be used
    #[prost(message, optional, tag = "5")]
    pub caller_address: ::core::option::Option<::prost::alloc::string::String>,
    /// execution start state
    ///
    /// Whether to start execution from final or active state
    #[prost(bool, tag = "6")]
    pub is_final: bool,
    /// Coins transferred to the target address during the call
    #[prost(message, optional, tag = "7")]
    pub coins: ::core::option::Option<NativeAmount>,
    /// fee paid by the caller when the call is processed.
    #[prost(message, optional, tag = "8")]
    pub fee: ::core::option::Option<NativeAmount>,
    /// Target of the call
    #[prost(oneof = "read_only_execution_call::Target", tags = "3, 4")]
    pub target: ::core::option::Option<read_only_execution_call::Target>,
}
/// Nested message and enum types in `ReadOnlyExecutionCall`.
pub mod read_only_execution_call {
    /// Target of the call
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Target {
        /// Byte code
        #[prost(message, tag = "3")]
        BytecodeCall(super::BytecodeExecution),
        /// Function call
        #[prost(message, tag = "4")]
        FunctionCall(super::FunctionCall),
    }
}
/// / Execute a bytecode
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BytecodeExecution {
    /// Byte code
    #[prost(bytes = "vec", tag = "1")]
    pub bytecode: ::prost::alloc::vec::Vec<u8>,
    /// Serialized datastore (key value store)  for `ExecuteSC` Operation (Optional)
    #[prost(bytes = "vec", tag = "2")]
    pub operation_datastore: ::prost::alloc::vec::Vec<u8>,
}
/// Execute a function call
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionCall {
    /// Target address
    #[prost(string, tag = "1")]
    pub target_address: ::prost::alloc::string::String,
    /// Target function
    #[prost(string, tag = "2")]
    pub target_function: ::prost::alloc::string::String,
    /// Parameter to pass to the target function
    #[prost(bytes = "vec", tag = "3")]
    pub parameter: ::prost::alloc::vec::Vec<u8>,
}
/// Structure describing the output of a read only execution
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadOnlyExecutionOutput {
    /// Output of a single execution
    #[prost(message, optional, tag = "1")]
    pub out: ::core::option::Option<ExecutionOutput>,
    /// Gas cost for this execution
    #[prost(uint64, tag = "2")]
    pub used_gas: u64,
    /// Returned value from the module call
    #[prost(bytes = "vec", tag = "3")]
    pub call_result: ::prost::alloc::vec::Vec<u8>,
}
/// Structure describing an element of the execution stack.
/// Every time a function is called from bytecode,
/// a new `ExecutionStackElement` is pushed at the top of the execution stack
/// to represent the local execution context of the called function,
/// instead of the caller's which should lie just below in the stack.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionStackElement {
    /// Called address
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Coins transferred to the target address during the call
    #[prost(message, optional, tag = "2")]
    pub coins: ::core::option::Option<NativeAmount>,
    /// List of addresses owned by the current call, and on which the current call has write access.
    /// This list should contain `ExecutionStackElement::address` in the sense that an address should have write access to itself.
    /// This list should also contain all addresses created previously during the call
    /// to allow write access on newly created addresses in order to set them up,
    /// but only within the scope of the current stack element.
    /// That way, only the current scope and neither its caller not the functions it calls gain this write access,
    /// which is important for security.
    /// Note that we use a vector instead of a pre-hashed set to ensure order determinism,
    /// the performance hit of linear search remains minimal because `owned_addresses` will always contain very few elements.
    #[prost(string, repeated, tag = "3")]
    pub owned_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Datastore (key value store) for `ExecuteSC` Operation (Optional)
    #[prost(message, repeated, tag = "4")]
    pub operation_datastore: ::prost::alloc::vec::Vec<BytesMapFieldEntry>,
}
/// ScExecutionEventStatus type enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ScExecutionEventStatus {
    /// Default enum value
    Unspecified = 0,
    /// Final status
    Final = 1,
    /// Read only status
    ReadOnly = 2,
    /// Candidate status
    Candidate = 3,
}
impl ScExecutionEventStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ScExecutionEventStatus::Unspecified => {
                "SC_EXECUTION_EVENT_STATUS_UNSPECIFIED"
            }
            ScExecutionEventStatus::Final => "SC_EXECUTION_EVENT_STATUS_FINAL",
            ScExecutionEventStatus::ReadOnly => "SC_EXECUTION_EVENT_STATUS_READ_ONLY",
            ScExecutionEventStatus::Candidate => "SC_EXECUTION_EVENT_STATUS_CANDIDATE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SC_EXECUTION_EVENT_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "SC_EXECUTION_EVENT_STATUS_FINAL" => Some(Self::Final),
            "SC_EXECUTION_EVENT_STATUS_READ_ONLY" => Some(Self::ReadOnly),
            "SC_EXECUTION_EVENT_STATUS_CANDIDATE" => Some(Self::Candidate),
            _ => None,
        }
    }
}
/// ExecutionOutputStatus type enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExecutionOutputStatus {
    /// Default enum value
    Unspecified = 0,
    /// Candidate status
    Candidate = 1,
    /// Final status
    Final = 2,
    /// Unknown status
    Unknown = 3,
}
impl ExecutionOutputStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExecutionOutputStatus::Unspecified => "EXECUTION_OUTPUT_STATUS_UNSPECIFIED",
            ExecutionOutputStatus::Candidate => "EXECUTION_OUTPUT_STATUS_CANDIDATE",
            ExecutionOutputStatus::Final => "EXECUTION_OUTPUT_STATUS_FINAL",
            ExecutionOutputStatus::Unknown => "EXECUTION_OUTPUT_STATUS_UNKNOWN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXECUTION_OUTPUT_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "EXECUTION_OUTPUT_STATUS_CANDIDATE" => Some(Self::Candidate),
            "EXECUTION_OUTPUT_STATUS_FINAL" => Some(Self::Final),
            "EXECUTION_OUTPUT_STATUS_UNKNOWN" => Some(Self::Unknown),
            _ => None,
        }
    }
}
/// OperationExecutionStatus type enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationExecutionStatus {
    /// Default enum value
    Unspecified = 0,
    /// Success status
    Success = 1,
    /// Failed only status
    Failed = 2,
}
impl OperationExecutionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperationExecutionStatus::Unspecified => {
                "OPERATION_EXECUTION_STATUS_UNSPECIFIED"
            }
            OperationExecutionStatus::Success => "OPERATION_EXECUTION_STATUS_SUCCESS",
            OperationExecutionStatus::Failed => "OPERATION_EXECUTION_STATUS_FAILED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPERATION_EXECUTION_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "OPERATION_EXECUTION_STATUS_SUCCESS" => Some(Self::Success),
            "OPERATION_EXECUTION_STATUS_FAILED" => Some(Self::Failed),
            _ => None,
        }
    }
}
/// AsyncPoolChangeType type enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AsyncPoolChangeType {
    /// Default enum value
    Unspecified = 0,
    /// Set type
    Set = 1,
    /// Activate only type
    Update = 2,
    /// Delete only type
    Delete = 3,
}
impl AsyncPoolChangeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AsyncPoolChangeType::Unspecified => "ASYNC_POOL_CHANGE_TYPE_UNSPECIFIED",
            AsyncPoolChangeType::Set => "ASYNC_POOL_CHANGE_TYPE_SET",
            AsyncPoolChangeType::Update => "ASYNC_POOL_CHANGE_TYPE_UPDATE",
            AsyncPoolChangeType::Delete => "ASYNC_POOL_CHANGE_TYPE_DELETE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ASYNC_POOL_CHANGE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "ASYNC_POOL_CHANGE_TYPE_SET" => Some(Self::Set),
            "ASYNC_POOL_CHANGE_TYPE_UPDATE" => Some(Self::Update),
            "ASYNC_POOL_CHANGE_TYPE_DELETE" => Some(Self::Delete),
            _ => None,
        }
    }
}
/// LedgerChangeType type enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LedgerChangeType {
    /// Default enum value
    Unspecified = 0,
    /// Set type
    Set = 1,
    /// Update type
    Update = 2,
    /// Delete type
    Delete = 3,
}
impl LedgerChangeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LedgerChangeType::Unspecified => "LEDGER_CHANGE_TYPE_UNSPECIFIED",
            LedgerChangeType::Set => "LEDGER_CHANGE_TYPE_SET",
            LedgerChangeType::Update => "LEDGER_CHANGE_TYPE_UPDATE",
            LedgerChangeType::Delete => "LEDGER_CHANGE_TYPE_DELETE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LEDGER_CHANGE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "LEDGER_CHANGE_TYPE_SET" => Some(Self::Set),
            "LEDGER_CHANGE_TYPE_UPDATE" => Some(Self::Update),
            "LEDGER_CHANGE_TYPE_DELETE" => Some(Self::Delete),
            _ => None,
        }
    }
}
/// NativeTime represents a native duration or unix timestamp
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeTime {
    /// Milliseconds
    #[prost(uint64, tag = "1")]
    pub milliseconds: u64,
}
/// Consensus statistics
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusStats {
    /// Start of the time span for stats
    #[prost(message, optional, tag = "1")]
    pub start_timespan: ::core::option::Option<NativeTime>,
    /// End of the time span for stats
    #[prost(message, optional, tag = "2")]
    pub end_timespan: ::core::option::Option<NativeTime>,
    /// Number of final blocks
    #[prost(uint64, tag = "3")]
    pub final_block_count: u64,
    /// Number of stale blocks in memory
    #[prost(uint64, tag = "4")]
    pub stale_block_count: u64,
    /// Number of actives cliques
    #[prost(uint64, tag = "5")]
    pub clique_count: u64,
}
/// Pool statistics
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolStats {
    /// Endorsements
    #[prost(uint64, tag = "1")]
    pub endorsements_count: u64,
    /// Operations
    #[prost(uint64, tag = "2")]
    pub operations_count: u64,
}
/// Network statistics
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkStats {
    /// In connections count
    #[prost(uint64, tag = "1")]
    pub in_connection_count: u64,
    /// Out connections count
    #[prost(uint64, tag = "2")]
    pub out_connection_count: u64,
    /// Total known peers count
    #[prost(uint64, tag = "3")]
    pub known_peer_count: u64,
    /// Banned node count
    #[prost(uint64, tag = "4")]
    pub banned_peer_count: u64,
    /// Active node count
    #[prost(uint64, tag = "5")]
    pub active_node_count: u64,
}
/// Execution statistics
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionStats {
    /// Time window start
    #[prost(message, optional, tag = "1")]
    pub time_window_start: ::core::option::Option<NativeTime>,
    /// Time window end
    #[prost(message, optional, tag = "2")]
    pub time_window_end: ::core::option::Option<NativeTime>,
    /// Number of final blocks in the time window
    #[prost(uint64, tag = "3")]
    pub final_block_count: u64,
    /// Number of final executed operations in the time window
    #[prost(uint64, tag = "4")]
    pub final_executed_operations_count: u64,
}
/// Node status
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeStatus {
    /// Our node id
    #[prost(string, tag = "1")]
    pub node_id: ::prost::alloc::string::String,
    /// Optional node ip
    #[prost(string, tag = "2")]
    pub node_ip: ::prost::alloc::string::String,
    /// Node version
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
    /// Now
    #[prost(message, optional, tag = "4")]
    pub current_time: ::core::option::Option<NativeTime>,
    /// Current cycle
    #[prost(uint64, tag = "5")]
    pub current_cycle: u64,
    /// Current cycle starting timestamp
    #[prost(message, optional, tag = "6")]
    pub current_cycle_time: ::core::option::Option<NativeTime>,
    /// Next cycle starting timestamp
    #[prost(message, optional, tag = "7")]
    pub next_cycle_time: ::core::option::Option<NativeTime>,
    /// Connected nodes
    #[prost(message, repeated, tag = "8")]
    pub connected_nodes: ::prost::alloc::vec::Vec<ConnectedNode>,
    /// Last executed final slot
    #[prost(message, optional, tag = "9")]
    pub last_executed_final_slot: ::core::option::Option<Slot>,
    /// Last executed speculative slot
    #[prost(message, optional, tag = "10")]
    pub last_executed_speculative_slot: ::core::option::Option<Slot>,
    /// The hash of the XOF final state hash
    #[prost(string, tag = "11")]
    pub final_state_fingerprint: ::prost::alloc::string::String,
    /// Consensus stats
    #[prost(message, optional, tag = "12")]
    pub consensus_stats: ::core::option::Option<ConsensusStats>,
    /// Pool stats (operation count and endorsement count)
    #[prost(message, optional, tag = "13")]
    pub pool_stats: ::core::option::Option<PoolStats>,
    /// Network stats
    #[prost(message, optional, tag = "14")]
    pub network_stats: ::core::option::Option<NetworkStats>,
    /// Execution stats
    #[prost(message, optional, tag = "15")]
    pub execution_stats: ::core::option::Option<ExecutionStats>,
    /// Compact configuration
    #[prost(message, optional, tag = "16")]
    pub config: ::core::option::Option<CompactConfig>,
}
/// Connected node
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectedNode {
    /// Node id
    #[prost(string, tag = "1")]
    pub node_id: ::prost::alloc::string::String,
    /// Node ip
    #[prost(string, tag = "2")]
    pub node_ip: ::prost::alloc::string::String,
    /// Connection type
    #[prost(enumeration = "ConnectionType", tag = "3")]
    pub connection_type: i32,
}
/// Compact configuration
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompactConfig {
    /// Time in milliseconds when the blockclique started.
    #[prost(message, optional, tag = "1")]
    pub genesis_timestamp: ::core::option::Option<NativeTime>,
    /// TESTNET: time when the blockclique is ended.
    #[prost(message, optional, tag = "2")]
    pub end_timestamp: ::core::option::Option<NativeTime>,
    /// Number of threads
    #[prost(uint32, tag = "3")]
    pub thread_count: u32,
    /// Time between the periods in the same thread.
    #[prost(message, optional, tag = "4")]
    pub t0: ::core::option::Option<NativeTime>,
    /// Threshold for fitness.
    #[prost(uint64, tag = "5")]
    pub delta_f0: u64,
    /// Maximum operation validity period count
    #[prost(uint64, tag = "6")]
    pub operation_validity_periods: u64,
    /// cycle duration in periods
    #[prost(uint64, tag = "7")]
    pub periods_per_cycle: u64,
    /// Reward amount for a block creation
    #[prost(message, optional, tag = "8")]
    pub block_reward: ::core::option::Option<NativeAmount>,
    /// Price of a roll on the network
    #[prost(message, optional, tag = "9")]
    pub roll_price: ::core::option::Option<NativeAmount>,
    /// Max total size of a block
    #[prost(uint32, tag = "10")]
    pub max_block_size: u32,
}
/// Public status
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicStatus {
    /// Our node id
    #[prost(string, tag = "1")]
    pub node_id: ::prost::alloc::string::String,
    /// Node version
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
    /// Now
    #[prost(message, optional, tag = "4")]
    pub current_time: ::core::option::Option<NativeTime>,
    /// Current cycle
    #[prost(uint64, tag = "5")]
    pub current_cycle: u64,
    /// Current cycle starting timestamp
    #[prost(message, optional, tag = "6")]
    pub current_cycle_time: ::core::option::Option<NativeTime>,
    /// Next cycle starting timestamp
    #[prost(message, optional, tag = "7")]
    pub next_cycle_time: ::core::option::Option<NativeTime>,
    /// Last executed final slot
    #[prost(message, optional, tag = "8")]
    pub last_executed_final_slot: ::core::option::Option<Slot>,
    /// Last executed speculative slot
    #[prost(message, optional, tag = "9")]
    pub last_executed_speculative_slot: ::core::option::Option<Slot>,
    /// The hash of the XOF final state hash
    #[prost(string, tag = "10")]
    pub final_state_fingerprint: ::prost::alloc::string::String,
    /// Compact configuration
    #[prost(message, optional, tag = "11")]
    pub config: ::core::option::Option<CompactConfig>,
}
/// ConnectionType enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ConnectionType {
    /// Default enum value
    Unspecified = 0,
    /// Incoming connection
    Incoming = 1,
    /// Outgoing connection
    Outgoing = 2,
}
impl ConnectionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ConnectionType::Unspecified => "CONNECTION_TYPE_UNSPECIFIED",
            ConnectionType::Incoming => "CONNECTION_TYPE_INCOMING",
            ConnectionType::Outgoing => "CONNECTION_TYPE_OUTGOING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CONNECTION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "CONNECTION_TYPE_INCOMING" => Some(Self::Incoming),
            "CONNECTION_TYPE_OUTGOING" => Some(Self::Outgoing),
            _ => None,
        }
    }
}
/// StakerEntry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StakerEntry {
    /// Address
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Rolls
    #[prost(uint64, tag = "2")]
    pub rolls: u64,
}
/// Entry for GetMipStatusResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MipStatusEntry {
    /// Mip info
    #[prost(message, optional, tag = "1")]
    pub mip_info: ::core::option::Option<MipInfo>,
    /// State id
    #[prost(enumeration = "ComponentStateId", tag = "2")]
    pub state_id: i32,
}
/// MIP info (name & versions & time range for a MIP)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MipInfo {
    /// MIP name or descriptive name
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Network (or global) version (to be included in block header)
    #[prost(uint32, tag = "2")]
    pub version: u32,
    /// A timestamp at which the version gains its meaning (e.g. announced in block header)
    #[prost(message, optional, tag = "3")]
    pub start: ::core::option::Option<NativeTime>,
    /// A timestamp at the which the deployment is considered failed
    #[prost(message, optional, tag = "4")]
    pub timeout: ::core::option::Option<NativeTime>,
    /// Once deployment has been locked, wait for this duration before deployment is considered active
    #[prost(message, optional, tag = "5")]
    pub activation_delay: ::core::option::Option<NativeTime>,
    /// Components concerned by this versioning (e.g. a new Block version), and the associated component_version
    #[prost(message, repeated, tag = "6")]
    pub components: ::prost::alloc::vec::Vec<MipComponentEntry>,
}
/// MipComponentEntry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MipComponentEntry {
    /// Kind
    #[prost(enumeration = "MipComponent", tag = "1")]
    pub kind: i32,
    /// Version
    #[prost(uint32, tag = "2")]
    pub version: u32,
}
/// State machine for a Versioning component that tracks the deployment state
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ComponentStateId {
    /// Default enum value
    Unspecified = 0,
    /// Error state
    Error = 1,
    /// Initial state
    Defined = 2,
    /// Past start, can only go to LockedIn after the threshold is above a given value
    Started = 3,
    /// Locked but wait for some time before going to active (to let users the time to upgrade)
    Lockedin = 4,
    /// After LockedIn, deployment is considered successful (after activation delay)
    Active = 5,
    /// Past the timeout, if LockedIn is not reach
    Failed = 6,
}
impl ComponentStateId {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ComponentStateId::Unspecified => "COMPONENT_STATE_ID_UNSPECIFIED",
            ComponentStateId::Error => "COMPONENT_STATE_ID_ERROR",
            ComponentStateId::Defined => "COMPONENT_STATE_ID_DEFINED",
            ComponentStateId::Started => "COMPONENT_STATE_ID_STARTED",
            ComponentStateId::Lockedin => "COMPONENT_STATE_ID_LOCKEDIN",
            ComponentStateId::Active => "COMPONENT_STATE_ID_ACTIVE",
            ComponentStateId::Failed => "COMPONENT_STATE_ID_FAILED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "COMPONENT_STATE_ID_UNSPECIFIED" => Some(Self::Unspecified),
            "COMPONENT_STATE_ID_ERROR" => Some(Self::Error),
            "COMPONENT_STATE_ID_DEFINED" => Some(Self::Defined),
            "COMPONENT_STATE_ID_STARTED" => Some(Self::Started),
            "COMPONENT_STATE_ID_LOCKEDIN" => Some(Self::Lockedin),
            "COMPONENT_STATE_ID_ACTIVE" => Some(Self::Active),
            "COMPONENT_STATE_ID_FAILED" => Some(Self::Failed),
            _ => None,
        }
    }
}
/// Versioning component enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MipComponent {
    /// Default enum value
    Unspecified = 0,
    /// Address component
    Address = 1,
    /// Keypair component
    Keypair = 2,
}
impl MipComponent {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MipComponent::Unspecified => "MIP_COMPONENT_UNSPECIFIED",
            MipComponent::Address => "MIP_COMPONENT_ADDRESS",
            MipComponent::Keypair => "MIP_COMPONENT_KEYPAIR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MIP_COMPONENT_UNSPECIFIED" => Some(Self::Unspecified),
            "MIP_COMPONENT_ADDRESS" => Some(Self::Address),
            "MIP_COMPONENT_KEYPAIR" => Some(Self::Keypair),
            _ => None,
        }
    }
}
