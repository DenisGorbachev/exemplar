use derive_getters::Getters;
use derive_more::Deref;
use errgonomic::handle_bool;
use thiserror::Error;

#[derive(Deref, Clone, Debug)]
pub struct NonEmptyString(String);

impl TryFrom<String> for NonEmptyString {
    type Error = TryFromStringForNonEmptyStringError;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        use TryFromStringForNonEmptyStringError::*;
        handle_bool!(input.is_empty(), EmptyInput, input);
        Ok(Self(input))
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
    age: u32,
}

#[derive(Getters, Clone, Debug)]
pub struct Adult {
    name: NonEmptyString,
    age: u32,
}

impl TryFrom<Human> for Adult {
    type Error = ();

    fn try_from(_input: Human) -> Result<Self, Self::Error> {
        // TODO: Ensure that name is NonEmptyString
        // TODO: Ensure that age > 18
        todo!()
    }
}
