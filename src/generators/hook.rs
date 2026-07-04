use std::{fs, io};
use std::path::Path;
use console::style;
use heck::ToUpperCamelCase;
use crate::fs_utils::write_new_file;

pub fn generate_hook(name: &str, base_path: &Path) -> io::Result<()> {
    let hook_name = normalize_hook_name(name);
    let dir = base_path;

    fs::create_dir_all(&dir)?;

    let hook_path = dir.join(format!("{hook_name}.ts"));

    let content = format!(
        "export function {hook_name}() {{
  return null;
}}
"
    );

    write_new_file(&hook_path, &content)?;

    println!(
        "Created Hook: {} {}",
        style(hook_name).cyan().bold(),
        style(format!("(at: {})", hook_path.display())).dim()
    );

    Ok(())
}

fn normalize_hook_name(name: &str) -> String {
    let camel = name.to_upper_camel_case();

    if camel.starts_with("Use") {
        format!("use{}", &camel[3..])
    } else {
        format!("use{camel}")
    }
}
