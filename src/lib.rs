use std::fmt::{Debug, Display};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum SerdeResult<T, E> {
    Ok(T),
    Err(E)
}

impl<T, E> From<sfo_result::Result<T>> for SerdeResult<T, E>
where
    T: Serialize + for<'a> Deserialize<'a>,
    E: Serialize + for<'a> Deserialize<'a> + Debug + Display + Sync + Send + 'static + From<String> {
    fn from(value: sfo_result::Result<T>) -> Self {
        match value {
            Ok(t) => SerdeResult::Ok(t),
            Err(e) => SerdeResult::Err(e.downcast::<E>().unwrap_or_else(|e| E::from(format!("{}", e)))),
        }
    }
}

impl<T, E> Into<sfo_result::Result<T>> for SerdeResult<T, E>
where T: Serialize + for<'a> Deserialize<'a>,
      E: Serialize + for<'a> Deserialize<'a> + Send + Sync + std::error::Error + 'static {
    fn into(self) -> sfo_result::Result<T> {
        match self {
            SerdeResult::Ok(t) => Ok(t),
            SerdeResult::Err(e) => Err(sfo_result::AnyError::new(e)),
        }
    }
}
