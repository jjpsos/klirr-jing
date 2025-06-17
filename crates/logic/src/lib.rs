mod calendar_logic;
mod create_pdf;
mod error;
mod models;
mod pdf;
mod prepare_input_data;
mod read_data;
mod save_pdf_location_to_tmp_file;
mod serde_to_typst;

pub mod prelude {
    pub(crate) use crate::calendar_logic::*;
    pub use crate::create_pdf::*;
    pub use crate::error::*;
    pub use crate::models::*;
    pub use crate::pdf::*;
    pub use crate::prepare_input_data::*;
    pub(crate) use crate::read_data::*;
    pub use crate::save_pdf_location_to_tmp_file::*;
    pub use crate::serde_to_typst::*;

    pub use std::{
        collections::HashMap,
        path::{Path, PathBuf},
        str::FromStr,
    };

    pub use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveDateTime, Weekday};
    pub use derive_more::{AsRef, Deref, Display, From};
    pub use derive_more::{IsVariant, TryUnwrap};
    pub use getset::Getters;
    pub use indexmap::{IndexMap, IndexSet};
    pub use log::{debug, error, info, trace, warn};
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::Value;
    pub use serde_with::{DeserializeFromStr, SerializeDisplay};
    pub use thiserror::Error as ThisError;
    pub use typed_builder::TypedBuilder;
}
