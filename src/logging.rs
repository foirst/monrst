//! Logger used for `monrst`

use std::path::PathBuf;

use anyhow::Result;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::json::JsonEncoder;
use log4rs::Config;

/// Initializes the logger in the given output file
///
/// In the output file, each line is a JSON expression.
pub fn init(output_file: PathBuf) -> Result<()> {
    let requests = FileAppender::builder().encoder(Box::new(JsonEncoder::new())).build(output_file)?;
    let config = Config::builder()
        .appender(Appender::builder().build("requests", Box::new(requests)))
        .build(Root::builder().appender("requests").build(LevelFilter::max()))?;
    log4rs::init_config(config)?;
    Ok(())
}
