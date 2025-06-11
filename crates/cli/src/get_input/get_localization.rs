use crate::prelude::*;

pub fn get_localization() -> Result<L18n> {
    info!("☑️ Reading localisation data...");
    L18n::load_from_ron_file(Path::new("./input/l18n/english.ron"))
}
