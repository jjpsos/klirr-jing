mod init_logging;
mod input;

pub mod prelude {
    pub(crate) use clap::{Parser, Subcommand};
    pub(crate) use derive_more::FromStr;
    pub(crate) use invoice_typst_logic::prelude::*;
    pub(crate) use invoice_typst_render::prelude::*;

    pub(crate) use crate::input::*;
}

use prelude::*;

fn run() -> Result<()> {
    let input = get_input()?;
    info!("🔮 Starting PDF creation, input: {}...", input);
    let pdf_location = create_pdf(input, |l18n, data| {
        render(Path::new("./crates/render/src/invoice.typ"), l18n, data)
    })?;
    save_pdf_location_to_tmp_file(pdf_location)
}

fn main() {
    init_logging::init_logging();
    match run() {
        Ok(_) => {}
        Err(e) => {
            error!("Error creating PDF: {}", e);
        }
    }
}
