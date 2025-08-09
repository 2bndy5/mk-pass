#ifndef MK_PASS_H_
#define MK_PASS_H_

/* Generated with cbindgen:0.29.0 */

#include <stdint.h>

namespace mk_pass {

/// A structure to describe the requirements of a password's contents.
struct PasswordRequirements {
    /// The length of the password.
    uint16_t length;
    /// How many decimal integer characters should the password contain?
    uint16_t decimal;
    /// How many special characters should the password contain?
    uint16_t specials;
    /// Should the first character always be a letter?
    bool firstIsLetter;

    bool operator==(const PasswordRequirements &other) const {
        return length == other.length && decimal == other.decimal
               && specials == other.specials
               && firstIsLetter == other.firstIsLetter;
    }
    bool operator!=(const PasswordRequirements &other) const {
        return length != other.length || decimal != other.decimal
               || specials != other.specials
               || firstIsLetter != other.firstIsLetter;
    }
};

extern "C" {

/// Generate a password given the constraints specified by `config`.
///
/// This function will invoke `validateRequirements(config)` to
/// ensure basic password requirements are met.
uint16_t generatePassword(char* string, PasswordRequirements config);

/// The function used as a native entrypoint for an executable.
void runMain();

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
PasswordRequirements validateRequirements(const PasswordRequirements* config);

} // extern "C"

} // namespace mk_pass

#endif // MK_PASS_H_
