// SPDX-License-Identifier: EUPL-1.2

use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum T4lError {
    #[error("value of {1} is not supported for {0}")]
    UnsupportedIntValue(String, i32),
    #[error("USB IO error: {0}")]
    USBIOError(i32),
    #[error("IO error: {0}")]
    IOError(#[from] io::Error),
    #[error("no camera found")]
    NoCameraFound,
    #[error("Invalid setting")]
    InvalidSetting,
}
