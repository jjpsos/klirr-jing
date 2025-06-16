use crate::prelude::*;

pub fn get_localization() -> Result<L18n> {
    debug!("☑️ Reading localisation data...");
    let l18n = L18n::load_from_ron_file(Path::new("./input/l18n/english.ron"))?;
    debug!("✅ Read localisation data!");
    Ok(l18n)
}
