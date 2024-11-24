#![feature(async_closure)]
#![feature(duration_constructors)]

use crate::server::{server, setup};
use crate::server_error::ServerError;
use helper_macros::generate_endpoint;

mod auth;
mod config;
mod dto;
mod endpoints;
mod error;
mod extractors;
mod init_env;

#[cfg(not(test))]
mod logging;
mod middlewares;
mod models;
mod server;
mod server_error;
mod services;
mod state;
mod swagger;
mod utils;

#[actix::main]
async fn main() -> Result<(), ServerError> {
    setup().await?;

    let port = tosic_utils::prelude::env!("PORT", "9999");

    server!()
        .bind(format!("0.0.0.0:{port}"))?
        .bind(format!("[::1]:{port}"))?
        .run()
        .await?;

    Ok(())
}
