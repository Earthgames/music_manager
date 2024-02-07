use clap::{CommandFactory, ValueEnum};
use clap_complete::{generate_to, Shell};
use clap_mangen::Man;
use std::{
    env,
    fs::File,
    io::Error,
    path::{Path, PathBuf},
};

include!("src/cli.rs");
//TODO Find a way to make add suggest files, and build it automatically instead of being unusable
fn build_shell_completion(outdir: &Path) -> Result<(), Error> {
    let mut app = Cli::command();
    let shells = Shell::value_variants();

    for shell in shells {
        generate_to(*shell, &mut app, "music_manager", outdir)?;
    }

    Ok(())
}

fn build_man_pages(outdir: &Path) -> Result<(), Error> {
    let app = Cli::command();
    let name = app.get_display_name().unwrap_or_else(|| app.get_name());

    let file = Path::new(&outdir).join(format!("{name}.1"));
    let mut file = File::create(file)?;
    Man::new(app.clone()).render(&mut file)?;

    for sub in app.get_subcommands() {
        let sub_name = sub.get_display_name().unwrap_or_else(|| sub.get_name());
        let file = Path::new(&outdir).join(format!("{name}-{sub_name}.1"));
        let mut file = File::create(file)?;
        Man::new(sub.to_owned()).render(&mut file)?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=src/cli.rs");
    println!("cargo:rerun-if-changed=man");

    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    // Create `target/assets/` folder.
    let out_path = PathBuf::from(outdir);
    let mut path = out_path.ancestors().nth(4).unwrap().to_owned();
    path.push("assets");
    std::fs::create_dir_all(&path).unwrap();

    build_shell_completion(&path)?;
    build_man_pages(&path)?;

    Ok(())
}
