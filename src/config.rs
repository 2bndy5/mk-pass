use crate::helpers::{LOWERCASE, NUMBERS, SPECIAL_CHARACTERS, UPPERCASE};

#[cfg(feature = "clap")]
use clap::Parser;

/// A structure to describe password requirements.
#[cfg_attr(
    feature = "clap",
    derive(Parser),
    command(about = "Generate a password comprehensively.", version, long_about = None)
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PasswordRequirements {
    /// The length of the password.
    #[cfg_attr(feature = "clap", arg(long, short, default_value = "16"))]
    pub length: u16,

    /// How many numeric characters should the password contain?
    #[cfg_attr(feature = "clap", arg(long, short, default_value = "1"))]
    pub numbers: u16,

    /// How many special characters should the password contain?
    #[cfg_attr(feature = "clap", arg(long, short, default_value = "1"))]
    pub specials: u16,

    /// Should the first character always be a letter?
    #[cfg_attr(
        feature = "clap",
        arg(
            long = "no-first-is-letter",
            short,
            help = "Do not restrict the first character to only letters.",
            long_help = "Do not restrict the first character to only letters.\
            \n\nBy default, the first character is always a letter.",
            default_value = "true",
            action = clap::ArgAction::SetFalse
        )
    )]
    pub first_is_letter: bool,
}

impl PasswordRequirements {
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
    ///    - 52 if only letters (no numbers or special characters) are used
    ///    - 62 if only letters and numbers are used
    ///    - 68 if only letters and special characters are used
    ///    - 78 if letters, numbers, and special characters are used
    /// 3. `specials` character count does not overrule the required number of
    ///
    ///    - letters (2; 1 uppercase and 1 lowercase)
    ///    - numbers (if `numbers` is specified as non-zero value)
    /// 4. `numbers` character count does not overrule the required number of
    ///
    ///    - letters (2; 1 uppercase and 1 lowercase)
    ///    - special characters (if `specials` is specified as non-zero value)
    ///
    /// # About resolving conflicts
    ///
    /// If this function finds a conflict between the specified number of
    /// `specials` characters and `numbers`, then numbers takes precedence.
    ///
    /// For example:
    ///
    /// ```rust
    /// use mk_pass::PasswordRequirements;
    /// let req = PasswordRequirements {
    ///     length: 16,
    ///     specials: 16,
    ///     numbers: 16,
    ///     ..Default::default()
    /// };
    /// let expected = PasswordRequirements {
    ///     length: 16,
    ///     specials: 1,
    ///     numbers: 13,
    ///     ..Default::default()
    /// };
    /// assert_eq!(req.validate(), expected);
    /// ```
    pub fn validate(&self) -> Self {
        let len = self.length.max(10).min(
            UPPERCASE.len() as u16
                + LOWERCASE.len() as u16
                + {
                    if self.specials > 0 {
                        SPECIAL_CHARACTERS.len() as u16
                    } else {
                        0
                    }
                }
                + {
                    if self.numbers > 0 {
                        NUMBERS.len() as u16
                    } else {
                        0
                    }
                },
        );
        let non_letter_max_len = len - 2;
        let max_special = non_letter_max_len - self.numbers.min(non_letter_max_len - 1);
        let max_numbers = non_letter_max_len - max_special;
        Self {
            length: len,
            numbers: self.numbers.min(max_numbers),
            specials: self.specials.min(max_special),
            first_is_letter: self.first_is_letter,
        }
    }
}

impl Default for PasswordRequirements {
    /// Create default password requirements.
    fn default() -> Self {
        Self {
            length: 16,
            numbers: 1,
            specials: 1,
            first_is_letter: true,
        }
    }
}
