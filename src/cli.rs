use crate::config::BuildConfig;
use crate::error::Result;
use crate::fail;
use crate::util::{clean_path, is_valid_package, slugify, strip_quotes};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct Args {
    pub input: Option<String>,
    pub output: Option<String>,
    pub name: Option<String>,
    pub package: Option<String>,
    pub yes: bool,
}

pub fn parse_args() -> Result<Args> {
    let mut args = Args::default();
    let mut it = std::env::args().skip(1);
    while let Some(flag) = it.next() {
        let mut value = |name: &str| -> Result<String> {
            match it.next() {
                Some(v) => Ok(v),
                None => fail!("Missing value for {name}"),
            }
        };
        match flag.as_str() {
            "-i" | "--input" => args.input = Some(value("--input")?),
            "-o" | "--output" => args.output = Some(value("--output")?),
            "-n" | "--name" => args.name = Some(value("--name")?),
            "-p" | "--package" => args.package = Some(value("--package")?),
            "-y" | "--yes" => args.yes = true,
            "-h" | "--help" => {
                print_help();
                std::process::exit(0);
            }
            other => fail!("Unknown option: {other} (try --help)"),
        }
    }
    Ok(args)
}

fn print_help() {
    println!(
        "APK-Builder {}\n\n\
         USAGE:\n  apk-builder [OPTIONS]\n\n\
         OPTIONS:\n  \
         -i, --input <PATH>     HTML file or folder containing index.html\n  \
         -o, --output <DIR>     Folder for the final APK\n  \
         -n, --name <NAME>      App name (default: derived from input)\n  \
         -p, --package <ID>     Android package id (default: com.apkbuilder.<name>)\n  \
         -y, --yes              Skip the confirmation prompt\n  \
         -h, --help             Show this help\n\n\
         Run without options to use the interactive mode.",
        env!("CARGO_PKG_VERSION")
    );
}

fn ask(question: &str) -> Result<String> {
    print!("{question}");
    io::stdout().flush()?;
    let mut line = String::new();
    if io::stdin().read_line(&mut line)? == 0 {
        fail!("Input closed unexpectedly.");
    }
    Ok(strip_quotes(&line))
}

fn validate_input(path: &Path) -> Option<String> {
    if !path.exists() {
        return Some("Path does not exist.".into());
    }
    if path.is_file() {
        let ok = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.eq_ignore_ascii_case("html") || e.eq_ignore_ascii_case("htm"))
            .unwrap_or(false);
        return if ok { None } else { Some("File must be .html or .htm.".into()) };
    }
    if path.join("index.html").is_file() {
        None
    } else {
        Some("Folder must contain an index.html file.".into())
    }
}

fn default_name(input: &Path) -> String {
    let raw = if input.is_file() {
        input.file_stem()
    } else {
        input.file_name()
    };
    raw.and_then(|s| s.to_str()).unwrap_or("MyApp").to_string()
}

pub fn collect_config(args: &Args) -> Result<BuildConfig> {
    let input = loop {
        let raw = match &args.input {
            Some(v) => strip_quotes(v),
            None => ask("Path of the HTML file (or site folder) to build: ")?,
        };
        let path = PathBuf::from(&raw);
        match validate_input(&path) {
            None => break clean_path(path.canonicalize()?),
            Some(reason) if args.input.is_some() => fail!("Invalid input: {reason}"),
            Some(reason) => println!("[!] {reason} Try again."),
        }
    };

    let output_dir = loop {
        let raw = match &args.output {
            Some(v) => strip_quotes(v),
            None => ask("Folder where the APK should be saved: ")?,
        };
        if !raw.is_empty() {
            break PathBuf::from(raw);
        }
        if args.output.is_some() {
            fail!("Output folder cannot be empty.");
        }
        println!("[!] Please enter a folder path.");
    };

    let default = default_name(&input);
    let app_name = match &args.name {
        Some(n) => n.clone(),
        None => {
            let a = ask(&format!("App name [{default}]: "))?;
            if a.is_empty() { default.clone() } else { a }
        }
    };

    let default_pkg = format!("com.apkbuilder.{}", slugify(&app_name));
    let package = loop {
        let candidate = match &args.package {
            Some(p) => p.clone(),
            None => {
                let a = ask(&format!("Package id [{default_pkg}]: "))?;
                if a.is_empty() { default_pkg.clone() } else { a }
            }
        };
        if is_valid_package(&candidate) {
            break candidate;
        }
        if args.package.is_some() {
            fail!("Invalid package id: {candidate}");
        }
        println!("[!] Use a format like com.example.myapp. Try again.");
    };

    Ok(BuildConfig {
        input,
        output_dir,
        app_name,
        package,
        version_name: "1.0".to_string(),
    })
}

pub fn confirm(cfg: &BuildConfig, skip: bool) -> Result<bool> {
    println!();
    println!("---------------- Build summary ----------------");
    println!("  Source  : {}", cfg.input.display());
    println!("  Output  : {}", cfg.output_dir.display());
    println!("  App name: {}", cfg.app_name);
    println!("  Package : {}", cfg.package);
    println!("-----------------------------------------------");
    if skip {
        return Ok(true);
    }
    let a = ask("Start build? [Y/n]: ")?.to_ascii_lowercase();
    Ok(a.is_empty() || a == "y" || a == "yes")
}
