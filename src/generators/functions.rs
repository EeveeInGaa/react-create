use std::{fs, io, path::Path};
use console::style;
use heck::{ToKebabCase, ToLowerCamelCase};

use crate::fs_utils::write_new_file;

pub fn generate_function(name: &str, base_path: &Path) -> io::Result<()> {
    let file_name = name.to_kebab_case();
    let function_name = name.to_lower_camel_case();

    fs::create_dir_all(base_path)?;

    let function_path = base_path.join(format!("{file_name}.ts"));

    let content = format!(
        "export function {function_name}() {{\n\
}}\n"
    );

    write_new_file(&function_path, &content)?;

    println!(
        "Created Function: {} {}",
        style(function_name).cyan().bold(),
        style(format!("(at: {})", function_path.display())).dim()
    );

    Ok(())
}