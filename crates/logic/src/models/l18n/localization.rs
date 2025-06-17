use crate::prelude::*;
use std::collections::HashMap;

/// The language used and the content of the localization file.
#[derive(Debug, Clone, Serialize, Deserialize, Getters, TypedBuilder)]
pub struct L18n {
    /// Which language this localization file is for, e.g.
    /// "EN" for English
    #[builder(setter(into))]
    #[getset(get = "pub")]
    language: Language,

    /// The content of the localization file, which includes
    /// client information, invoice information, vendor information,
    /// and line items.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    content: L18nContent,
}

impl L18n {
    pub fn new(language: Language) -> Result<Self> {
        let Some(content) = L18N_MAP.get(&language) else {
            return Err(Error::L18nNotFound { language });
        };
        Ok(content.clone())
    }

    fn load_from_file(language: Language) -> Result<Self> {
        let path = format!("./input/l18n/{}.ron", language);
        deserialize_contents_of_ron(Path::new(&path))
    }
}

lazy_static::lazy_static! {
    static ref L18N_MAP: HashMap<Language, L18n> = {
        let mut m = HashMap::new();
        Language::all().for_each(|language| {
            m.insert(language, L18n::load_from_file(language).expect("Failed to load L18n"));
        });
        m
    };
}

#[cfg(test)]
mod tests {
    use insta::assert_ron_snapshot;

    use super::*;

    #[test]
    fn test_l18n_english() {
        assert_ron_snapshot!(&L18n::new(Language::EN).unwrap());
    }
}
