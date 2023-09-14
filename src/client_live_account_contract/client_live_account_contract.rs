#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "client-live-account-contract", label = "ClientLiveAccountContract")]
pub struct ClientLiveAccountContractUpdateSbModel{
    #[prost(message, tag = "1")]
    pub event: Option<ClientLiveAccountContractUpdateBodySbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientLiveAccountContractUpdateBodySbModel{
    #[prost(string, tag = "1")]
    pub client_id: String,
    #[prost(string, tag = "2")]
    pub trader_account_aggregate_id: String,
    #[prost(string, tag = "3")]
    pub contract_id: String,
    #[prost(enumeration="ContractStatus", tag = "4")]
    pub status: i32,
    #[prost(string, optional, tag = "5")]
    pub description: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContractStatus{
    Created = 0,
    SentToClient = 1,
    SignedByClientAndReceived = 2,
    GrantedLiveAccount = 3,
    Rejected = 4,
    RejectedAndBlocked = 5,
    Uploaded = 6,
    Downloaded = 7,
}