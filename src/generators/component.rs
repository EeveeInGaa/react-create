use std::{fs, io};
use std::path::{Path};
use heck::ToUpperCamelCase;
use crate::fs_utils::write_new_file;
use console::style;

pub fn generate_component(
    name: &str,
    base_path: &Path,
    with_css: bool,
    with_props: bool,
    with_docs: bool,
    with_test: bool,
    with_story: bool
) -> io::Result<()> {
    let component_name = name.to_upper_camel_case();
    let dir = base_path.join(&component_name);

    fs::create_dir_all(&dir)?;

    let component_path = dir.join(format!("{component_name}.tsx"));

    let import_style = if with_css {
        format!("import styles from './{component_name}.module.css';\n\n")
    } else {
        String::new()
    };

    let props_type = if with_props {
        format!("type {component_name}Props = {{\n  \n}};\n\n")
    } else {
        String::new()
    };

    let function_signature = match with_props {
        true => format!("export function {component_name}({{}}: {component_name}Props) {{"),
        false => format!("export function {component_name}() {{"),
    };

    let root_attributes = match with_css {
        true => " className={styles.root}",
        false => "",
    };

    let content = format!(
        "{import_style}{props_type}{function_signature}
  return (
    <div{root_attributes}>
      {component_name}
    </div>
  );
}}
"
    );

    write_new_file(&component_path, &content)?;

    if with_css {
        let style_path = dir.join(format!("{component_name}.module.css"));
        write_new_file(&style_path, ".root {\n}\n")?;
    }

    if with_docs {
        let docs_path = dir.join(format!("{component_name}.docs.mdx"));
        let docs_content = format!("# {component_name} Documentation\n");
        write_new_file(&docs_path, &docs_content)?;
    }

    if with_test {
        let docs_path = dir.join(format!("{component_name}.test.tsx"));
        let docs_content = format!(
            r#"import {{ describe, expect, it }} from 'vitest';

describe('{component_name}', () => {{
    it('should ...', () => {{
        expect().toBe();
    }});
}});
"#
        );
        write_new_file(&docs_path, &docs_content)?;
    }

    if with_story {
        let docs_path = dir.join(format!("{component_name}.stories.tsx"));
        write_new_file(&docs_path, "")?;
    }

    println!(
        "Created Component: {} {}",
        style(component_name).cyan().bold(),
        style(format!("(at: {})", component_path.display())).dim()
    );

    Ok(())
}
