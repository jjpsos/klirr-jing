use crate::prelude::*;

pub fn get_localization() -> Result<L18n> {
    debug!("☑️ Reading localisation data...");
    let l18n = deserialize_contents_of_ron(Path::new("./input/l18n/english.ron"))?;
    debug!("✅ Read localisation data!");
    Ok(l18n)
}
