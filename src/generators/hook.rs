use std::{fs, io};
use std::path::PathBuf;
use heck::ToUpperCamelCase;
use crate::fs_utils::write_new_file;

pub fn generate_hook(name: &str) -> io::Result<()> {
    let hook_name = normalize_hook_name(name);
    let dir = PathBuf::from("src/hooks");

    fs::create_dir_all(&dir)?;

    let hook_path = dir.join(format!("{hook_name}.ts"));

    let content = format!(
        "export function {hook_name}() {{
  return null;
}}
"
    );

    write_new_file(&hook_path, &content)?;

    println!("Created hook: {}", hook_path.display());

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
