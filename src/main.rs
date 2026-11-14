mod builder;
mod cli;
mod config;
mod error;
mod project;
mod util;

use error::Result;
use std::time::Instant;

fn main() {
    if let Err(e) = run() {
        eprintln!("\n[x] {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    println!("=============================================");
    println!("   APK-Builder v{}  |  HTML -> Android APK", env!("CARGO_PKG_VERSION"));
    println!("=============================================\n");

    let args = cli::parse_args()?;
    let cfg = cli::collect_config(&args)?;

    if !cli::confirm(&cfg, args.yes)? {
        println!("Cancelled.");
        return Ok(());
    }

    let started = Instant::now();

    println!("\n[1/3] Checking toolchain...");
    let toolchain = builder::detect()?;

    println!("[2/3] Generating Android project...");
    let root = project::generate(&cfg)?;

    println!("[3/3] Building APK (the first build downloads dependencies)...\n");
    let apk = builder::build(&toolchain, &root, &cfg)?;

    println!("\n[+] Done in {:.1}s", started.elapsed().as_secs_f32());
    println!("[+] APK saved to: {}", apk.display());
    Ok(())
}
