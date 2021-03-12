use clap::{App, Arg};

fn create_clap_app() -> App<'static, 'static> {
    App::new("mmb-objdump")
        .version(clap::crate_version!())
        .about("Display information on Metamath Zero binary proof files")
        .author(clap::crate_authors!())
        .arg(
            Arg::with_name("PATH")
                .help("Path to the binary proof file")
                .required(true)
                .index(1),
        )
}

use anyhow::{Context, Result};
use mmb_parser::Mmb;

fn main() -> Result<()> {
    let app = create_clap_app();
    let matches = app.get_matches();

    let path = matches.value_of("PATH").expect("required argument");
    let data = std::fs::read(path)
        .with_context(|| format!("Could not read binary proof file from \"{}\"", path))?;

    let mmb = Mmb::from(&data).unwrap();

    println!("{}", mmb.version);

    Ok(())
}
