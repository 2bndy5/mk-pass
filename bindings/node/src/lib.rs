use ::mk_pass::clap::Parser;
use napi_derive::napi;

/// The function used as a native entrypoint for the executable script.
#[napi]
#[allow(
    dead_code,
    reason = "This function is exported in FFI binding, so it is used externally."
)]
pub fn main(args: Vec<String>) {
    let config = ::mk_pass::PasswordRequirements::parse_from(args);
    let password = ::mk_pass::generate_password(config);
    println!("{password}");
}

/// An object used to describe the requirements for generating a password.
#[napi(object)]
#[derive(Debug, PartialEq, Eq)]
pub struct PasswordRequirements {
    /// The length of the password.
    pub length: Option<i32>,

    /// How many numeric characters should the password contain?
    pub numbers: Option<i32>,

    /// How many special characters should the password contain?
    pub specials: Option<i32>,

    /// Ensure the first character is a letter?
    pub first_is_letter: Option<bool>,
}

impl From<&PasswordRequirements> for ::mk_pass::PasswordRequirements {
    fn from(value: &PasswordRequirements) -> ::mk_pass::PasswordRequirements {
        ::mk_pass::PasswordRequirements {
            length: value.length.unwrap_or(16) as u16,
            numbers: value.numbers.unwrap_or(1) as u16,
            specials: value.specials.unwrap_or(1) as u16,
            first_is_letter: value.first_is_letter.unwrap_or(true),
        }
    }
}

impl From<::mk_pass::PasswordRequirements> for PasswordRequirements {
    fn from(value: ::mk_pass::PasswordRequirements) -> PasswordRequirements {
        PasswordRequirements {
            length: Some(value.length as i32),
            numbers: Some(value.numbers as i32),
            specials: Some(value.specials as i32),
            first_is_letter: Some(value.first_is_letter),
        }
    }
}

/// Validates the given {@link PasswordRequirements} instance's values.
///
/// This returns a mutated copy of the `config` instance where
/// the values satisfy "sane minimum requirements" suitable for
/// any password.
///
/// The phrase "sane minimum requirements" implies
///
/// 1. `length` is not less than 10
/// 2. To avoid repetitions, `length` is not more than
///
///    - 52 if only letters (no numbers or special characters) are used
///    - 62 if only letters and numbers are used
///    - 68 if only letters and special characters are used
///    - 78 if letters, numbers, and special characters are used
/// 3. `special` character count does not overrule the required number of
///
///    - letters (2; 1 uppercase and 1 lowercase)
///    - numbers (if `numbers` is specified as non-zero value)
/// 4. `numbers` character count does not overrule the required number of
///
///    - letters (2; 1 uppercase and 1 lowercase)
///    - special characters (if `special` is specified as non-zero value)
///
/// # About resolving conflicts
///
/// If this function finds a conflict between the specified number of
/// `specials` characters and `numbers`, then `numbers` takes precedence.
///
/// For example:
///
/// ```js
/// const cpg = require('@mk-pass/mk-pass');
/// const assert = require('node:assert/strict');
/// let config = {
///   length: 16,
///   specials: 15,
///   numbers: 15
/// };
/// let expected = {
///   length: 16,
///   numbers: 13,
///   specials: 1,
///   firstIsLetter: true
/// };
/// assert.isDeepStrictEqual(
///   validateRequirements(config),
///   expected
/// );
/// ```
#[napi]
pub fn validate_requirements(config: PasswordRequirements) -> PasswordRequirements {
    let req = ::mk_pass::PasswordRequirements::from(&config);
    req.validate().into()
}

/// Generate a password given the constraints specified by `config`.
///
/// This function will invoke {@link validateRequirements} on the given `config`
/// to ensure basic password requirements are met.
#[napi]
pub fn generate_password(config: PasswordRequirements) -> String {
    ::mk_pass::generate_password(::mk_pass::PasswordRequirements::from(&config))
}

/// A class of factory constructors to fetch the sample sets used
/// when generating a password.
#[napi]
pub struct Samples {
    pub set: Vec<String>,
}

#[napi]
impl Samples {
    /// The list of possible special characters used when generating a password.
    #[napi(factory)]
    pub fn special_characters() -> Self {
        Self {
            set: ::mk_pass::SPECIAL_CHARACTERS
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>(),
        }
    }

    /// The list of possible numbers used when generating a password.
    #[napi(factory)]
    pub fn numbers() -> Self {
        Self {
            set: ::mk_pass::NUMBERS
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>(),
        }
    }

    /// The list of possible lowercase letters used when generating a password.
    #[napi(factory)]
    pub fn lowercase() -> Self {
        Self {
            set: ::mk_pass::LOWERCASE
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>(),
        }
    }

    /// The list of possible uppercase letters used when generating a password.
    #[napi(factory)]
    pub fn uppercase() -> Self {
        Self {
            set: ::mk_pass::UPPERCASE
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>(),
        }
    }
}
