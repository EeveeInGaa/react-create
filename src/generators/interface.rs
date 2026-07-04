use std::{fs, io, path::Path};
use console::style;
use heck::ToUpperCamelCase;

use crate::fs_utils::write_new_file;

pub fn generate_interface(name: &str, base_path: &Path) -> io::Result<()> {
    let interface_name = name.to_upper_camel_case();
    let interface_path = base_path.join(format!("{interface_name}.ts"));

    let content = format!(
        "export interface {interface_name} {{
}}
"
    );

    fs::create_dir_all(base_path)?;
    write_new_file(&interface_path, &content)?;

    println!(
        "Created Interface: {} {}",
        style(interface_name).cyan().bold(),
        style(format!("(at: {})", interface_path.display())).dim()
    );

    Ok(())
}