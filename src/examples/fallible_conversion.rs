use derive_getters::Getters;
use derive_more::Deref;
use thiserror::Error;

#[derive(Deref, Clone, Debug)]
pub struct NonEmptyString(String);

/// This is an example of a "simple" fallible conversion
/// `if` is used instead of `match` because there's only one boolean constraint
/// `handle_bool!` is not used because there's only one boolean constraint
impl TryFrom<String> for NonEmptyString {
    type Error = TryFromStringForNonEmptyStringError;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        use TryFromStringForNonEmptyStringError::*;
        if input.is_empty() {
            Err(EmptyInput {
                input,
            })
        } else {
            Ok(Self(input))
        }
    }
}

#[derive(Error, Debug)]
pub enum TryFromStringForNonEmptyStringError {
    #[error("expected input to be non-empty")]
    EmptyInput { input: String },
}

#[derive(Getters, Clone, Debug)]
pub struct Human {
    name: String,
    #[getter(copy)]
    age: u32,
}

#[derive(Getters, Clone, Debug)]
pub struct Adult {
    name: NonEmptyString,
    #[getter(copy)]
    age: u32,
}

/// This is an example of "normal" fallible conversion
impl TryFrom<Human> for Adult {
    type Error = ConvertHumanToAdultError;

    fn try_from(input: Human) -> Result<Self, Self::Error> {
        use ConvertHumanToAdultError::*;
        let Human {
            name,
            age,
        } = input;
        let name_result = NonEmptyString::try_from(name);
        let is_adult = age > 18;
        match (name_result, is_adult) {
            (Ok(name), true) => Ok(Self {
                name,
                age,
            }),
            (name_result, is_adult) => Err(ConversionFailed {
                name_result,
                age,
                is_adult,
            }),
        }
    }
}

#[derive(Error, Debug)]
pub enum ConvertHumanToAdultError {
    #[error("failed to convert human to adult")]
    ConversionFailed { name_result: Result<NonEmptyString, TryFromStringForNonEmptyStringError>, age: u32, is_adult: bool },
}
