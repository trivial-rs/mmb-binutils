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

fn main() {
    let app = create_clap_app();
    let matches = app.get_matches();

    let x = matches.value_of("PATH").expect("required argument");

    println!("{}", x);
}
