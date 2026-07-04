use std::{fs, io, path::Path};
use console::style;
use heck::ToUpperCamelCase;

use crate::fs_utils::write_new_file;

pub fn generate_enum(name: &str, base_path: &Path) -> io::Result<()> {
    let enum_name = name.to_upper_camel_case();
    let enum_path = base_path.join(format!("{enum_name}.ts"));

    let content = format!(
        "export enum {enum_name} {{
}}
"
    );

    fs::create_dir_all(base_path)?;
    write_new_file(&enum_path, &content)?;

    println!(
        "Created Enum: {} {}",
        style(enum_name).cyan().bold(),
        style(format!("(at: {})", enum_path.display())).dim()
    );

    Ok(())
}