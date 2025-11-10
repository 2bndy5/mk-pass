#ifndef MK_PASS_H_
#define MK_PASS_H_

/* Generated with cbindgen:0.29.2 */

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
    /// Allow characters to be used more than once?
    bool allowRepeats;

    bool operator==(const PasswordRequirements &other) const {
        return length == other.length && decimal == other.decimal
               && specials == other.specials
               && firstIsLetter == other.firstIsLetter
               && allowRepeats == other.allowRepeats;
    }
    bool operator!=(const PasswordRequirements &other) const {
        return length != other.length || decimal != other.decimal
               || specials != other.specials
               || firstIsLetter != other.firstIsLetter
               || allowRepeats != other.allowRepeats;
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
///    - 65535 if repeated characters are allowed
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
/// #include <mk_pass.hpp>
///
/// PasswordRequirements req = {
///     16,    // length
///     16,    // specials
///     16,    // decimal
///     true,  // firstIsLetter
///     false, // allowRepeats
/// };
/// PasswordRequirements expected = {
///     16,    // length
///     1,     // specials
///     13,    // decimal
///     true,  // firstIsLetter
///     false, // allowRepeats
/// };
/// assert(validateRequirements(&req) == expected);
/// ```
PasswordRequirements validateRequirements(const PasswordRequirements* config);

} // extern "C"

} // namespace mk_pass

#endif // MK_PASS_H_
