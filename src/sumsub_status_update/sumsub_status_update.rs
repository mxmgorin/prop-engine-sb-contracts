#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "sumsub-status-update")]
pub struct SumsubUpdateSbModel {
    #[prost(message, tag = "1")]
    pub event: Option<SumsubUpdateBodySbModel>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SumsubReviewStatus {
    Init = 0,
    Pending = 1,
    Prechecked = 2,
    Queued =3,
    Completed = 4,
    OnHold = 5,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SumsubProofType {
    ProofOfAddress = 0,
    ProofOfIdentity = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SumsubNotificationType {
    Reviewed = 0,
    Pending = 1,
    Created = 2,
    OnHold = 3,
    PersonalInfoChanged = 4,
    Prechecked = 5,
    Deleted = 6,
    LevelChanged = 7,
    VideoIdentStatusChanged = 8,
    Reset = 9,
    ActionPending = 10,
    ActionReviewed = 11,
    ActionOnHold = 12,
    TravelRuleStatusChanged = 13,
    WorkflowCompleted = 14,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SumsubUpdateBodySbModel {
    #[prost(string, tag = "1")]
    pub client_id: String,
    #[prost(string, tag = "2")]
    pub verification_id: String,
    #[prost(string, tag = "3")]
    pub applicant_id: String,
    #[prost(int64, tag = "4")]
    pub created_at: i64,
    #[prost(enumeration="SumsubNotificationType", tag = "5")]
    pub notification_type: i32,
    #[prost(enumeration = "SumsubProofType", tag = "6")]
    pub proof_type: i32,
    #[prost(enumeration="SumsubReviewStatus", tag = "7")]
    pub review_status: i32
}


impl Into<SumsubProofType> for i32 {
    fn into(self) -> SumsubProofType {
        match &*self {
            0 => SumsubProofType::ProofOfAddress,
            1 => SumsubProofType::ProofOfIdentity,
            _ => panic!("Unknown SumsubProofType: {}", &*self),
        }
    }
}

impl Into<SumsubReviewStatus> for i32 {
    fn into(self) -> SumsubReviewStatus {
        match &*self {
            0 => SumsubReviewStatus::Init,
            1 => SumsubReviewStatus::Pending,
            2 => SumsubReviewStatus::Prechecked,
            3 => SumsubReviewStatus::Queued,
            4 => SumsubReviewStatus::Completed,
            5 => SumsubReviewStatus::OnHold,
            _ => panic!("Unknown SumsubReviewStatus: {}", &*self),
        }
    }
}

impl Into<SumsubNotificationType> for i32{
    fn into(self) -> SumsubNotificationType {
        match &*self {
            0 => SumsubNotificationType::Reviewed,
            1 => SumsubNotificationType::Pending,
            2 => SumsubNotificationType::Created,
            3 => SumsubNotificationType::OnHold,
            4 => SumsubNotificationType::PersonalInfoChanged,
            5 => SumsubNotificationType::Prechecked,
            6 => SumsubNotificationType::Deleted,
            7 => SumsubNotificationType::LevelChanged,
            8 => SumsubNotificationType::VideoIdentStatusChanged,
            9 => SumsubNotificationType::Reset,
            10 => SumsubNotificationType::ActionPending,
            11 => SumsubNotificationType::ActionReviewed,
            12 => SumsubNotificationType::ActionOnHold,
            13 => SumsubNotificationType::TravelRuleStatusChanged,
            14 => SumsubNotificationType::WorkflowCompleted,
            _ => panic!("Unknown SumsubNotificationType: {}", &*self),
        }
    }
}