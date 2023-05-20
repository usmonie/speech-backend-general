use std::net::{IpAddr};
use num_bigint_dig::{BigInt, RandBigInt};
use speech_backend_common::ApiResult;
use speech_backend_common::domain::UseCase;
use speech_backend_sessions::models::request::{AddUserToSessionRequest, CreateSessionRequest, GetSessionRequest};
use speech_backend_sessions::models::results::{Device, Session};
use speech_encryption::models::requests::GenerateKeyPairRequest;
use speech_encryption::models::result::GenerateKeyPairResult;
use tonic::{Request, Response, Status};
use uuid::{Bytes, Uuid};
use monie_rpc::monie::auth::{PublicKeyRequest, PublicKeyResponse};

pub type GenerateKeyPairUseCase = dyn UseCase<GenerateKeyPairRequest, GenerateKeyPairResult> + Send + Sync;

pub type CreateSessionUseCase = dyn UseCase<CreateSessionRequest, Session> + Send + Sync;
pub type GetSessionUseCase = dyn UseCase<GetSessionRequest, Session> + Send + Sync;
pub type AddUserToSessionUseCase = dyn UseCase<AddUserToSessionRequest, Session> + Send + Sync;

pub(crate) async fn create_session(
    request: Request<PublicKeyRequest>,
    generate_key_pair_use_case: &GenerateKeyPairUseCase,
    create_session_use_case: &CreateSessionUseCase,
) -> Result<Response<PublicKeyResponse>, Status> {
    let ip_addr = request.remote_addr().unwrap().ip();
    let client_key = request.into_inner();
    let key_pair_result = generate_key_pair_use_case.execute(GenerateKeyPairRequest {
        x: BigInt::from_signed_bytes_be(client_key.x.as_slice()),
        y: BigInt::from_signed_bytes_be(client_key.y.as_slice()),
    }).await;

    let nonce = {
        let rand = rand::thread_rng().gen_biguint(64);
        rand.to_bytes_be()
    };
    return match key_pair_result {
        ApiResult::Ok(key_pair) => {
            match create_session_use_case.execute(
                CreateSessionRequest {
                    device: Device::PC { name: "Usman Test 12345".to_string() },
                    ip_addr,
                    session_key: key_pair.private_key,
                }
            ).await {
                ApiResult::Ok(session) => {
                    Ok(Response::new(PublicKeyResponse {
                        session_id: session.id.into_bytes().as_slice().to_vec(),
                        nonce,
                        x: key_pair.x.to_signed_bytes_be(),
                        y: key_pair.x.to_signed_bytes_be(),
                    }))
                }
                ApiResult::Err(err) => {  Err(Status::aborted(err.message)) }
            }
        }
        ApiResult::Err(err) => {  Err(Status::aborted(err.message))}
    };
}

pub(crate) async fn add_user_to_session(
    session_id: Uuid,
    latest_ip_address: IpAddr,
    user_id: Uuid,
    session_key: Vec<u8>,
    user_password_hash: Vec<u8>,
    add_user_to_session_use_case: &AddUserToSessionUseCase,
) {
    add_user_to_session_use_case.execute(
        AddUserToSessionRequest {
            session_id,
            latest_ip_address,
            user_id,
            session_key,
            user_password_hash,
        }
    ).await;
}

pub(crate) async fn get_session(
    session_id: Vec<u8>,
    latest_ip_address: IpAddr,
    get_session_use_case: &GetSessionUseCase) -> ApiResult<Session> {
    get_session_use_case.execute(GetSessionRequest {
        id: Uuid::from_bytes(Bytes::try_from(session_id.as_slice()).unwrap())
    }).await
}
