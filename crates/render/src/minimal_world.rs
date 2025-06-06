use chrono::DateTime;
use chrono::Datelike;
use chrono::FixedOffset;
use chrono::Local;
use std::path::Path;
use typst::Library;
use typst::World;
use typst::foundations::Bytes;
use typst::foundations::Datetime;
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst_kit::fonts::FontSearcher;

pub struct MinimalWorld {
    file_id: FileId,
    source: Source,
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    fonts: Vec<typst_kit::fonts::FontSlot>,
    now: DateTime<Local>,
}

impl MinimalWorld {
    fn new(source: Source, file_id: FileId) -> Self {
        // Build the standard library (Typst definitions and styles).
        let lib = Library::builder().build();
        // Search for fonts (includes Typst default fonts if embed feature enabled).
        let fonts_data = FontSearcher::new().search();
        // Get the current local date and time
        let now = Local::now();

        MinimalWorld {
            file_id,
            source,
            library: LazyHash::new(lib),
            book: LazyHash::new(fonts_data.book),
            fonts: fonts_data.fonts,
            now,
        }
    }

    pub fn with_path(path: &Path) -> Self {
        // Create a new FileId for the virtual main file ("/main.typ").
        let file_id = FileId::new(None, VirtualPath::new(path));
        // Read the Typst source from the file.
        let source_text = std::fs::read_to_string(path).unwrap_or_else(|e| {
            panic!(
                "Failed to read Typst source from {:?}, error: {:?}",
                path, e
            )
        });
        // Prepare the Typst source.
        let source = Source::new(file_id, source_text);
        Self::new(source, file_id)
    }

    pub fn with_string_literal(source_text: &str) -> Self {
        // Create a new FileId for the virtual main file ("/main.typ").
        let file_id = FileId::new(None, VirtualPath::new(Path::new("main.typ")));
        // Prepare the Typst source.
        let source = Source::new(file_id, source_text.to_string());
        Self::new(source, file_id)
    }
}

impl World for MinimalWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    fn main(&self) -> FileId {
        self.file_id
    }

    fn source(&self, id: FileId) -> typst::diag::FileResult<Source> {
        if id == self.file_id {
            Ok(self.source.clone())
        } else {
            panic!("Only the main file is supported in this minimal example")
        }
    }

    fn file(&self, _id: FileId) -> typst::diag::FileResult<Bytes> {
        /*
            assert!(id.package().is_none());
        Ok(Bytes::new(
            typst_dev_assets::get_by_name(
                &id.vpath().as_rootless_path().to_string_lossy(),
            )
            .unwrap_or_else(|| panic!("failed to load {:?}", id.vpath())),
        ))
        */
        panic!("File access not implemented in this minimal example")
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index)?.get()
    }

    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        // Apply the UTC offset to self.now
        let with_offset = match offset {
            None => self.now.with_timezone(self.now.offset()).fixed_offset(),
            Some(hours) => {
                let seconds = i32::try_from(hours).ok()?.checked_mul(3600)?;
                let fixed = FixedOffset::east_opt(seconds)?;
                self.now.with_timezone(&fixed)
            }
        };

        Datetime::from_ymd(
            with_offset.year(),
            with_offset.month().try_into().ok()?,
            with_offset.day().try_into().ok()?,
        )
    }
}
