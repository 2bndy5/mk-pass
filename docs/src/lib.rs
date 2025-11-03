//! This exposes a function in Python, so an mkdocs plugin can use it to generate the CLI document.
//! For actual library/binary source code look in cpp-linter folder.
use std::collections::HashMap;

use mk_pass::{
    PasswordRequirements,
    clap::{ArgAction, CommandFactory},
};
use pyo3::{exceptions::PyValueError, prelude::*};

#[pyfunction]
fn generate_cli_doc(metadata: HashMap<String, HashMap<String, Py<PyAny>>>) -> PyResult<String> {
    let mut out = String::new();
    let mut command = PasswordRequirements::command();
    out.push_str(
        format!(
            "```text title=\"Usage\"\n{}\n```\n",
            command
                .render_usage()
                .to_string()
                .trim_start_matches("Usage: ")
        )
        .as_str(),
    );

    out.push_str("\n## Options\n");
    for arg in command.get_arguments() {
        let arg_id = arg.get_id().as_str();
        let long_name = arg.get_long().ok_or(PyValueError::new_err(format!(
            "Failed to get long name of argument with id {arg_id}",
        )))?;
        out.push_str(
            format!(
                "\n### `-{}, --{}`\n\n",
                arg.get_short().ok_or(PyValueError::new_err(format!(
                    "Failed to get short name for argument with id {arg_id}"
                )))?,
                long_name
            )
            .as_str(),
        );
        if let Some(map) = metadata.get(long_name) {
            if let Some(val) = map.get("minimum-version") {
                out.push_str(format!("<!-- md:version {val} -->\n").as_str());
            }
            if map.contains_key("experimental") {
                out.push_str("<!-- md:flag experimental -->\n");
            }
        }
        match arg.get_action() {
            ArgAction::SetTrue | ArgAction::SetFalse | ArgAction::Help | ArgAction::Version => {
                out.push_str("<!-- md:flag -->\n\n");
            }
            _ => {
                let default = arg.get_default_values();
                if let Some(default_value) = default.first() {
                    out.push_str(
                        format!(
                            "<!-- md:default {} -->\n\n",
                            default_value.to_string_lossy()
                        )
                        .as_str(),
                    );
                } else {
                    out.push('\n');
                }
            }
        }
        if let Some(help) = &arg.get_long_help().or(arg.get_help()) {
            out.push_str(format!("{}\n", help.to_string().trim()).as_str());
        }
    }
    Ok(out)
}

#[pymodule(gil_used = false)]
pub fn cli_gen(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_cli_doc, m)?)?;
    Ok(())
}
