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

use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};

use anyhow::Result;
use futures_util::SinkExt;
use monrst_api::model::user::client::Client;
use tokio::net::TcpListener;
use tokio_websockets::ServerBuilder;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    /* let clients = Arc::new(Mutex::new(HashMap::<Uuid, Client>::new())); */

    let bind = env::var("HOST").unwrap_or_else(|_| "0.0.0.0:13009".to_owned());
    let listener = TcpListener::bind(bind).await?;

    while let Ok((stream, addr)) = listener.accept().await {
        println!("{:?} {}", stream, addr);
        tokio::spawn(async move {
            let mut ws_stream = match ServerBuilder::new().accept(stream).await {
                Ok(ws) => ws,
                Err(err) => {
                    eprintln!("{:?}", err);
                    return;
                },
            };
            while let Some(Ok(msg)) = ws_stream.next().await {
                println!("{:?}", msg);
                ws_stream.send(msg).await.expect("Failed to send the message");
            }
        });
    }

    Ok(())
}
