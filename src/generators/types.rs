use std::{fs, io, path::Path};
use console::style;
use heck::ToUpperCamelCase;

use crate::fs_utils::write_new_file;

pub fn generate_type(name: &str, base_path: &Path) -> io::Result<()> {
    let type_name = name.to_upper_camel_case();
    let type_path = base_path.join(format!("{type_name}.ts"));

    let content = format!(
        "export type {type_name} {{
}}
"
    );

    fs::create_dir_all(base_path)?;
    write_new_file(&type_path, &content)?;

    println!(
        "Created Type: {} {}",
        style(type_name).cyan().bold(),
        style(format!("(at: {})", type_path.display())).dim()
    );

    Ok(())
}