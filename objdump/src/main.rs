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
use std::path::Path;

fn main() -> Result<()> {
    let app = create_clap_app();
    let matches = app.get_matches();

    let path = matches.value_of("PATH").expect("required argument");
    let path = Path::new(path);
    let data = std::fs::read(path).with_context(|| {
        format!(
            "Could not read binary proof file from \"{}\"",
            path.display()
        )
    })?;

    let mmb = Mmb::from(&data).unwrap();

    handle(path, mmb)?;

    Ok(())
}

fn print_offsets(mmb: &Mmb, range: &[u8], text: &str) {
    let offset = range.as_ptr() as usize - mmb.file().as_ptr() as usize;

    println!("{:16x} .. {:16x} {}", offset, offset + range.len(), text);
}

fn handle(path: &Path, mmb: Mmb) -> Result<()> {
    println!(
        "{}: mmb version {} ({} bytes)\n",
        path.display(),
        mmb.version(),
        mmb.file().len()
    );

    println!("File header:");
    println!("number of sorts: {}", mmb.num_sorts());
    println!("number of terms and definitions: {}", mmb.num_terms());
    println!("number of theorems and axioms: {}", mmb.num_theorems());

    println!("");
    println!("           start ..              end");

    print_offsets(&mmb, mmb.sorts(), "sort table");
    print_offsets(&mmb, mmb.terms(), "term table");
    print_offsets(&mmb, mmb.theorems(), "theorem table");
    print_offsets(&mmb, mmb.proofs(), "proof section");

    if let Some(index) = &mmb.index() {
        print_offsets(&mmb, index.sorts(), "sort index");
        print_offsets(&mmb, index.terms(), "term index");
        print_offsets(&mmb, index.theorems(), "theorem index");
    }

    Ok(())
}
