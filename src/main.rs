//! [Monrst], a simple server for [Foirst]

#![deny(
    clippy::complexity,
    clippy::correctness,
    clippy::nursery,
    clippy::pedantic,
    clippy::perf,
    clippy::restriction,
    clippy::style
)]
#![allow(
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::blanket_clippy_restriction_lints,
    clippy::else_if_without_else,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::expect_used,
    clippy::implicit_return,
    clippy::match_same_arms,
    clippy::match_wildcard_for_single_variants,
    clippy::missing_trait_methods,
    clippy::mod_module_files,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::pattern_type_mismatch,
    clippy::question_mark_used,
    clippy::separated_literal_suffix,
    clippy::shadow_reuse,
    clippy::shadow_unrelated,
    clippy::std_instead_of_core,
    clippy::unreachable,
    clippy::unwrap_in_result,
    clippy::wildcard_in_or_patterns,
    const_item_mutation
)]
#![cfg_attr(
    test,
    allow(
        clippy::assertions_on_result_states,
        clippy::collection_is_never_read,
        clippy::enum_glob_use,
        clippy::indexing_slicing,
        clippy::non_ascii_literal,
        clippy::too_many_lines,
        clippy::unwrap_used,
        clippy::wildcard_imports
    )
)]
#![feature(async_fn_in_trait)]

extern crate alloc;

mod client;
mod logging;

use alloc::sync::Arc;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use anyhow::Result;
use argh::FromArgs;
use chrono::Utc;
use log::{error, info, LevelFilter};
use monrst_api::protocol::VERSION;
use rustls::{Certificate, PrivateKey};
use rustls_pemfile::{certs, pkcs8_private_keys};
use tokio::io;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;

/// `foirst` server
#[derive(FromArgs)]
struct Options {
    /// cert file
    #[argh(option, short = 'c')]
    cert: PathBuf,

    /// key file
    #[argh(option, short = 'k')]
    key: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    logging::init(Path::new(&format!("logs/{}", Utc::now().format("%Y-%m-%d_%H-%M-%S.log"))).into())?;
    log::set_max_level(LevelFilter::Info);
    let options: Options = argh::from_env();

    info!("Starting monrst version {}", *VERSION);

    let bind = env::var("HOST").unwrap_or_else(|_| "0.0.0.0:13009".to_owned());
    let listener = TcpListener::bind(&bind).await?;
    info!("Listening on {}", bind);

    let certs = certs(&mut BufReader::new(File::open(&options.cert)?))
        .map_err(|_err| io::Error::new(io::ErrorKind::InvalidData, "invalid cert file"))
        .map(|keys| keys.into_iter().map(Certificate).collect::<Vec<Certificate>>())?;
    let mut keys = pkcs8_private_keys(&mut BufReader::new(File::open(&options.key)?))
        .map_err(|_err| io::Error::new(io::ErrorKind::InvalidData, "invalid key file"))
        .map(|keys| keys.into_iter().map(PrivateKey).collect::<Vec<PrivateKey>>())?;

    let config = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, keys.remove(0))?;
    let acceptor = TlsAcceptor::from(Arc::new(config));

    loop {
        let (stream, addr) = listener.accept().await?;
        let tls_stream = acceptor.clone().accept(stream).await?;

        tokio::spawn(async move {
            if let Err(err) = client::spawn(tls_stream, addr).await {
                error!("Error from client {}: {:?}", addr, err);
            }
        });
    }
}
