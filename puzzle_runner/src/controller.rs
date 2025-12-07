#[derive(Debug)]
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

pub type ControllerResult<T> = Result<T, ControllerError>;

/// A controller handles generic actions (e.g., getting inputs, validating results) for specific
/// series.
#[allow(clippy::missing_errors_doc)]
pub trait Controller: Send + Sync {}

/// The default [`Controller`], which returns [`ControllerError::NotImplemented`] for all
/// functions.
pub struct DefaultController;
impl Controller for DefaultController {
}
