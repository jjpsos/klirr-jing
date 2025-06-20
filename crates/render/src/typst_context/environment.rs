use crate::prelude::*;

use typst::{Library, text::FontBook, utils::LazyHash};
use typst_kit::fonts::FontSearcher;

#[derive(Debug, Getters)]
pub struct Environment {
    #[getset(get = "pub")]
    library: LazyHash<Library>,

    #[getset(get = "pub")]
    book: LazyHash<FontBook>,

    #[getset(get = "pub")]
    fonts: Vec<typst_kit::fonts::FontSlot>,

    #[getset(get = "pub")]
    now: DateTime<Local>,
}

impl Default for Environment {
    fn default() -> Self {
        // Build the standard library (Typst definitions and styles).
        let lib = Library::builder().build();
        // Search for fonts (includes Typst default fonts if embed feature enabled).
        let fonts_data = FontSearcher::new().search();

        // Get the current local date and time
        let now = Local::now();

        Self {
            library: LazyHash::new(lib),
            book: LazyHash::new(fonts_data.book),
            fonts: fonts_data.fonts,
            now,
        }
    }
}
