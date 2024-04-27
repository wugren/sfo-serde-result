use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum SerdeResult<T, E> {
    Ok(T),
    Err(E)
}

impl<T, E> From<std::result::Result<T, E>> for SerdeResult<T, E> {
    fn from(value: std::result::Result<T, E>) -> Self {
        match value {
            Ok(t) => SerdeResult::Ok(t),
            Err(e) => SerdeResult::Err(e),
        }
    }
}

impl<T, E> Into<std::result::Result<T, E>> for SerdeResult<T, E> {
    fn into(self) -> std::result::Result<T, E> {
        match self {
            SerdeResult::Ok(t) => Ok(t),
            SerdeResult::Err(e) => Err(e),
        }
    }
}
