use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn strip_quotes(s: &str) -> String {
    s.trim()
        .trim_matches(|c| c == '"' || c == '\'')
        .trim()
        .to_string()
}

pub fn clean_path(p: PathBuf) -> PathBuf {
    let stripped = p
        .to_string_lossy()
        .strip_prefix(r"\\?\")
        .map(PathBuf::from);
    stripped.unwrap_or(p)
}

pub fn slugify(name: &str) -> String {
    let s: String = name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    match s.chars().next() {
        None => "app".to_string(),
        Some(c) if c.is_ascii_digit() => format!("app{s}"),
        _ => s,
    }
}

pub fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

pub fn is_valid_package(p: &str) -> bool {
    const RESERVED: &[&str] = &[
        "abstract", "class", "default", "do", "double", "enum", "final", "for", "if", "in",
        "int", "is", "new", "null", "object", "package", "private", "public", "return",
        "static", "super", "this", "throw", "try", "val", "var", "when", "while",
    ];
    let parts: Vec<&str> = p.split('.').collect();
    parts.len() >= 2
        && parts.iter().all(|seg| {
            let mut chars = seg.chars();
            matches!(chars.next(), Some(c) if c.is_ascii_alphabetic())
                && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
                && !RESERVED.contains(&seg.to_ascii_lowercase().as_str())
        })
}

pub fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<u64> {
    fs::create_dir_all(dst)?;
    let mut count = 0;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let name = entry.file_name();
        if name == ".git" || name == ".svn" {
            continue;
        }
        let target = dst.join(&name);
        if entry.file_type()?.is_dir() {
            count += copy_dir_recursive(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), &target)?;
            count += 1;
        }
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slug_cases() {
        assert_eq!(slugify("My Cool App!"), "mycoolapp");
        assert_eq!(slugify("123"), "app123");
        assert_eq!(slugify("!!!"), "app");
    }

    #[test]
    fn package_validation() {
        assert!(is_valid_package("com.example.app"));
        assert!(!is_valid_package("example"));
        assert!(!is_valid_package("com.1bad.app"));
        assert!(!is_valid_package("com.class.app"));
    }

    #[test]
    fn quotes_and_xml() {
        assert_eq!(strip_quotes("  \"C:\\a b\\x.html\"  "), "C:\\a b\\x.html");
        assert_eq!(xml_escape("A & B <c>"), "A &amp; B &lt;c&gt;");
    }
}
