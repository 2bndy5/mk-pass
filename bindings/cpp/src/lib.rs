#![allow(clippy::missing_safety_doc)]
use std::ffi::{CStr, CString, c_char};

// /// The list of possible special characters used when generating a password.
// pub const SPECIAL_CHARACTERS: [char; 16] = ::mk_pass::SPECIAL_CHARACTERS;

// /// The list of possible decimal used when generating a password.
// pub const DECIMAL: [char; 10] = ::mk_pass::DECIMAL;

// /// The list of possible uppercase letters used when generating a password.
// pub const UPPERCASE: [char; 26] = ::mk_pass::UPPERCASE;

// /// The list of possible lowercase letters used when generating a password.
// pub const LOWERCASE: [char; 26] = ::mk_pass::LOWERCASE;

/// A structure to describe the requirements of a password's contents.
#[repr(C)]
pub struct PasswordRequirements {
    /// The length of the password.
    pub length: u16,

    /// How many decimal integer characters should the password contain?
    pub decimal: u16,

    /// How many special characters should the password contain?
    pub specials: u16,

    /// Should the first character always be a letter?
    pub first_is_letter: bool,
}

impl From<&PasswordRequirements> for ::mk_pass::PasswordRequirements {
    fn from(value: &PasswordRequirements) -> Self {
        Self {
            length: value.length,
            decimal: value.decimal,
            specials: value.specials,
            first_is_letter: value.first_is_letter,
        }
    }
}

impl From<PasswordRequirements> for ::mk_pass::PasswordRequirements {
    fn from(value: PasswordRequirements) -> Self {
        Self {
            length: value.length,
            decimal: value.decimal,
            specials: value.specials,
            first_is_letter: value.first_is_letter,
        }
    }
}

impl From<::mk_pass::PasswordRequirements> for PasswordRequirements {
    fn from(value: ::mk_pass::PasswordRequirements) -> Self {
        Self {
            length: value.length,
            decimal: value.decimal,
            specials: value.specials,
            first_is_letter: value.first_is_letter,
        }
    }
}

/// Validates the instance's values.
///
/// This returns a mutated copy of the instance where the values satisfy
/// "sane minimum requirements" suitable for any password.
///
/// The phrase "sane minimum requirements" implies
///
/// 1. `length` is not less than 10
/// 2. To avoid repetitions, `length` is not more than
///
///    - 52 if only letters (no decimal integers or special characters) are used
///    - 62 if only letters and decimal integers are used
///    - 68 if only letters and special characters are used
///    - 78 if letters, decimal integers, and special characters are used
/// 3. `specials` character count does not overrule the required number of
///
///    - letters (2; 1 uppercase and 1 lowercase)
///    - decimal integers (if `decimal` is specified as non-zero value)
/// 4. `decimal` character count does not overrule the required number of
///
///    - letters (2; 1 uppercase and 1 lowercase)
///    - special characters (if `specials` is specified as non-zero value)
///
/// # About resolving conflicts
///
/// If this function finds a conflict between the specified number of
/// `specials` characters and `decimal`, then decimal integers takes precedence.
///
/// For example:
///
/// ```c
/// include "mk_pass.h";
///
/// PasswordRequirements req = PasswordRequirements {
///     length: 16,
///     specials: 16,
///     decimal: 16,
///     first_is_letter: true,
/// };
/// PasswordRequirements expected = PasswordRequirements {
///     length: 16,
///     specials: 1,
///     decimal: 13,
///     first_is_letter: true,
/// };
/// assert(validateRequirements(&req) == expected);
/// ```
#[unsafe(no_mangle)]
pub extern "C" fn validateRequirements(config: &PasswordRequirements) -> PasswordRequirements {
    ::mk_pass::PasswordRequirements::from(config)
        .validate()
        .into()
}

/// Generate a password given the constraints specified by `config`.
///
/// This function will invoke `validateRequirements(config)` to
/// ensure basic password requirements are met.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn generatePassword(
    string: *mut c_char,
    config: PasswordRequirements,
) -> u16 {
    if string.is_null() {
        eprintln!("The given pointer to c_str buffer was null");
        return 0;
    }
    let c_string = unsafe { CStr::from_ptr(string) };
    if c_string.is_empty() {
        eprintln!("Given c_str buffer was zero sized");
        return 0;
    }
    let real_config: ::mk_pass::PasswordRequirements = config.into();
    let new_password = ::mk_pass::generate_password(real_config);
    let result = CString::new(new_password.clone());
    match result {
        Ok(new_string) => {
            unsafe { std::ptr::copy(new_string.as_ptr(), string, real_config.length as usize) };
            real_config.length
        }
        Err(e) => {
            eprintln!("Failed to convert rust String to CString: {e:?}");
            0
        }
    }
}

/// The function used as a native entrypoint for an executable.
#[unsafe(no_mangle)]
pub extern "C" fn runMain() {
    use ::mk_pass::clap::Parser;

    let config = ::mk_pass::PasswordRequirements::parse();
    let password = ::mk_pass::generate_password(config);
    println!("{password}");
}
