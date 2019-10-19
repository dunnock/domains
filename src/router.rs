#[allow(dead_code)]
use actix_files as fs;
use actix_web::{error, web, App, HttpRequest, HttpResponse, Scope};

use crate::index;
use crate::models::db::ConnDsl;
use crate::share::common::AppState;
use crate::user_api::auth::login;
use actix::Addr;
use actix_web::error::JsonPayloadError;
use serde::private::de::IdentifierDeserializer;
use serde_derive::{Deserialize, Serialize};
use std::error::Error;

pub fn user_api_scope(path: &str) -> Scope {
    web::scope(path)
        .data(web::JsonConfig::default().limit(4096))
        .service(
            web::resource("/login")
                .data(
                    web::JsonConfig::default()
                        .limit(1024)
                        .error_handler(json_error_handler),
                )
                .route(web::post().to_async(login)),
        )
    // .service(web::resource("/path2").to_async(|| HttpResponse::Ok()))
    // .service(web::resource("/path3").to_async(|| HttpResponse::MethodNotAllowed()))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseError {
    pub msg: String,
}

fn json_error_handler(err: JsonPayloadError, req: &HttpRequest) -> actix_web::Error {
    dbg!(&err);
    if let JsonPayloadError::Deserialize(error) = &err {
        dbg!(error);
    };
    error::InternalError::from_response(
        err,
        HttpResponse::BadRequest().json(ResponseError {
            msg: String::from("error messsage"),
        }),
    )
    .into()
}