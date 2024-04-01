use crate::account_manager::AccountManager;
use crate::auth_verifier::Access;
use crate::models::{InternalErrorCode, InternalErrorMessageResponse};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rsky_lexicon::com::atproto::server::{AppPassword, ListAppPasswordsOutput};

#[rocket::get("/xrpc/com.atproto.server.listAppPasswords")]
pub async fn list_app_passwords(
    auth: Access,
) -> Result<Json<ListAppPasswordsOutput>, status::Custom<Json<InternalErrorMessageResponse>>> {
    let did = auth.access.credentials.unwrap().did.unwrap();
    match AccountManager::list_app_passwords(&did).await {
        Ok(passwords) => {
            let passwords: Vec<AppPassword> = passwords
                .into_iter()
                .map(|password| AppPassword {
                    name: password.0,
                    created_at: password.1,
                })
                .collect();
            Ok(Json(ListAppPasswordsOutput { passwords }))
        }
        Err(error) => {
            eprintln!("Internal Error: {error}");
            let internal_error = InternalErrorMessageResponse {
                code: Some(InternalErrorCode::InternalError),
                message: Some("Internal error".to_string()),
            };
            return Err(status::Custom(
                Status::InternalServerError,
                Json(internal_error),
            ));
        }
    }
}
