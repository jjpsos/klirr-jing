use typst::syntax::{FileId, Source, VirtualPath};

use crate::prelude::*;

pub(crate) trait LoadSource: Sized {
    fn load_source_at(path: impl AsRef<Path>) -> Result<Self>;
}

pub(crate) trait InlineSource: Sized {
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
    fn load_source_at(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
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
