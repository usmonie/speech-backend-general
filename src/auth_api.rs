use async_trait::async_trait;
use tonic::{Request, Response, Status};
use monie_rpc::monie::auth::authentication_api_server::AuthenticationApi;
use monie_rpc::monie::auth::{PublicKeyRequest, PublicKeyResponse, StartLoginConfirmCodesRequest, StartLoginConfirmCodesResponse, StartLoginRequest, StartLoginResponse, StartRegisterRequest, StartRegisterResponse};
use crate::sessions::{AddUserToSessionUseCase, create_session, CreateSessionUseCase, GenerateKeyPairUseCase, get_session, GetSessionUseCase};
use crate::users::login_user_via_username;

pub(crate) struct AuthenticationApiServer {
    generate_key_pair_use_case: Box<GenerateKeyPairUseCase>,
    create_session_use_case: Box<CreateSessionUseCase>,
    get_session_use_case: Box<GetSessionUseCase>,
    add_user_to_session_use_case: Box<AddUserToSessionUseCase>,
}

#[async_trait]
impl AuthenticationApi for AuthenticationApiServer {
    async fn create_new_session(&self, request: Request<PublicKeyRequest>) -> Result<Response<PublicKeyResponse>, Status> {
        return create_session(request, &*self.generate_key_pair_use_case, &*self.create_session_use_case).await;
    }

    async fn register_user(&self, request: Request<StartRegisterRequest>) -> Result<Response<StartRegisterResponse>, Status> {
        todo!()
    }


    async fn login_username(&self, request: Request<StartLoginRequest>) -> Result<Response<StartLoginResponse>, Status> {
        login_user_via_username(request, &*self.get_session_use_case).await
    }

    async fn login_confirm_codes(&self, request: Request<StartLoginConfirmCodesRequest>) -> Result<Response<StartLoginConfirmCodesResponse>, Status> {
        todo!()
    }
}