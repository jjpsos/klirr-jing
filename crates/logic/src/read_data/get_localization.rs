use crate::prelude::*;

pub fn get_localization(language: &Language) -> Result<L18n> {
    debug!("☑️ Reading localisation data...");
    let l18n = L18n::new(*language)?;
    debug!("✅ Read localisation data!");
    Ok(l18n)
}
