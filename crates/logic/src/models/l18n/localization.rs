use crate::prelude::*;
use std::collections::HashMap;

/// The language used and the content of the localization file.
#[derive(Debug, Clone, Serialize, Deserialize, Getters, TypedBuilder)]
pub struct L18n {
    /// Which language this localization file is for, e.g.
    /// "en" for English
    #[builder(setter(into))]
    #[getset(get = "pub")]
    locale: String,
    /// The content of the localization file, which includes
    /// client information, invoice information, vendor information,
    /// and line items.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    content: L18nContent,
}

impl L18n {
    pub fn new(locale: impl AsRef<str>) -> Result<Self> {
        let locale = locale.as_ref().to_owned();
        let Some(content) = L18N_MAP.get(&locale) else {
            return Err(Error::L18nNotFound { locale });
        };
        Ok(content.clone())
    }
}

impl L18n {
    pub fn english() -> Self {
        Self::builder()
            .locale("en")
            .content(L18nContent::english())
            .build()
    }
}

lazy_static::lazy_static! {
    static ref L18N_MAP: HashMap<String, L18n> = {
        let mut m = HashMap::new();
        m.insert("en".to_string(), L18n::english());
        m
    };
}

#[cfg(test)]
mod tests {
    use insta::assert_ron_snapshot;

    use super::*;

    #[test]
    fn test_l18n_english() {
        assert_ron_snapshot!(&L18n::english());
    }
}
