//! A controller handles generic actions (e.g., getting inputs, validating results) for a specific
//! series.

use serde::{Deserialize, Serialize};

use crate::{
    derived::{ChapterBuilder, ExampleBuilder, PartBuilder, SeriesBuilder},
    source::{ChapterSources, IOResult, PartFileType},
};

/// The actions for a controller.
#[allow(clippy::missing_errors_doc)]
pub trait Controller: Send + Sync {
    /// Create a new instance of the controller.
    fn new() -> ControllerResult<Self>
    where
        Self: Sized;

    /// Process metadata for the entire [`Series`](puzzle_runner::derived::Series).
    ///
    /// All nested items will already have been processed before this is called.
    ///
    /// # Errors
    ///
    /// Returns an error if the series is invalid.
    fn process_series(&self, series: &mut SeriesBuilder) -> ControllerResult<()> {
        let _ = series;
        Ok(())
    }

    /// Process metadata for a [`Chapter`](puzzle_runner::derived::Chapter).
    ///
    /// All nested items will already have been processed before this is called.
    ///
    /// # Errors
    ///
    /// Returns an error if the chapter is invalid.
    fn process_chapter(&self, chapter: &mut ChapterBuilder) -> ControllerResult<()> {
        let _ = chapter;
        Ok(())
    }

    /// Process metadata for a [`Part`](puzzle_runner::derived::Part).
    ///
    /// All nested items will already have been processed before this is called.
    ///
    /// # Errors
    ///
    /// Returns an error if the part is invalid.
    fn process_part(&self, part: &mut PartBuilder) -> ControllerResult<()> {
        let _ = part;
        Ok(())
    }

    /// Process metadata for an [`Example`](puzzle_runner::derived::Example).
    ///
    /// All nested items will already have been processed before this is called.
    ///
    /// # Errors
    ///
    /// Returns an error if the example is invalid.
    fn process_example(&self, example: &mut ExampleBuilder) -> ControllerResult<()> {
        let _ = example;
        Ok(())
    }

    /// Get the URL for a chapter.
    fn chapter_url(&self, chapter: &str) -> ControllerResult<String> {
        let _ = chapter;
        Err(ControllerError::NotImplemented)
    }

    /// Get the input for a chapter.
    fn get_input(&self, chapter: &str) -> ControllerResult<String> {
        let _ = chapter;
        Err(ControllerError::NotImplemented)
    }

    /// Validate the result for a part.
    ///
    /// This uses [`Controller::validate_result_impl`] & the files for the part to create a
    /// complete flow.
    ///
    /// If the known-good file exists this will simply compare against it & base the outcome on
    /// that. Else, if the known-bad file exists & the result is in it the outcome will be
    /// negative. In both cases [`Controller::validate_result_impl`] will be skipped.
    ///
    /// If the result cannot be validated based on the existing files
    /// [`Controller::validate_result_impl`] will be called. If the outcome of this is that result
    /// is valid the known-good file will be created and the known-bad & pending files will be
    /// deleted (if they exist). If the outcome is that the result is invalid it will be added to
    /// the known-bad file (creating it if it does not yet exist).
    fn validate_result(
        &self,
        chapter: &str,
        part: u8,
        result: &str,
        sources: &ChapterSources,
    ) -> ControllerResult<(bool, String)> {
        // If there is a result file we can base the outcome entirely on that.
        let resultfile = sources.part(part, &PartFileType::Result).to_value()?;
        if let IOResult::Ok(expected) = resultfile.read() {
            return if result == expected {
                Ok((
                    true,
                    format!("Matches known-good value in {}.", resultfile.source()),
                ))
            } else {
                Ok((
                    false,
                    format!(
                        "Doesn't match known-good value {} in {}.",
                        expected,
                        resultfile.source()
                    ),
                ))
            };
        }

        // Load the incorrect file, & determine the outcome if the result is already in that list.
        let incorrectfile = sources.part(part, &PartFileType::Incorrect).to_value()?;
        let mut incorrect = if let IOResult::Ok(incorrect) = incorrectfile.read()
            && let Ok(incorrect) = serde_json::from_str::<Vec<String>>(&incorrect)
        {
            incorrect
        } else {
            Vec::new()
        };
        if incorrect.contains(&result.to_owned()) {
            return Ok((
                false,
                format!(
                    "Value is in known-bad list in {} due to failing a previous validation.",
                    incorrectfile.source()
                ),
            ));
        }

        // If the result is a value that seems more likely to be a placeholder than a correct value
        // avoid submitting it.
        if ["0", "1", ""].contains(&result) || result.contains('\n') {
            return Ok((false, "Value seems unlikely to be valid.".into()));
        }

        // The outcome cannot be determined based on existing info, so run the real validation &
        // update the files based on the outcome of this.
        let outcome = self.validate_result_impl(chapter, part, result)?;
        if outcome.0 {
            resultfile.write(result).to_value()?;
            incorrectfile.delete().to_option()?;
            sources
                .part(part, &PartFileType::Pending)
                .to_value()?
                .delete()
                .to_option()?;
        } else {
            incorrect.push(result.to_owned());
            incorrectfile
                .write(&serde_json::to_string(&incorrect)?)
                .to_value()?;
        }
        Ok(outcome)
    }

    /// Validate the result for a part.
    ///
    /// In most cases this should not be called directly and [`Controller::validate_result`] should
    /// be used instead, which includes caching to avoid performing unneccesary validations against
    /// outside services.
    fn validate_result_impl(
        &self,
        chapter: &str,
        part: u8,
        result: &str,
    ) -> ControllerResult<(bool, String)> {
        let _ = (chapter, part, result);
        Err(ControllerError::NotImplemented)
    }
}

/// The result of a controller action.
pub type ControllerResult<T> = Result<T, ControllerError>;

/// An error that prevented the controller action from completing.
///
/// Note that this is only instead for actions that could not be completed, actions which complete
/// but have a negative outcome (e.g., a validation on a value that turns out to be invalid) should
/// _not_ use this type.
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
