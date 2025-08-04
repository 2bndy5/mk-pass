use crate::PasswordRequirements;
use clap::Parser;
use pyo3::prelude::*;

/// The function used as an entrypoint for the executable script.
///
/// This function takes no parameters because
/// they are parsed directly from `sys.argv`.
#[pyfunction]
fn main(py: Python) -> PyResult<()> {
    let args = py
        .import("sys")?
        .getattr("argv")?
        .extract::<Vec<String>>()?;
    let config = PasswordRequirements::parse_from(args);
    let password = generate_password(&config);
    println!("{password}");
    Ok(())
}

#[pymethods]
impl PasswordRequirements {
    #[new]
    #[pyo3(
        signature = (length = 16, numbers=1, specials=1, first_is_letter = true)
    )]
    pub fn new(
        length: Option<i32>,
        numbers: Option<i32>,
        specials: Option<i32>,
        first_is_letter: Option<bool>,
    ) -> Self {
        Self {
            length: length.unwrap_or(16) as u16,
            numbers: numbers.unwrap_or(1) as u16,
            specials: specials.unwrap_or(1) as u16,
            first_is_letter: first_is_letter.is_none_or(|v| v),
        }
    }

    pub fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    /// Validates the instance's values.
    ///
    /// This returns a mutated clone of the instance where the values satisfy
    /// "sane minimum requirements" suitable for any password.
    ///
    /// The phrase "sane minimum requirements" implies
    ///
    /// 1. ``length`` is not less than 10
    /// 2. To avoid repetitions, ``length`` is not more than
    ///
    ///    - 52 if only letters (no numbers or special characters) are used
    ///    - 62 if only letters and numbers are used
    ///    - 68 if only letters and special characters are used
    ///    - 78 if letters, numbers, and special characters are used
    /// 3. ``special`` character count does not overrule the required number of
    ///
    ///    - letters (2; 1 uppercase and 1 lowercase)
    ///    - numbers (if ``numbers`` is specified as non-zero value)
    /// 4. ``numbers`` character count does not overrule the required number of
    ///
    ///    - letters (2; 1 uppercase and 1 lowercase)
    ///    - special characters (if ``special`` is specified as non-zero value)
    ///
    /// Note:
    ///     If this function finds a conflict between the specified number of
    ///     ``special`` characters and ``numbers``, then numbers takes precedence.
    ///
    ///     For example:
    ///
    ///     ```python
    ///     >>> from comp_gen_pass import PasswordRequirements
    ///     >>> req = PasswordRequirements(length=16, specials=16, numbers=16)
    ///     >>> req
    ///     PasswordRequirements { length: 16, numbers: 16, specials: 16, first_is_letter: true }
    ///     >>> req.validate()
    ///     PasswordRequirements { length: 16, numbers: 13, specials: 1, first_is_letter: true }
    ///     ```
    #[pyo3(name = "validate")]
    pub fn validate_py(&self) -> Self {
        (*self).validate()
    }
}

/// Generate a password given the constraints specified by `config`.
///
/// This function will invoke
/// [`PasswordRequirements.validate()`][mk_pass.PasswordRequirements.validate]
/// to ensure basic password requirements are met.
#[pyfunction]
pub fn generate_password(config: &PasswordRequirements) -> String {
    crate::generate_password(config.to_owned())
}

/// A python package binding the mk-pass library
/// written in rust.
#[pymodule]
pub fn mk_pass(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_password, m)?)?;
    m.add_class::<PasswordRequirements>()?;
    m.add_function(wrap_pyfunction!(main, m)?)?;
    Ok(())
}
