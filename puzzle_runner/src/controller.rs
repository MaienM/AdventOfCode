//! A controller handles generic actions (e.g., getting inputs, validating results) for a specific
//! series.

use serde::{Deserialize, Serialize};

/// The actions for a controller. This should be implemented manually.
#[allow(clippy::missing_errors_doc)]
pub trait Controller: Send + Sync {
    /// Create a new instance of the controller.
    fn new() -> ControllerResult<Self>
    where
        Self: Sized;

    /// Get the input for a chapter.
    fn get_input(&self, chapter: &str) -> ControllerResult<String> {
        let _ = chapter;
        Err(ControllerError::NotImplemented)
    }
}

/// The result of a controller action.
pub type ControllerResult<T> = Result<T, ControllerError>;

/// An error that prevented the controller action from completing.
///
/// Any error type that implements [`ToString`] can be cast into this with the `?` operator, and
/// it in turn can be cast into a [`String`] in the same manner.
#[derive(Debug, Serialize, Deserialize)]
pub enum ControllerError {
    NotImplemented,
    Err(String),
}
impl<T> From<T> for ControllerError
where
    T: ToString,
{
    fn from(value: T) -> Self {
        ControllerError::Err(value.to_string())
    }
}
impl From<ControllerError> for String {
    fn from(value: ControllerError) -> Self {
        match value {
            ControllerError::NotImplemented => "not implemented".to_owned(),
            ControllerError::Err(err) => err.clone(),
        }
    }
}

/// The default [`Controller`], which returns [`ControllerError::NotImplemented`] for all
/// functions.
pub struct DefaultController;
impl Controller for DefaultController {
    fn new() -> ControllerResult<Self>
    where
        Self: Sized,
    {
        Ok(DefaultController)
    }
}
