use crate::prelude::*;

use chrono::{DateTime, Datelike, FixedOffset, Local};
use invoice_typst_logic::prelude::TypedBuilder;
use std::path::Path;
use typst::{
    Library, World,
    foundations::{Bytes, Datetime},
    syntax::{FileId, Source, VirtualPath},
    text::{Font, FontBook},
    utils::LazyHash,
};
use typst_kit::fonts::FontSearcher;

#[derive(Debug, Getters)]
pub struct Environment {
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

#[derive(Debug, Getters, TypedBuilder)]
pub struct Content {
    #[getset(get = "pub")]
    data: Source,
    #[getset(get = "pub")]
    ui: Source,
}

#[derive(Debug, Getters)]
pub struct MinimalWorld {
    #[getset(get = "pub")]
    content: Content,
    #[getset(get = "pub")]
    environment: Environment,
}

trait LoadSource: Sized {
    fn load_source_at(path: &Path) -> Result<Self>;
}

trait InlineSource: Sized {
    fn inline(source_text: String, virtual_path: impl AsRef<Path>) -> Result<Self>;
}

impl InlineSource for Source {
    fn inline(source_text: String, virtual_path: impl AsRef<Path>) -> Result<Self> {
        // Create a new FileId for the virtual inline file ("/inline.typ").
        let file_id = FileId::new(None, VirtualPath::new(virtual_path));
        // Prepare the Typst source.
        Ok(Source::new(file_id, source_text))
    }
}

impl LoadSource for Source {
    fn load_source_at(path: &Path) -> Result<Self> {
        // Create a new FileId for the virtual main file ("/main.typ").
        let file_id = FileId::new(None, VirtualPath::new(path));
        // Read the Typst source from the file.
        let source_text = std::fs::read_to_string(path).map_err(|e| {
            let msg = format!(
                "Failed to read Typst source from {:?}, error: {:?}",
                path, e
            );
            error!("{}", msg);
            Error::LoadSource { underlying: msg }
        })?;
        // Prepare the Typst source.
        Ok(Source::new(file_id, source_text))
    }
}

impl MinimalWorld {
    fn new(ui: Source, data: Source) -> Self {
        let content = Content::builder().data(data).ui(ui).build();
        let environment = Environment::default();

        Self {
            content,
            environment,
        }
    }

    pub fn with_path(path_to_ui: &Path, data_inline: String) -> Result<Self> {
        Ok(Self::new(
            Source::load_source_at(path_to_ui)?,
            Source::inline(data_inline, "/crates/render/src/input.typ")?,
        ))
    }
}

impl World for MinimalWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.environment().library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.environment().book()
    }

    fn main(&self) -> FileId {
        self.content().ui().id()
    }

    fn source(&self, id: FileId) -> typst::diag::FileResult<Source> {
        if id == self.main() {
            let source = self.content().ui().clone();
            Ok(source)
        } else if id == self.content.data().id() {
            let source = self.content().data().clone();
            Ok(source)
        } else {
            panic!(
                "Only the main file is supported in this minimal example, got id: '{:?}' != {:?} (self.content.data.id())",
                id,
                self.content.data().id()
            );
        }
    }

    fn file(&self, _id: FileId) -> typst::diag::FileResult<Bytes> {
        panic!("File access not implemented in this minimal example")
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.environment().fonts.get(index)?.get()
    }

    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        let now = self.environment().now();
        let with_offset = match offset {
            None => now.with_timezone(now.offset()).fixed_offset(),
            Some(hours) => {
                let seconds = i32::try_from(hours).ok()?.checked_mul(3600)?;
                let fixed = FixedOffset::east_opt(seconds)?;
                now.with_timezone(&fixed)
            }
        };

        Datetime::from_ymd(
            with_offset.year(),
            with_offset.month().try_into().ok()?,
            with_offset.day().try_into().ok()?,
        )
    }
}
