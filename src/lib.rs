pub mod user_v1 {
    tonic::include_proto!("user.v1");
}

pub mod common_v1 {
    tonic::include_proto!("common.v1");
}

pub use user_v1::user_service_server::UserService;
pub use user_v1::user_service_server::UserServiceServer;

pub use user_v1::user_service_client::UserServiceClient;

pub use user_v1::UserSummary;

pub use user_v1::{
    CreateRandomNicknameUserRequest, CreateRandomNicknameUserResponse, FindUserByAuthRequest,
    FindUserByAuthResponse,
};

pub use common_v1::ErrorDetail;
