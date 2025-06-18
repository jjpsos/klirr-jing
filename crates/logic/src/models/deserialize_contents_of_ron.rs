use crate::prelude::*;
use serde::de::DeserializeOwned;

pub fn type_name<T>() -> String {
    std::any::type_name::<T>().to_string()
}

pub fn deserialize_contents_of_ron<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T> {
    debug!(
        "☑️ Deserializing {} from {}",
        type_name::<T>(),
        path.as_ref().display()
    );
    use ron::de::from_str;
    use std::fs;
    let path = path.as_ref();
    let ron_str = fs::read_to_string(path).map_err(|e| {
        error!(
            "Failed to read data from '{}', error: {:?}",
            path.display(),
            e
        );
        Error::FileNotFound {
            path: path.display().to_string(),
        }
    })?;
    let result = from_str(&ron_str).map_err(|e| {
        error!("Failed to deserialize {}: {}", path.display(), e);
        Error::Deserialize {
            type_name: type_name::<T>(),
            error: e.to_string(),
        }
    })?;
    debug!(
        "✅ Deserialized {} from {}",
        type_name::<T>(),
        path.display()
    );
    Ok(result)
}
