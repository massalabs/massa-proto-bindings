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
    /// A secure hash of the data. See also \[massa_hash::Hash\]
    #[prost(string, tag = "5")]
    pub id: ::prost::alloc::string::String,
    /// The size of the serialized endorsement in bytes
    #[prost(uint64, tag = "6")]
    pub serialized_size: u64,
}
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
/// Packages a type such that it can be securely sent and received in a trust-free network
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecureShare {
    /// Content in sharable, deserializable form. Is used in the secure verification protocols
    #[prost(bytes = "vec", tag = "1")]
    pub serialized_data: ::prost::alloc::vec::Vec<u8>,
    /// A cryptographically generated value using `serialized_data` and a public key.
    #[prost(string, tag = "2")]
    pub signature: ::prost::alloc::string::String,
    /// The public-key component used in the generation of the signature
    #[prost(string, tag = "3")]
    pub content_creator_pub_key: ::prost::alloc::string::String,
    /// Derived from the same public key used to generate the signature
    #[prost(string, tag = "4")]
    pub content_creator_address: ::prost::alloc::string::String,
    /// A secure hash of the data. See also \[massa_hash::Hash\]
    #[prost(string, tag = "5")]
    pub id: ::prost::alloc::string::String,
}
/// The operation as sent in the network
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
    /// The fee they have decided for this operation
    #[prost(uint64, tag = "1")]
    pub fee: u64,
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
    #[prost(uint64, tag = "2")]
    pub amount: u64,
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
    pub target_addr: ::prost::alloc::string::String,
    /// Target function name. No function is called if empty
    #[prost(string, tag = "2")]
    pub target_func: ::prost::alloc::string::String,
    /// Parameter to pass to the target function
    #[prost(bytes = "vec", tag = "3")]
    pub param: ::prost::alloc::vec::Vec<u8>,
    /// The maximum amount of gas that the execution of the contract is allowed to cost
    #[prost(uint64, tag = "4")]
    pub max_gas: u64,
    /// Extra coins that are spent from the caller's balance and transferred to the target
    #[prost(uint64, tag = "5")]
    pub coins: u64,
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
    /// A secure hash of the data. See also \[massa_hash::Hash\]
    #[prost(string, tag = "5")]
    pub id: ::prost::alloc::string::String,
    /// The size of the serialized operation in bytes
    #[prost(uint64, tag = "6")]
    pub serialized_size: u64,
}
/// A wrapper around an operation with its metadata
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationWrapper {
    /// The unique ID of the operation.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The IDs of the blocks in which the operation appears
    #[prost(string, repeated, tag = "3")]
    pub block_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The thread in which the operation can be included
    #[prost(uint32, tag = "5")]
    pub thread: u32,
    /// The operation object itself
    #[prost(message, optional, tag = "6")]
    pub operation: ::core::option::Option<SignedOperation>,
    /// The execution statuses of the operation
    #[prost(enumeration = "OperationStatus", repeated, tag = "7")]
    pub status: ::prost::alloc::vec::Vec<i32>,
}
/// Possible statuses for an operation
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationStatus {
    /// Default enum value
    Unspecified = 0,
    /// The operation is still pending
    Pending = 1,
    /// The operation is final
    Final = 2,
    /// The operation was executed successfully
    Success = 3,
    /// The operation failed to execute
    Failure = 4,
    /// The status of the operation is unknown
    Unknown = 5,
}
impl OperationStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperationStatus::Unspecified => "OPERATION_STATUS_UNSPECIFIED",
            OperationStatus::Pending => "OPERATION_STATUS_PENDING",
            OperationStatus::Final => "OPERATION_STATUS_FINAL",
            OperationStatus::Success => "OPERATION_STATUS_SUCCESS",
            OperationStatus::Failure => "OPERATION_STATUS_FAILURE",
            OperationStatus::Unknown => "OPERATION_STATUS_UNKNOWN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPERATION_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "OPERATION_STATUS_PENDING" => Some(Self::Pending),
            "OPERATION_STATUS_FINAL" => Some(Self::Final),
            "OPERATION_STATUS_SUCCESS" => Some(Self::Success),
            "OPERATION_STATUS_FAILURE" => Some(Self::Failure),
            "OPERATION_STATUS_UNKNOWN" => Some(Self::Unknown),
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
    pub operations: ::prost::alloc::vec::Vec<FilledOperationTuple>,
}
/// Block header
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockHeader {
    /// Slot
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<Slot>,
    /// parents
    #[prost(string, repeated, tag = "2")]
    pub parents: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// All operations hash
    #[prost(string, tag = "3")]
    pub operation_merkle_root: ::prost::alloc::string::String,
    /// Signed endorsements
    #[prost(message, repeated, tag = "4")]
    pub endorsements: ::prost::alloc::vec::Vec<SignedEndorsement>,
}
/// Filled Operation Tuple
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilledOperationTuple {
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
    /// A secure hash of the data. See also \[massa_hash::Hash\]
    #[prost(string, tag = "5")]
    pub id: ::prost::alloc::string::String,
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
    /// A secure hash of the data. See also \[massa_hash::Hash\]
    #[prost(string, tag = "5")]
    pub id: ::prost::alloc::string::String,
    /// The size of the serialized block header in bytes
    #[prost(uint64, tag = "6")]
    pub serialized_size: u64,
}
/// A wrapper around a block with its metadata
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockWrapper {
    /// The unique ID of the block.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The block object itself
    #[prost(message, optional, tag = "2")]
    pub block: ::core::option::Option<Block>,
    /// The execution statuses of the block
    #[prost(enumeration = "BlockStatus", repeated, tag = "3")]
    pub status: ::prost::alloc::vec::Vec<i32>,
}
/// Possible statuses for a block
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BlockStatus {
    /// Default enum value
    Unspecified = 0,
    /// The block is in the greatest clique (and not final)
    InBlockclique = 1,
    /// The block is final
    Final = 2,
    /// The block is candidate (active any clique but not final)
    Candidate = 3,
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
            BlockStatus::InBlockclique => "BLOCK_STATUS_IN_BLOCKCLIQUE",
            BlockStatus::Final => "BLOCK_STATUS_FINAL",
            BlockStatus::Candidate => "BLOCK_STATUS_CANDIDATE",
            BlockStatus::Discarded => "BLOCK_STATUS_DISCARDED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BLOCK_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "BLOCK_STATUS_IN_BLOCKCLIQUE" => Some(Self::InBlockclique),
            "BLOCK_STATUS_FINAL" => Some(Self::Final),
            "BLOCK_STATUS_CANDIDATE" => Some(Self::Candidate),
            "BLOCK_STATUS_DISCARDED" => Some(Self::Discarded),
            _ => None,
        }
    }
}
/// Selector draws
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelectorDraws {
    /// Address
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Next block draws
    #[prost(message, repeated, tag = "2")]
    pub next_block_draws: ::prost::alloc::vec::Vec<Slot>,
    /// Next endorsements draws
    #[prost(message, repeated, tag = "3")]
    pub next_endorsement_draws: ::prost::alloc::vec::Vec<IndexedSlot>,
}
/// SlotExecutionOutput
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SlotExecutionOutput {
    /// Status
    #[prost(enumeration = "ExecutionOutputStatus", repeated, tag = "1")]
    pub status: ::prost::alloc::vec::Vec<i32>,
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
    /// Block id at that slot (optional)
    #[prost(string, optional, tag = "2")]
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
    /// json data string
    #[prost(string, tag = "2")]
    pub data: ::prost::alloc::string::String,
}
/// ScExecutionEvent context
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScExecutionEventContext {
    /// base58 encoded slot(period + thread) + index_in_slot
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// When was it generated
    #[prost(message, optional, tag = "2")]
    pub origin_slot: ::core::option::Option<Slot>,
    /// Block id if there was a block at that slot (optional)
    #[prost(string, optional, tag = "3")]
    pub block_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Index of the event in the slot
    #[prost(uint64, tag = "4")]
    pub index_in_slot: u64,
    /// Call stack addresses. most recent at the end
    #[prost(string, repeated, tag = "5")]
    pub call_stack: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Origin operation id (optional)
    #[prost(string, optional, tag = "6")]
    pub origin_operation_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Status
    #[prost(enumeration = "ScExecutionEventStatus", repeated, tag = "7")]
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
    #[prost(enumeration = "OperationExecutionStatus", repeated, tag = "1")]
    pub status: ::prost::alloc::vec::Vec<i32>,
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
    #[prost(uint64, tag = "7")]
    pub fee: u64,
    /// Coins sent from the sender to the target address of the message.
    /// Those coins are spent by the sender address when the message is sent,
    /// and credited to the destination address when receiving the message.
    /// In case of failure or discard, those coins are reimbursed to the sender.
    #[prost(uint64, tag = "8")]
    pub coins: u64,
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
    /// Hash of the message
    #[prost(string, tag = "14")]
    pub hash: ::prost::alloc::string::String,
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
    pub emission_index: ::core::option::Option<SetOrKeepFixed64>,
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
    pub max_gas: ::core::option::Option<SetOrKeepFixed64>,
    /// Change the fee paid by the sender when the message is processed.
    #[prost(message, optional, tag = "7")]
    pub fee: ::core::option::Option<SetOrKeepFixed64>,
    /// Change the coins sent from the sender to the target address of the message.
    /// Those coins are spent by the sender address when the message is sent,
    /// and credited to the destination address when receiving the message.
    /// In case of failure or discard, those coins are reimbursed to the sender.
    #[prost(message, optional, tag = "8")]
    pub coins: ::core::option::Option<SetOrKeepFixed64>,
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
    /// Change the hash of the message
    #[prost(message, optional, tag = "14")]
    pub hash: ::core::option::Option<SetOrKeepString>,
}
/// Set or Keep Slot
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrKeepSlot {
    /// The type of the change
    #[prost(enumeration = "SetOrKeepType", tag = "1")]
    pub r#type: i32,
    /// The value of that entry (optional)
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<Slot>,
}
/// Set or Keep Fixed64
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrKeepFixed64 {
    /// The type of the change
    #[prost(enumeration = "SetOrKeepType", tag = "1")]
    pub r#type: i32,
    /// The value of that entry (optional)
    #[prost(uint64, optional, tag = "2")]
    pub value: ::core::option::Option<u64>,
}
/// Set or Keep String
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrKeepString {
    /// The type of the change
    #[prost(enumeration = "SetOrKeepType", tag = "1")]
    pub r#type: i32,
    /// The value of that entry (optional)
    #[prost(string, optional, tag = "2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
/// Set or Keep Bytes
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrKeepBytes {
    /// The type of the change
    #[prost(enumeration = "SetOrKeepType", tag = "1")]
    pub r#type: i32,
    /// The value of that entry (optional)
    #[prost(bytes = "vec", optional, tag = "2")]
    pub value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Set or Keep Bool
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrKeepBool {
    /// The type of the change
    #[prost(enumeration = "SetOrKeepType", tag = "1")]
    pub r#type: i32,
    /// The value of that entry (optional)
    #[prost(bool, optional, tag = "2")]
    pub value: ::core::option::Option<bool>,
}
/// Set or Keep AsyncMessageTrigger
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrKeepAsyncMessageTrigger {
    /// The type of the change
    #[prost(enumeration = "SetOrKeepType", tag = "1")]
    pub r#type: i32,
    /// The value of that entry (optional)
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<AsyncMessageTrigger>,
}
/// Structure defining a trigger for an asynchronous message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncMessageTrigger {
    /// Filter on the address
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Filter on the datastore key (optional)
    #[prost(bytes = "vec", optional, tag = "2")]
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
    #[prost(uint64, tag = "1")]
    pub balance: u64,
    /// Executable bytecode
    #[prost(bytes = "vec", tag = "2")]
    pub bytecode: ::prost::alloc::vec::Vec<u8>,
    /// A key-value store associating a hash to arbitrary bytes
    #[prost(message, repeated, tag = "3")]
    pub entries: ::prost::alloc::vec::Vec<BytesMapFieldEntry>,
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
    pub bytecode: ::core::option::Option<SetOrKeepBytecode>,
    /// Change datastore entries
    #[prost(message, repeated, tag = "3")]
    pub datastore: ::prost::alloc::vec::Vec<SetOrDeleteDatastoreEntry>,
}
/// Set or Keep Balance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrKeepBalance {
    /// The type of the change
    #[prost(enumeration = "SetOrKeepType", tag = "1")]
    pub r#type: i32,
    /// The balance of that entry (optional)
    #[prost(uint64, optional, tag = "2")]
    pub balance: ::core::option::Option<u64>,
}
/// Set or Keep Bytecode
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrKeepBytecode {
    /// The type of the change
    #[prost(enumeration = "SetOrKeepType", tag = "1")]
    pub r#type: i32,
    /// Executable bytecode (optional)
    #[prost(bytes = "vec", optional, tag = "2")]
    pub bytecode: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Set or Delete DatastoreEntry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrDeleteDatastoreEntry {
    /// The type of the change
    #[prost(enumeration = "SetOrDeleteType", tag = "1")]
    pub r#type: i32,
    /// The balance of that entry (optioal)
    #[prost(message, optional, tag = "2")]
    pub datastore_entry: ::core::option::Option<BytesMapFieldEntry>,
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
/// Query state query item
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionQueryRequestItem {
    #[prost(
        oneof = "execution_query_request_item::RequestItem",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20"
    )]
    pub request_item: ::core::option::Option<execution_query_request_item::RequestItem>,
}
/// Nested message and enum types in `ExecutionQueryRequestItem`.
pub mod execution_query_request_item {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequestItem {
        /// Checks if address exists (candidate)
        #[prost(message, tag = "1")]
        AddressExistsCandidate(super::AddressExistsCandidate),
        /// Checks if address exists (final)
        #[prost(message, tag = "2")]
        AddressExistsFinal(super::AddressExistsFinal),
        /// Gets the balance (candidate) of an address
        #[prost(message, tag = "3")]
        AddressBalanceCandidate(super::AddressBalanceCandidate),
        /// Gets the balance (final) of an address
        #[prost(message, tag = "4")]
        AddressBalanceFinal(super::AddressBalanceFinal),
        /// Gets the bytecode (candidate) of an address
        #[prost(message, tag = "5")]
        AddressBytecodeCandidate(super::AddressBytecodeCandidate),
        /// Gets the bytecode (final) of an address
        #[prost(message, tag = "6")]
        AddressBytecodeFinal(super::AddressBytecodeFinal),
        /// Gets the datastore keys (candidate) of an address
        #[prost(message, tag = "7")]
        AddressDatastoreKeysCandidate(super::AddressDatastoreKeysCandidate),
        /// Gets the datastore keys (final) of an address
        #[prost(message, tag = "8")]
        AddressDatastoreKeysFinal(super::AddressDatastoreKeysFinal),
        /// Gets a datastore value (candidate) for an address
        #[prost(message, tag = "9")]
        AddressDatastoreValueCandidate(super::AddressDatastoreValueCandidate),
        /// Gets a datastore value (final) for an address
        #[prost(message, tag = "10")]
        AddressDatastoreValueFinal(super::AddressDatastoreValueFinal),
        /// Gets the execution status (candidate) for an operation
        #[prost(message, tag = "11")]
        OpExecutionStatusCandidate(super::OpExecutionStatusCandidate),
        /// Gets the execution status (final) for an operation
        #[prost(message, tag = "12")]
        OpExecutionStatusFinal(super::OpExecutionStatusFinal),
        /// Gets the execution status (candidate) for a denunciation
        #[prost(message, tag = "13")]
        DenunciationExecutionStatusCandidate(
            super::DenunciationExecutionStatusCandidate,
        ),
        /// Gets the execution status (final) for a denunciation
        #[prost(message, tag = "14")]
        DenunciationExecutionStatusFinal(super::DenunciationExecutionStatusFinal),
        /// Gets the roll count (candidate) of an address
        #[prost(message, tag = "15")]
        AddressRollsCandidate(super::AddressRollsCandidate),
        /// Gets the roll count (final) of an address
        #[prost(message, tag = "16")]
        AddressRollsFinal(super::AddressRollsFinal),
        /// Gets the deferred credits (candidate) of an address
        #[prost(message, tag = "17")]
        AddressDeferredCreditsCandidate(super::AddressDeferredCreditsCandidate),
        /// Gets the deferred credits (final) of an address
        #[prost(message, tag = "18")]
        AddressDeferredCreditsFinal(super::AddressDeferredCreditsFinal),
        /// Gets all information for a given cycle
        #[prost(message, tag = "19")]
        CycleInfos(super::CycleInfos),
        /// Gets filtered events
        #[prost(message, tag = "20")]
        Events(super::Events),
    }
}
/// Execution state query response item
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionQueryResponseItem {
    #[prost(
        oneof = "execution_query_response_item::ResponseItem",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9"
    )]
    pub response_item: ::core::option::Option<
        execution_query_response_item::ResponseItem,
    >,
}
/// Nested message and enum types in `ExecutionQueryResponseItem`.
pub mod execution_query_response_item {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ResponseItem {
        /// Boolean value
        #[prost(bool, tag = "1")]
        Boolean(bool),
        /// Roll counts value
        #[prost(uint64, tag = "2")]
        RollCount(u64),
        /// Amount value
        #[prost(uint64, tag = "3")]
        Amount(u64),
        /// Bytes value
        #[prost(bytes, tag = "4")]
        Bytes(::prost::alloc::vec::Vec<u8>),
        /// Vector of bytes value
        #[prost(message, tag = "5")]
        VecBytes(super::ArrayOfBytesWrapper),
        /// Deferred credits value
        #[prost(uint64, tag = "6")]
        DeferredCredits(u64),
        /// Execution status value
        #[prost(enumeration = "super::ExecutionQueryExecutionStatus", tag = "7")]
        ExecutionStatus(i32),
        /// Cycle infos value
        #[prost(message, tag = "8")]
        CycleInfos(super::ExecutionQueryCycleInfos),
        /// Events
        #[prost(message, tag = "9")]
        Events(super::ScOutputEventsWrapper),
    }
}
/// Cycle information for execution query
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionQueryCycleInfos {
    /// Cycle number
    #[prost(uint64, tag = "1")]
    pub cycle: u64,
    /// Whether the cycle is final
    #[prost(bool, tag = "2")]
    pub is_final: bool,
    /// Infos for each PoS-participating address among the ones that were asked
    #[prost(message, repeated, tag = "3")]
    pub staker_infos: ::prost::alloc::vec::Vec<ExecutionQueryStakerInfoEntry>,
}
/// ExecutionQueryStakerInfo entry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionQueryStakerInfoEntry {
    /// Address of the staker
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Staker info
    #[prost(message, optional, tag = "2")]
    pub info: ::core::option::Option<ExecutionQueryStakerInfo>,
}
/// Staker information for execution query
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionQueryStakerInfo {
    /// Active roll count
    #[prost(uint64, tag = "1")]
    pub active_rolls: u64,
    /// Production stats
    #[prost(message, repeated, tag = "2")]
    pub production_stats: ::prost::alloc::vec::Vec<
        ExecutionQueryStakerInfoProductionStatsEntry,
    >,
}
/// ExecutionQueryStakerInfoProductionStats entry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionQueryStakerInfoProductionStatsEntry {
    /// Address of the staker
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Production failure
    #[prost(message, optional, tag = "2")]
    pub stats: ::core::option::Option<ExecutionQueryStakerInfoProductionStats>,
}
/// Production statistics for staker info in execution query
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionQueryStakerInfoProductionStats {
    /// Production successes
    #[prost(uint64, tag = "1")]
    pub block_success_count: u64,
    /// Production failures
    #[prost(uint64, tag = "2")]
    pub block_failure_count: u64,
}
/// Request to check if address exists (candidate)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressExistsCandidate {
    /// Address to check
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// Request to check if address exists (final)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressExistsFinal {
    /// Address to check
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// Request to get the balance (candidate) of an address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressBalanceCandidate {
    /// Address to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// Request to get the balance (final) of an address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressBalanceFinal {
    /// Address to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// Request to get the bytecode (candidate) of an address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressBytecodeCandidate {
    /// Address to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// Request to get the bytecode (final) of an address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressBytecodeFinal {
    /// Address to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// Request to get the datastore keys (candidate) of an address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressDatastoreKeysCandidate {
    /// Address to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Prefix for the keys
    #[prost(bytes = "vec", tag = "2")]
    pub prefix: ::prost::alloc::vec::Vec<u8>,
}
/// Request to get the datastore keys (final) of an address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressDatastoreKeysFinal {
    /// Address to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Prefix for the keys
    #[prost(bytes = "vec", tag = "2")]
    pub prefix: ::prost::alloc::vec::Vec<u8>,
}
/// Request to get a datastore value (candidate) for an address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressDatastoreValueCandidate {
    /// Address to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Key for the value
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
/// Request to get a datastore value (final) for an address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressDatastoreValueFinal {
    /// Address to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Key for the value
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
/// Request to get the execution status (candidate) for an operation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpExecutionStatusCandidate {
    /// Operation ID to query
    #[prost(string, tag = "1")]
    pub operation_id: ::prost::alloc::string::String,
}
/// Request to get the execution status (final) for an operation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpExecutionStatusFinal {
    /// Operation ID to query
    #[prost(string, tag = "1")]
    pub operation_id: ::prost::alloc::string::String,
}
/// Request to get the execution status (candidate) for a denunciation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenunciationExecutionStatusCandidate {
    /// Denunciation index to query
    #[prost(message, optional, tag = "1")]
    pub denunciation_index: ::core::option::Option<DenunciationIndex>,
}
/// Request to get the execution status (final) for a denunciation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenunciationExecutionStatusFinal {
    /// Denunciation index to query
    #[prost(message, optional, tag = "1")]
    pub denunciation_index: ::core::option::Option<DenunciationIndex>,
}
/// Request to get the roll count (candidate) of an address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressRollsCandidate {
    /// Address to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// Request to get the roll count (final) of an address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressRollsFinal {
    /// Address to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// Request to get the deferred credits (candidate) of an address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressDeferredCreditsCandidate {
    /// Address to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// Request to get the deferred credits (final) of an address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressDeferredCreditsFinal {
    /// Address to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// Request to get all information for a given cycle
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CycleInfos {
    /// Cycle to query
    #[prost(uint64, tag = "1")]
    pub cycle: u64,
    /// Addresses to restrict the query (if None, info for all addresses will be returned)
    #[prost(string, repeated, tag = "2")]
    pub restrict_to_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request to get filtered events
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    /// Event filter to apply
    #[prost(message, optional, tag = "1")]
    pub filter: ::core::option::Option<ScExecutionEventsFilter>,
}
/// ScExecutionEvents Filter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScExecutionEventsFilter {
    /// Start slot (Optional)
    #[prost(message, optional, tag = "1")]
    pub start_slot: ::core::option::Option<Slot>,
    /// End slot (Optional)
    #[prost(message, optional, tag = "2")]
    pub end_slot: ::core::option::Option<Slot>,
    /// Caller address
    #[prost(string, optional, tag = "3")]
    pub caller_address: ::core::option::Option<::prost::alloc::string::String>,
    /// Emitter address (Optional)
    #[prost(string, optional, tag = "4")]
    pub emitter_address: ::core::option::Option<::prost::alloc::string::String>,
    /// Original operation id (Optional)
    #[prost(string, optional, tag = "5")]
    pub original_operation_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Status
    #[prost(enumeration = "ScExecutionEventStatus", repeated, tag = "6")]
    pub status: ::prost::alloc::vec::Vec<i32>,
}
/// SCOutputEvents wrapper
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScOutputEventsWrapper {
    /// Events
    #[prost(message, repeated, tag = "1")]
    pub event: ::prost::alloc::vec::Vec<ScExecutionEvent>,
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
    /// Failure status
    Failure = 3,
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
            ScExecutionEventStatus::Failure => "SC_EXECUTION_EVENT_STATUS_FAILURE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SC_EXECUTION_EVENT_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "SC_EXECUTION_EVENT_STATUS_FINAL" => Some(Self::Final),
            "SC_EXECUTION_EVENT_STATUS_READ_ONLY" => Some(Self::ReadOnly),
            "SC_EXECUTION_EVENT_STATUS_FAILURE" => Some(Self::Failure),
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
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXECUTION_OUTPUT_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "EXECUTION_OUTPUT_STATUS_CANDIDATE" => Some(Self::Candidate),
            "EXECUTION_OUTPUT_STATUS_FINAL" => Some(Self::Final),
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
/// SetOrKeepType type enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SetOrKeepType {
    /// Default enum value
    Unspecified = 0,
    /// Sets a new absolute value
    Set = 1,
    /// Keeps the existing value
    Keep = 2,
}
impl SetOrKeepType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SetOrKeepType::Unspecified => "SET_OR_KEEP_TYPE_UNSPECIFIED",
            SetOrKeepType::Set => "SET_OR_KEEP_TYPE_SET",
            SetOrKeepType::Keep => "SET_OR_KEEP_TYPE_KEEP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SET_OR_KEEP_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "SET_OR_KEEP_TYPE_SET" => Some(Self::Set),
            "SET_OR_KEEP_TYPE_KEEP" => Some(Self::Keep),
            _ => None,
        }
    }
}
/// SetOrDeleteType type enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SetOrDeleteType {
    /// Default enum value
    Unspecified = 0,
    /// Sets a new absolute value
    Set = 1,
    /// Deletes the existing value
    Delete = 2,
}
impl SetOrDeleteType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SetOrDeleteType::Unspecified => "SET_OR_DELETE_TYPE_UNSPECIFIED",
            SetOrDeleteType::Set => "SET_OR_DELETE_TYPE_SET",
            SetOrDeleteType::Delete => "SET_OR_DELETE_TYPE_DELETE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SET_OR_DELETE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "SET_OR_DELETE_TYPE_SET" => Some(Self::Set),
            "SET_OR_DELETE_TYPE_DELETE" => Some(Self::Delete),
            _ => None,
        }
    }
}
/// Execution status of an operation or denunciation
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExecutionQueryExecutionStatus {
    /// Default enum value
    Unspecified = 0,
    /// The operation or denunciation was executed recently with success
    AlreadyExecutedWithSuccess = 1,
    /// The operation or denunciation was executed recently with failure
    AlreadyExecutedWithFailure = 2,
    /// The operation or denunciation was not executed recently but can still be executed unless expired
    ExecutableOrExpired = 3,
}
impl ExecutionQueryExecutionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExecutionQueryExecutionStatus::Unspecified => {
                "EXECUTION_QUERY_EXECUTION_STATUS_UNSPECIFIED"
            }
            ExecutionQueryExecutionStatus::AlreadyExecutedWithSuccess => {
                "EXECUTION_QUERY_EXECUTION_STATUS_ALREADY_EXECUTED_WITH_SUCCESS"
            }
            ExecutionQueryExecutionStatus::AlreadyExecutedWithFailure => {
                "EXECUTION_QUERY_EXECUTION_STATUS_ALREADY_EXECUTED_WITH_FAILURE"
            }
            ExecutionQueryExecutionStatus::ExecutableOrExpired => {
                "EXECUTION_QUERY_EXECUTION_STATUS_EXECUTABLE_OR_EXPIRED"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXECUTION_QUERY_EXECUTION_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "EXECUTION_QUERY_EXECUTION_STATUS_ALREADY_EXECUTED_WITH_SUCCESS" => {
                Some(Self::AlreadyExecutedWithSuccess)
            }
            "EXECUTION_QUERY_EXECUTION_STATUS_ALREADY_EXECUTED_WITH_FAILURE" => {
                Some(Self::AlreadyExecutedWithFailure)
            }
            "EXECUTION_QUERY_EXECUTION_STATUS_EXECUTABLE_OR_EXPIRED" => {
                Some(Self::ExecutableOrExpired)
            }
            _ => None,
        }
    }
}
/// NativeHash.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeHash {
    /// Version of the hash.
    #[prost(fixed64, tag = "1")]
    pub version: u64,
    /// Content of the hash.
    #[prost(bytes = "vec", tag = "2")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
/// Consensus statistics
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusStats {
    /// Start of the time span for stats
    #[prost(uint64, tag = "1")]
    pub start_timespan: u64,
    /// End of the time span for stats
    #[prost(uint64, tag = "2")]
    pub end_timespan: u64,
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
    #[prost(uint64, tag = "1")]
    pub time_window_start: u64,
    /// Time window end
    #[prost(uint64, tag = "2")]
    pub time_window_end: u64,
    /// Number of final blocks in the time window
    #[prost(uint64, tag = "3")]
    pub final_block_count: u64,
    /// Number of final executed operations in the time window
    #[prost(uint64, tag = "4")]
    pub final_executed_operations_count: u64,
    /// Active execution cursor slot
    #[prost(message, optional, tag = "5")]
    pub active_cursor: ::core::option::Option<Slot>,
    /// Final execution cursor slot
    #[prost(message, optional, tag = "6")]
    pub final_cursor: ::core::option::Option<Slot>,
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
    #[prost(uint64, tag = "4")]
    pub current_time: u64,
    /// Current cycle
    #[prost(uint64, tag = "5")]
    pub current_cycle: u64,
    /// Current cycle starting timestamp
    #[prost(uint64, tag = "6")]
    pub current_cycle_time: u64,
    /// Next cycle starting timestamp
    #[prost(uint64, tag = "7")]
    pub next_cycle_time: u64,
    /// Connected nodes
    #[prost(message, repeated, tag = "8")]
    pub connected_nodes: ::prost::alloc::vec::Vec<ConnectedNode>,
    /// Latest slot, none if now is before genesis timestamp
    #[prost(message, optional, tag = "9")]
    pub last_slot: ::core::option::Option<Slot>,
    /// Next slot
    #[prost(message, optional, tag = "10")]
    pub next_slot: ::core::option::Option<Slot>,
    /// Consensus stats
    #[prost(message, optional, tag = "11")]
    pub consensus_stats: ::core::option::Option<ConsensusStats>,
    /// Pool stats (operation count and endorsement count)
    #[prost(message, optional, tag = "12")]
    pub pool_stats: ::core::option::Option<PoolStats>,
    /// Network stats
    #[prost(message, optional, tag = "13")]
    pub network_stats: ::core::option::Option<NetworkStats>,
    /// Execution stats
    #[prost(message, optional, tag = "14")]
    pub execution_stats: ::core::option::Option<ExecutionStats>,
    /// Compact configuration
    #[prost(message, optional, tag = "15")]
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
    #[prost(uint64, tag = "1")]
    pub genesis_timestamp: u64,
    /// TESTNET: time when the blockclique is ended.
    #[prost(uint64, tag = "2")]
    pub end_timestamp: u64,
    /// Number of threads
    #[prost(uint32, tag = "3")]
    pub thread_count: u32,
    /// Time between the periods in the same thread.
    #[prost(uint64, tag = "4")]
    pub t0: u64,
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
    #[prost(uint64, tag = "8")]
    pub block_reward: u64,
    /// Price of a roll on the network
    #[prost(uint64, tag = "9")]
    pub roll_price: u64,
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
    #[prost(uint64, tag = "4")]
    pub current_time: u64,
    /// Current cycle
    #[prost(uint64, tag = "5")]
    pub current_cycle: u64,
    /// Current cycle starting timestamp
    #[prost(uint64, tag = "6")]
    pub current_cycle_time: u64,
    /// Next cycle starting timestamp
    #[prost(uint64, tag = "7")]
    pub next_cycle_time: u64,
    /// Latest slot, none if now is before genesis timestamp
    #[prost(message, optional, tag = "9")]
    pub last_slot: ::core::option::Option<Slot>,
    /// Next slot
    #[prost(message, optional, tag = "10")]
    pub next_slot: ::core::option::Option<Slot>,
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
/// Native Signature
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeSig {
    /// Version of the signature
    #[prost(fixed64, tag = "1")]
    pub version: u64,
    /// Content of the signature
    #[prost(bytes = "vec", tag = "2")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
/// Native Public Key
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativePubKey {
    /// Version of the public key
    #[prost(fixed64, tag = "1")]
    pub version: u64,
    /// Content of the public key
    #[prost(bytes = "vec", tag = "2")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
/// NativeTime represents a native duration or unix timestamp
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeTime {
    /// Milliseconds
    #[prost(uint64, tag = "1")]
    pub milliseconds: u64,
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
    #[prost(uint64, tag = "3")]
    pub start: u64,
    /// A timestamp at the which the deployment is considered failed
    #[prost(uint64, tag = "4")]
    pub timeout: u64,
    /// Once deployment has been locked, wait for this duration before deployment is considered active
    #[prost(uint64, tag = "5")]
    pub activation_delay: u64,
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
