use speech_backend_common::ApiResult;
use speech_backend_common::domain::UseCase;
use speech_backend_sessions::models::request::AddUserToSessionRequest;
use speech_backend_sessions::models::results::Session;
use speech_backend_user::models::request::SearchUserByUsernameRequest;
use speech_backend_user::models::results::User;
use tonic::{Request, Response, Status};
use monie_rpc::monie::auth::{StartLoginRequest, StartLoginResponse};
use crate::sessions::{get_session, GetSessionUseCase};

pub type SearchUserByUsernameUseCase = dyn UseCase<SearchUserByUsernameRequest, User> + Send + Sync;

pub(crate) async fn login_user_via_username(
    request: Request<StartLoginRequest>,
    get_session_use_case: &GetSessionUseCase,
) -> Result<Response<StartLoginResponse>, Status> {
    let ip_address = request.remote_addr().unwrap().ip();
    let start_login_req = request.into_inner();
    match get_session(
        start_login_req.session_id.clone(),
        ip_address,
        get_session_use_case,
    ).await {
        ApiResult::Ok(session) => {
            let username = String::from_utf8(
                enigma::decrypt_data(
                    start_login_req.username.as_slice(),
                    session.session_key.as_slice(),
                    &*start_login_req.nonce,
                )
            );
            let password = String::from_utf8(
                enigma::decrypt_data(
                    start_login_req.password.as_slice(),
                    session.session_key.as_slice(),
                    &*start_login_req.nonce,
                )
            );




            Ok(
                Response::new(
                    StartLoginResponse {
                        session_id: start_login_req.session_id,
                    }
                )
            )
        }
        ApiResult::Err(err) => { Err(Status::aborted(err.message)) }
    }
}