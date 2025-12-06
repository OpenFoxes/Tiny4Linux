// SPDX-License-Identifier: EUPL-1.2
use rust_i18n::i18n;
i18n!("src/locales", fallback = "en");

mod camera;
mod errors;

mod usbio;
pub use camera::*;
