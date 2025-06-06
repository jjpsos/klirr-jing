mod minimal_world;
mod render;

pub mod prelude {
    pub use crate::minimal_world::*;
    pub use crate::render::*;

    pub use getset::Getters;
    pub use invoice_typst_logic::prelude::*;
    pub use typed_builder::TypedBuilder;
}
