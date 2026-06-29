use std::{fs, io};
use std::path::PathBuf;
use heck::ToUpperCamelCase;
use crate::fs_utils::write_new_file;

pub fn generate_component(name: &str, with_css: bool) -> io::Result<()> {
    let component_name = name.to_upper_camel_case();
    let dir = PathBuf::from("src/components").join(&component_name);

    fs::create_dir_all(&dir)?;

    let component_path = dir.join(format!("{component_name}.tsx"));

    let import_style = if with_css {
        format!("import styles from './{component_name}.module.css';\n\n")
    } else {
        String::new()
    };

    let content = format!(
        "{import_style}export function {component_name}() {{
  return (
    <div>
      {component_name}
    </div>
  );
}}
"
    );

    write_new_file(&component_path, &content)?;

    if with_css {
        let style_path = dir.join(format!("{component_name}.module.css"));
        write_new_file(&style_path, "")?;
    }

    println!("Created component: {}", component_path.display());

    Ok(())
}
