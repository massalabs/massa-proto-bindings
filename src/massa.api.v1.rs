/// GetBlocksRequest holds request for GetBlocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlocksRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Queries
    #[prost(message, repeated, tag = "2")]
    pub queries: ::prost::alloc::vec::Vec<GetBlocksQuery>,
}
/// GetBlocks Query
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlocksQuery {
    /// Filter
    #[prost(message, optional, tag = "1")]
    pub filter: ::core::option::Option<GetBlocksFilter>,
}
/// GetBlocks Filter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlocksFilter {
    /// Block id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// GetBlocksResponse holds response from GetBlocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlocksResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Context
    #[prost(message, optional, tag = "2")]
    pub context: ::core::option::Option<BlocksContext>,
    /// Blocks wrappers
    #[prost(message, repeated, tag = "3")]
    pub blocks: ::prost::alloc::vec::Vec<super::super::model::v1::BlockWrapper>,
}
/// Blocks context
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlocksContext {
    /// Slot
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<super::super::model::v1::Slot>,
}
/// GetBlocksBySlotsRequest holds request for GetBlocksBySlots
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlocksBySlotsRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Slots
    #[prost(message, repeated, tag = "2")]
    pub slots: ::prost::alloc::vec::Vec<super::super::model::v1::Slot>,
}
/// GetBlocksBySlotsResponse holds response from GetBlocksBySlots
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlocksBySlotsResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Blocks
    #[prost(message, repeated, tag = "2")]
    pub blocks: ::prost::alloc::vec::Vec<super::super::model::v1::Block>,
}
/// GetDatastoreEntriesRequest holds request from GetDatastoreEntries
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatastoreEntriesRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Queries
    #[prost(message, repeated, tag = "2")]
    pub queries: ::prost::alloc::vec::Vec<DatastoreEntriesQuery>,
}
/// DatastoreEntries Query
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatastoreEntriesQuery {
    /// Filter
    #[prost(message, optional, tag = "1")]
    pub filter: ::core::option::Option<DatastoreEntryFilter>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatastoreEntryFilter {
    /// / Associated address of the entry
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Datastore key
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
/// GetDatastoreEntriesResponse holds response from GetDatastoreEntries
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatastoreEntriesResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Datastore entries
    #[prost(message, repeated, tag = "2")]
    pub entries: ::prost::alloc::vec::Vec<DatastoreEntry>,
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
/// GetLargestStakersRequest holds request from GetLargestStakers
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLargestStakersRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Query
    #[prost(message, optional, tag = "2")]
    pub query: ::core::option::Option<LargestStakersQuery>,
}
/// LargestStakers Query
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LargestStakersQuery {
    /// Starting offset for the list of stakers. Defaults to 1
    #[prost(fixed64, tag = "1")]
    pub offset: u64,
    /// Limits the number of stakers to return. Defaults to 50
    #[prost(fixed64, tag = "2")]
    pub limit: u64,
    /// Filter
    #[prost(message, optional, tag = "3")]
    pub filter: ::core::option::Option<LargestStakersFilter>,
}
/// LargestStakers Filter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LargestStakersFilter {
    /// Minimum rolls (Optional)
    #[prost(fixed64, optional, tag = "1")]
    pub min_rolls: ::core::option::Option<u64>,
    /// Maximum rolls (Optional)
    #[prost(fixed64, optional, tag = "2")]
    pub max_rolls: ::core::option::Option<u64>,
}
/// GetLargestStakersResponse holds response from GetLargestStakers
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLargestStakersResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Context
    #[prost(message, optional, tag = "2")]
    pub context: ::core::option::Option<LargestStakersContext>,
    /// Largest stakers
    #[prost(message, repeated, tag = "3")]
    pub stakers: ::prost::alloc::vec::Vec<LargestStakerEntry>,
}
/// LargestStakers context
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LargestStakersContext {
    /// Slot
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<super::super::model::v1::Slot>,
}
/// LargestStakerEntry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LargestStakerEntry {
    /// Address
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Rolls
    #[prost(fixed64, tag = "2")]
    pub rolls: u64,
}
/// GetNextBlockBestParentsRequest holds request for GetNextBlockBestParents
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNextBlockBestParentsRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// GetNextBlockBestParentsResponse holds response from GetNextBlockBestParents
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNextBlockBestParentsResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Best parents
    #[prost(message, repeated, tag = "2")]
    pub parents: ::prost::alloc::vec::Vec<BlockParent>,
}
/// Block parent tuple
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockParent {
    /// Block id
    #[prost(string, tag = "1")]
    pub block_id: ::prost::alloc::string::String,
    /// Period
    #[prost(fixed64, tag = "2")]
    pub period: u64,
}
/// GetOperationsRequest holds request for GetOperations
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOperationsRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Queries
    #[prost(message, repeated, tag = "2")]
    pub queries: ::prost::alloc::vec::Vec<GetOperationsQuery>,
}
/// GetOperations Query
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOperationsQuery {
    /// Filter
    #[prost(message, optional, tag = "1")]
    pub filter: ::core::option::Option<GetOperationsFilter>,
}
/// GetOperations Filter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOperationsFilter {
    /// Operation id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// GetOperationsResponse holds response from GetOperations
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOperationsResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Context
    #[prost(message, optional, tag = "2")]
    pub context: ::core::option::Option<OperationsContext>,
    /// Operations wrappers
    #[prost(message, repeated, tag = "3")]
    pub operations: ::prost::alloc::vec::Vec<super::super::model::v1::OperationWrapper>,
}
/// Operations context
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationsContext {
    /// Slot
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<super::super::model::v1::Slot>,
}
/// GetScExecutionEventsRequest holds request for GetScExecutionEvents
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetScExecutionEventsRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Query
    #[prost(message, optional, tag = "2")]
    pub query: ::core::option::Option<GetScExecutionEventsQuery>,
}
/// GetScExecutionEvents Query
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetScExecutionEventsQuery {
    /// Filter
    #[prost(message, optional, tag = "1")]
    pub filter: ::core::option::Option<GetScExecutionEventsFilter>,
}
/// GetScExecutionEvents Filter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetScExecutionEventsFilter {
    /// Start slot (Optional)
    #[prost(message, optional, tag = "1")]
    pub start_slot: ::core::option::Option<super::super::model::v1::Slot>,
    /// End slot (Optional)
    #[prost(message, optional, tag = "2")]
    pub end_slot: ::core::option::Option<super::super::model::v1::Slot>,
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
    #[prost(
        enumeration = "super::super::model::v1::ScExecutionEventStatus",
        repeated,
        tag = "6"
    )]
    pub status: ::prost::alloc::vec::Vec<i32>,
}
/// GetScExecutionEventsResponse holds response from GetScExecutionEvents
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetScExecutionEventsResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Context
    #[prost(message, optional, tag = "2")]
    pub context: ::core::option::Option<GetScExecutionEventsContext>,
    /// ScExecutionEvents
    #[prost(message, repeated, tag = "3")]
    pub events: ::prost::alloc::vec::Vec<super::super::model::v1::ScExecutionEvent>,
}
/// ScExecutionEvents context
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetScExecutionEventsContext {
    /// Slot
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<super::super::model::v1::Slot>,
}
/// GetSelectorDrawsRequest holds request from GetSelectorDraws
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSelectorDrawsRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Queries
    #[prost(message, repeated, tag = "2")]
    pub queries: ::prost::alloc::vec::Vec<SelectorDrawsQuery>,
}
/// SelectorDraws Query
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelectorDrawsQuery {
    /// Filter
    #[prost(message, optional, tag = "1")]
    pub filter: ::core::option::Option<SelectorDrawsFilter>,
}
/// SelectorDraws Filter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelectorDrawsFilter {
    /// Address
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// GetSelectorDrawsResponse holds response from GetSelectorDraws
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSelectorDrawsResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Selector draws
    #[prost(message, repeated, tag = "2")]
    pub selector_draws: ::prost::alloc::vec::Vec<super::super::model::v1::SelectorDraws>,
}
/// GetTransactionsThroughputRequest holds request for GetTransactionsThroughput
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionsThroughputRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// GetTransactionsThroughputResponse holds response from GetTransactionsThroughput
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionsThroughputResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Transactions throughput
    #[prost(fixed32, tag = "2")]
    pub throughput: u32,
}
/// GetVersionRequest holds request from GetVersion
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVersionRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// GetVersionResponse holds response from GetVersion
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVersionResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Version
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
/// NewBlocksRequest holds request for NewBlocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewBlocksRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// NewBlocksResponse holds response from NewBlocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewBlocksResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Signed block
    #[prost(message, optional, tag = "2")]
    pub block: ::core::option::Option<super::super::model::v1::SignedBlock>,
}
/// NewBlocksHeadersRequest holds request for NewBlocksHeaders
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewBlocksHeadersRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// NewBlocksHeadersResponse holds response from NewBlocksHeaders
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewBlocksHeadersResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Signed block header
    #[prost(message, optional, tag = "2")]
    pub block_header: ::core::option::Option<super::super::model::v1::SignedBlockHeader>,
}
/// NewEndorsementsRequest holds request for NewEndorsements
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEndorsementsRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// NewEndorsementsResponse holds response from NewEndorsements
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEndorsementsResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Signed endorsement
    #[prost(message, optional, tag = "2")]
    pub endorsement: ::core::option::Option<super::super::model::v1::SignedEndorsement>,
}
/// NewFilledBlocksRequest holds request for NewFilledBlocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewFilledBlocksRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// NewFilledBlocksResponse holds response from NewFilledBlocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewFilledBlocksResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Block with operations content
    #[prost(message, optional, tag = "2")]
    pub filled_block: ::core::option::Option<super::super::model::v1::FilledBlock>,
}
/// NewOperationsRequest holds request for NewOperations
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewOperationsRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Query
    #[prost(message, optional, tag = "2")]
    pub query: ::core::option::Option<NewOperationsQuery>,
}
/// NewOperations Query
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewOperationsQuery {
    /// Filter
    #[prost(message, optional, tag = "1")]
    pub filter: ::core::option::Option<NewOperationsFilter>,
}
/// NewOperations Filter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewOperationsFilter {
    /// Operation type enum
    #[prost(enumeration = "OpType", repeated, tag = "1")]
    pub types: ::prost::alloc::vec::Vec<i32>,
}
/// NewOperationsResponse holds response from NewOperations
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewOperationsResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Signed operation
    #[prost(message, optional, tag = "2")]
    pub operation: ::core::option::Option<super::super::model::v1::SignedOperation>,
}
/// NewSlotExecutionOutputsRequest holds request for NewSlotExecutionOutputs
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSlotExecutionOutputsRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Query
    #[prost(message, optional, tag = "2")]
    pub query: ::core::option::Option<NewSlotExecutionOutputsQuery>,
}
/// NewSlotExecutionOutputs Query
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSlotExecutionOutputsQuery {
    /// Filter
    #[prost(message, optional, tag = "1")]
    pub filter: ::core::option::Option<NewSlotExecutionOutputsFilter>,
}
/// NewSlotExecutionOutputs Filter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSlotExecutionOutputsFilter {
    /// Execution output status enum
    #[prost(
        enumeration = "super::super::model::v1::ExecutionOutputStatus",
        repeated,
        tag = "1"
    )]
    pub status: ::prost::alloc::vec::Vec<i32>,
}
/// NewSlotExecutionOutputsResponse holds response from NewSlotExecutionOutputs
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSlotExecutionOutputsResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Slot execution output
    #[prost(message, optional, tag = "2")]
    pub output: ::core::option::Option<super::super::model::v1::SlotExecutionOutput>,
}
/// SendBlocksRequest holds parameters to SendBlocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendBlocksRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Secure shared block
    #[prost(message, optional, tag = "2")]
    pub block: ::core::option::Option<super::super::model::v1::SecureShare>,
}
/// SendBlocksResponse holds response from SendBlocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendBlocksResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Block result or a gRPC status
    #[prost(oneof = "send_blocks_response::Message", tags = "2, 3")]
    pub message: ::core::option::Option<send_blocks_response::Message>,
}
/// Nested message and enum types in `SendBlocksResponse`.
pub mod send_blocks_response {
    /// Block result or a gRPC status
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// Block result
        #[prost(message, tag = "2")]
        Result(super::BlockResult),
        /// gRPC error(status)
        #[prost(message, tag = "3")]
        Error(super::super::super::super::google::rpc::Status),
    }
}
/// Holds Block response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockResult {
    /// Block id
    #[prost(string, tag = "1")]
    pub block_id: ::prost::alloc::string::String,
}
/// SendEndorsementsRequest holds parameters to SendEndorsements
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendEndorsementsRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Secure shared endorsements
    #[prost(message, repeated, tag = "2")]
    pub endorsements: ::prost::alloc::vec::Vec<super::super::model::v1::SecureShare>,
}
/// SendEndorsementsResponse holds response from SendEndorsements
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendEndorsementsResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Endorsement result or gRPC status
    #[prost(oneof = "send_endorsements_response::Message", tags = "2, 3")]
    pub message: ::core::option::Option<send_endorsements_response::Message>,
}
/// Nested message and enum types in `SendEndorsementsResponse`.
pub mod send_endorsements_response {
    /// Endorsement result or gRPC status
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// Endorsement result
        #[prost(message, tag = "2")]
        Result(super::EndorsementResult),
        /// gRPC error(status)
        #[prost(message, tag = "3")]
        Error(super::super::super::super::google::rpc::Status),
    }
}
/// Holds Endorsement response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndorsementResult {
    /// Endorsements ids
    #[prost(string, repeated, tag = "1")]
    pub endorsements_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// SendOperationsRequest holds parameters to SendOperations
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendOperationsRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Secured shared operations
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<super::super::model::v1::SecureShare>,
}
/// SendOperationsResponse holds response from SendOperations
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendOperationsResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Operation result or gRPC status
    #[prost(oneof = "send_operations_response::Message", tags = "2, 3")]
    pub message: ::core::option::Option<send_operations_response::Message>,
}
/// Nested message and enum types in `SendOperationsResponse`.
pub mod send_operations_response {
    /// Operation result or gRPC status
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// Operation result
        #[prost(message, tag = "2")]
        Result(super::OperationResult),
        /// gRPC error(status)
        #[prost(message, tag = "3")]
        Error(super::super::super::super::google::rpc::Status),
    }
}
/// Holds Operation response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationResult {
    /// Operations ids
    #[prost(string, repeated, tag = "1")]
    pub operations_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// TransactionsThroughputRequest holds request for TransactionsThroughput
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionsThroughputRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Timer interval in seconds (Optional). Defaults to 10s
    #[prost(fixed64, optional, tag = "2")]
    pub interval: ::core::option::Option<u64>,
}
/// TransactionsThroughputResponse holds response from TransactionsThroughput
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionsThroughputResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Transactions throughput
    #[prost(fixed32, tag = "2")]
    pub throughput: u32,
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
/// Generated client implementations.
pub mod massa_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Massa gRPC service
    #[derive(Debug, Clone)]
    pub struct MassaServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MassaServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MassaServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MassaServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            MassaServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Get blocks by ids
        pub async fn get_blocks(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlocksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBlocksResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/GetBlocks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.MassaService", "GetBlocks"));
            self.inner.unary(req, path, codec).await
        }
        /// Get blocks by slots
        pub async fn get_blocks_by_slots(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlocksBySlotsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBlocksBySlotsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/GetBlocksBySlots",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("massa.api.v1.MassaService", "GetBlocksBySlots"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get datastore entries
        pub async fn get_datastore_entries(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDatastoreEntriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDatastoreEntriesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/GetDatastoreEntries",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("massa.api.v1.MassaService", "GetDatastoreEntries"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get largest stakers
        pub async fn get_largest_stakers(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLargestStakersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetLargestStakersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/GetLargestStakers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("massa.api.v1.MassaService", "GetLargestStakers"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get next block best parents
        pub async fn get_next_block_best_parents(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNextBlockBestParentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetNextBlockBestParentsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/GetNextBlockBestParents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "massa.api.v1.MassaService",
                        "GetNextBlockBestParents",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get operations
        pub async fn get_operations(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOperationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOperationsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/GetOperations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.MassaService", "GetOperations"));
            self.inner.unary(req, path, codec).await
        }
        /// Get smart contracts execution events
        pub async fn get_sc_execution_events(
            &mut self,
            request: impl tonic::IntoRequest<super::GetScExecutionEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetScExecutionEventsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/GetScExecutionEvents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("massa.api.v1.MassaService", "GetScExecutionEvents"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get selector draws
        pub async fn get_selector_draws(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSelectorDrawsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetSelectorDrawsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/GetSelectorDraws",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("massa.api.v1.MassaService", "GetSelectorDraws"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get transactions throughput
        pub async fn get_transactions_throughput(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransactionsThroughputRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTransactionsThroughputResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/GetTransactionsThroughput",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "massa.api.v1.MassaService",
                        "GetTransactionsThroughput",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get node version
        pub async fn get_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetVersionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/GetVersion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.MassaService", "GetVersion"));
            self.inner.unary(req, path, codec).await
        }
        /// New received and produced blocks
        pub async fn new_blocks(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::NewBlocksRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::NewBlocksResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/NewBlocks",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.MassaService", "NewBlocks"));
            self.inner.streaming(req, path, codec).await
        }
        /// New received and produced blocks headers
        pub async fn new_blocks_headers(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::NewBlocksHeadersRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::NewBlocksHeadersResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/NewBlocksHeaders",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("massa.api.v1.MassaService", "NewBlocksHeaders"),
                );
            self.inner.streaming(req, path, codec).await
        }
        /// New received and produced endorsements
        pub async fn new_endorsements(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::NewEndorsementsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::NewEndorsementsResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/NewEndorsements",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.MassaService", "NewEndorsements"));
            self.inner.streaming(req, path, codec).await
        }
        /// New received and produced blocks with operations
        pub async fn new_filled_blocks(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::NewFilledBlocksRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::NewFilledBlocksResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/NewFilledBlocks",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.MassaService", "NewFilledBlocks"));
            self.inner.streaming(req, path, codec).await
        }
        /// New received and produced operations
        pub async fn new_operations(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::NewOperationsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::NewOperationsResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/NewOperations",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.MassaService", "NewOperations"));
            self.inner.streaming(req, path, codec).await
        }
        /// New received and slot execution events
        pub async fn new_slot_execution_outputs(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::NewSlotExecutionOutputsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::NewSlotExecutionOutputsResponse>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/NewSlotExecutionOutputs",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "massa.api.v1.MassaService",
                        "NewSlotExecutionOutputs",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
        /// Send blocks
        pub async fn send_blocks(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::SendBlocksRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::SendBlocksResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/SendBlocks",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.MassaService", "SendBlocks"));
            self.inner.streaming(req, path, codec).await
        }
        /// Send endorsements
        pub async fn send_endorsements(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::SendEndorsementsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::SendEndorsementsResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/SendEndorsements",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("massa.api.v1.MassaService", "SendEndorsements"),
                );
            self.inner.streaming(req, path, codec).await
        }
        /// Send operations
        pub async fn send_operations(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::SendOperationsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::SendOperationsResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/SendOperations",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.MassaService", "SendOperations"));
            self.inner.streaming(req, path, codec).await
        }
        /// Transactions throughput
        pub async fn transactions_throughput(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::TransactionsThroughputRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::TransactionsThroughputResponse>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/TransactionsThroughput",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "massa.api.v1.MassaService",
                        "TransactionsThroughput",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod massa_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MassaServiceServer.
    #[async_trait]
    pub trait MassaService: Send + Sync + 'static {
        /// Get blocks by ids
        async fn get_blocks(
            &self,
            request: tonic::Request<super::GetBlocksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBlocksResponse>,
            tonic::Status,
        >;
        /// Get blocks by slots
        async fn get_blocks_by_slots(
            &self,
            request: tonic::Request<super::GetBlocksBySlotsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBlocksBySlotsResponse>,
            tonic::Status,
        >;
        /// Get datastore entries
        async fn get_datastore_entries(
            &self,
            request: tonic::Request<super::GetDatastoreEntriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDatastoreEntriesResponse>,
            tonic::Status,
        >;
        /// Get largest stakers
        async fn get_largest_stakers(
            &self,
            request: tonic::Request<super::GetLargestStakersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetLargestStakersResponse>,
            tonic::Status,
        >;
        /// Get next block best parents
        async fn get_next_block_best_parents(
            &self,
            request: tonic::Request<super::GetNextBlockBestParentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetNextBlockBestParentsResponse>,
            tonic::Status,
        >;
        /// Get operations
        async fn get_operations(
            &self,
            request: tonic::Request<super::GetOperationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOperationsResponse>,
            tonic::Status,
        >;
        /// Get smart contracts execution events
        async fn get_sc_execution_events(
            &self,
            request: tonic::Request<super::GetScExecutionEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetScExecutionEventsResponse>,
            tonic::Status,
        >;
        /// Get selector draws
        async fn get_selector_draws(
            &self,
            request: tonic::Request<super::GetSelectorDrawsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetSelectorDrawsResponse>,
            tonic::Status,
        >;
        /// Get transactions throughput
        async fn get_transactions_throughput(
            &self,
            request: tonic::Request<super::GetTransactionsThroughputRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTransactionsThroughputResponse>,
            tonic::Status,
        >;
        /// Get node version
        async fn get_version(
            &self,
            request: tonic::Request<super::GetVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetVersionResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the NewBlocks method.
        type NewBlocksStream: futures_core::Stream<
                Item = std::result::Result<super::NewBlocksResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// New received and produced blocks
        async fn new_blocks(
            &self,
            request: tonic::Request<tonic::Streaming<super::NewBlocksRequest>>,
        ) -> std::result::Result<tonic::Response<Self::NewBlocksStream>, tonic::Status>;
        /// Server streaming response type for the NewBlocksHeaders method.
        type NewBlocksHeadersStream: futures_core::Stream<
                Item = std::result::Result<
                    super::NewBlocksHeadersResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        /// New received and produced blocks headers
        async fn new_blocks_headers(
            &self,
            request: tonic::Request<tonic::Streaming<super::NewBlocksHeadersRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::NewBlocksHeadersStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the NewEndorsements method.
        type NewEndorsementsStream: futures_core::Stream<
                Item = std::result::Result<super::NewEndorsementsResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// New received and produced endorsements
        async fn new_endorsements(
            &self,
            request: tonic::Request<tonic::Streaming<super::NewEndorsementsRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::NewEndorsementsStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the NewFilledBlocks method.
        type NewFilledBlocksStream: futures_core::Stream<
                Item = std::result::Result<super::NewFilledBlocksResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// New received and produced blocks with operations
        async fn new_filled_blocks(
            &self,
            request: tonic::Request<tonic::Streaming<super::NewFilledBlocksRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::NewFilledBlocksStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the NewOperations method.
        type NewOperationsStream: futures_core::Stream<
                Item = std::result::Result<super::NewOperationsResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// New received and produced operations
        async fn new_operations(
            &self,
            request: tonic::Request<tonic::Streaming<super::NewOperationsRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::NewOperationsStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the NewSlotExecutionOutputs method.
        type NewSlotExecutionOutputsStream: futures_core::Stream<
                Item = std::result::Result<
                    super::NewSlotExecutionOutputsResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        /// New received and slot execution events
        async fn new_slot_execution_outputs(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::NewSlotExecutionOutputsRequest>,
            >,
        ) -> std::result::Result<
            tonic::Response<Self::NewSlotExecutionOutputsStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the SendBlocks method.
        type SendBlocksStream: futures_core::Stream<
                Item = std::result::Result<super::SendBlocksResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Send blocks
        async fn send_blocks(
            &self,
            request: tonic::Request<tonic::Streaming<super::SendBlocksRequest>>,
        ) -> std::result::Result<tonic::Response<Self::SendBlocksStream>, tonic::Status>;
        /// Server streaming response type for the SendEndorsements method.
        type SendEndorsementsStream: futures_core::Stream<
                Item = std::result::Result<
                    super::SendEndorsementsResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        /// Send endorsements
        async fn send_endorsements(
            &self,
            request: tonic::Request<tonic::Streaming<super::SendEndorsementsRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::SendEndorsementsStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the SendOperations method.
        type SendOperationsStream: futures_core::Stream<
                Item = std::result::Result<super::SendOperationsResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Send operations
        async fn send_operations(
            &self,
            request: tonic::Request<tonic::Streaming<super::SendOperationsRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::SendOperationsStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the TransactionsThroughput method.
        type TransactionsThroughputStream: futures_core::Stream<
                Item = std::result::Result<
                    super::TransactionsThroughputResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        /// Transactions throughput
        async fn transactions_throughput(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::TransactionsThroughputRequest>,
            >,
        ) -> std::result::Result<
            tonic::Response<Self::TransactionsThroughputStream>,
            tonic::Status,
        >;
    }
    /// Massa gRPC service
    #[derive(Debug)]
    pub struct MassaServiceServer<T: MassaService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: MassaService> MassaServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MassaServiceServer<T>
    where
        T: MassaService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/massa.api.v1.MassaService/GetBlocks" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlocksSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::UnaryService<super::GetBlocksRequest>
                    for GetBlocksSvc<T> {
                        type Response = super::GetBlocksResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlocksRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_blocks(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlocksSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/GetBlocksBySlots" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlocksBySlotsSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::UnaryService<super::GetBlocksBySlotsRequest>
                    for GetBlocksBySlotsSvc<T> {
                        type Response = super::GetBlocksBySlotsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlocksBySlotsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_blocks_by_slots(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlocksBySlotsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/GetDatastoreEntries" => {
                    #[allow(non_camel_case_types)]
                    struct GetDatastoreEntriesSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::UnaryService<super::GetDatastoreEntriesRequest>
                    for GetDatastoreEntriesSvc<T> {
                        type Response = super::GetDatastoreEntriesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDatastoreEntriesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_datastore_entries(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDatastoreEntriesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/GetLargestStakers" => {
                    #[allow(non_camel_case_types)]
                    struct GetLargestStakersSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::UnaryService<super::GetLargestStakersRequest>
                    for GetLargestStakersSvc<T> {
                        type Response = super::GetLargestStakersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetLargestStakersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_largest_stakers(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetLargestStakersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/GetNextBlockBestParents" => {
                    #[allow(non_camel_case_types)]
                    struct GetNextBlockBestParentsSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::UnaryService<super::GetNextBlockBestParentsRequest>
                    for GetNextBlockBestParentsSvc<T> {
                        type Response = super::GetNextBlockBestParentsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetNextBlockBestParentsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_next_block_best_parents(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetNextBlockBestParentsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/GetOperations" => {
                    #[allow(non_camel_case_types)]
                    struct GetOperationsSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::UnaryService<super::GetOperationsRequest>
                    for GetOperationsSvc<T> {
                        type Response = super::GetOperationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetOperationsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_operations(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetOperationsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/GetScExecutionEvents" => {
                    #[allow(non_camel_case_types)]
                    struct GetScExecutionEventsSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::UnaryService<super::GetScExecutionEventsRequest>
                    for GetScExecutionEventsSvc<T> {
                        type Response = super::GetScExecutionEventsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetScExecutionEventsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_sc_execution_events(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetScExecutionEventsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/GetSelectorDraws" => {
                    #[allow(non_camel_case_types)]
                    struct GetSelectorDrawsSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::UnaryService<super::GetSelectorDrawsRequest>
                    for GetSelectorDrawsSvc<T> {
                        type Response = super::GetSelectorDrawsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetSelectorDrawsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_selector_draws(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSelectorDrawsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/GetTransactionsThroughput" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionsThroughputSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::UnaryService<
                        super::GetTransactionsThroughputRequest,
                    > for GetTransactionsThroughputSvc<T> {
                        type Response = super::GetTransactionsThroughputResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetTransactionsThroughputRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_transactions_throughput(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionsThroughputSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/GetVersion" => {
                    #[allow(non_camel_case_types)]
                    struct GetVersionSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::UnaryService<super::GetVersionRequest>
                    for GetVersionSvc<T> {
                        type Response = super::GetVersionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetVersionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_version(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetVersionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/NewBlocks" => {
                    #[allow(non_camel_case_types)]
                    struct NewBlocksSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::StreamingService<super::NewBlocksRequest>
                    for NewBlocksSvc<T> {
                        type Response = super::NewBlocksResponse;
                        type ResponseStream = T::NewBlocksStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::NewBlocksRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).new_blocks(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewBlocksSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/NewBlocksHeaders" => {
                    #[allow(non_camel_case_types)]
                    struct NewBlocksHeadersSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::StreamingService<super::NewBlocksHeadersRequest>
                    for NewBlocksHeadersSvc<T> {
                        type Response = super::NewBlocksHeadersResponse;
                        type ResponseStream = T::NewBlocksHeadersStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::NewBlocksHeadersRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).new_blocks_headers(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewBlocksHeadersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/NewEndorsements" => {
                    #[allow(non_camel_case_types)]
                    struct NewEndorsementsSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::StreamingService<super::NewEndorsementsRequest>
                    for NewEndorsementsSvc<T> {
                        type Response = super::NewEndorsementsResponse;
                        type ResponseStream = T::NewEndorsementsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::NewEndorsementsRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).new_endorsements(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewEndorsementsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/NewFilledBlocks" => {
                    #[allow(non_camel_case_types)]
                    struct NewFilledBlocksSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::StreamingService<super::NewFilledBlocksRequest>
                    for NewFilledBlocksSvc<T> {
                        type Response = super::NewFilledBlocksResponse;
                        type ResponseStream = T::NewFilledBlocksStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::NewFilledBlocksRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).new_filled_blocks(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewFilledBlocksSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/NewOperations" => {
                    #[allow(non_camel_case_types)]
                    struct NewOperationsSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::StreamingService<super::NewOperationsRequest>
                    for NewOperationsSvc<T> {
                        type Response = super::NewOperationsResponse;
                        type ResponseStream = T::NewOperationsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::NewOperationsRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).new_operations(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewOperationsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/NewSlotExecutionOutputs" => {
                    #[allow(non_camel_case_types)]
                    struct NewSlotExecutionOutputsSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::StreamingService<
                        super::NewSlotExecutionOutputsRequest,
                    > for NewSlotExecutionOutputsSvc<T> {
                        type Response = super::NewSlotExecutionOutputsResponse;
                        type ResponseStream = T::NewSlotExecutionOutputsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::NewSlotExecutionOutputsRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).new_slot_execution_outputs(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewSlotExecutionOutputsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/SendBlocks" => {
                    #[allow(non_camel_case_types)]
                    struct SendBlocksSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::StreamingService<super::SendBlocksRequest>
                    for SendBlocksSvc<T> {
                        type Response = super::SendBlocksResponse;
                        type ResponseStream = T::SendBlocksStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::SendBlocksRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).send_blocks(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendBlocksSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/SendEndorsements" => {
                    #[allow(non_camel_case_types)]
                    struct SendEndorsementsSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::StreamingService<super::SendEndorsementsRequest>
                    for SendEndorsementsSvc<T> {
                        type Response = super::SendEndorsementsResponse;
                        type ResponseStream = T::SendEndorsementsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::SendEndorsementsRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).send_endorsements(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendEndorsementsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/SendOperations" => {
                    #[allow(non_camel_case_types)]
                    struct SendOperationsSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::StreamingService<super::SendOperationsRequest>
                    for SendOperationsSvc<T> {
                        type Response = super::SendOperationsResponse;
                        type ResponseStream = T::SendOperationsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::SendOperationsRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).send_operations(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendOperationsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/TransactionsThroughput" => {
                    #[allow(non_camel_case_types)]
                    struct TransactionsThroughputSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::StreamingService<
                        super::TransactionsThroughputRequest,
                    > for TransactionsThroughputSvc<T> {
                        type Response = super::TransactionsThroughputResponse;
                        type ResponseStream = T::TransactionsThroughputStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::TransactionsThroughputRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).transactions_throughput(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TransactionsThroughputSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: MassaService> Clone for MassaServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: MassaService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: MassaService> tonic::server::NamedService for MassaServiceServer<T> {
        const NAME: &'static str = "massa.api.v1.MassaService";
    }
}
