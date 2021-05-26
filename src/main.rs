use clap::{crate_version, App, Arg, SubCommand};
use std::error::Error;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const VER: &'static str = env!("VERGEN_GIT_SEMVER");

fn main() -> Result<(), Box<dyn Error>> {
    println!("VERSION: {}", VERSION);
    println!("VER: {}", VER);
    println!("Build Timestamp: {}", env!("VERGEN_BUILD_TIMESTAMP"));
    println!("git semver: {}", env!("VERGEN_GIT_SEMVER"));

    let matches = App::new("Clap subcommands")
        .version(crate_version!())
        .about("Experiment with dispatching subcommands")
        .subcommand(SubCommand::with_name("sub1").about("Sub command 1 has no parameters"))
        .subcommand(
            SubCommand::with_name("sub2")
                .about("Sub command 2 expects a SYMBOL parameter")
                .arg(
                    Arg::with_name("SYMBOL")
                        .help("Name of aseet")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    #[allow(dead_code)]
    enum Variants {
        V1,
        V2,
    }
    let variant = Variants::V2;
    match variant {
        Variants::V1 => {
            // If else if dispatch
            println!("Variants::V1");
            if matches.subcommand_matches("sub1").is_some() {
                println!("sub1");
            } else if let Some(matches) = matches.subcommand_matches("sub2") {
                let sym = match matches.value_of("SYMBOL") {
                    Some(s) => s,
                    None => return Err("No SYMBOL parameter".into()),
                };
                println!("sub2 SYMBOL={}", sym);
            } else {
                println!("Incorrect usage try: expr-clap-dispatching help");
            }
        }
        Variants::V2 => {
            // match dispatching
            println!("Variants::V2");
            let subcmd = match &matches.subcommand {
                Some(sc) => sc,
                None => return Err("No subcommand try: expr-clap-dispatching help".into()),
            };
            match subcmd.name.as_str() {
                "sub0" => {
                    println!("sub1");
                }
                "sub2" => {
                    let sym = match subcmd.matches.value_of("SYMBOL") {
                        Some(s) => s,
                        // None occurs if SYMBOL isn't defined for sub2, programming error!
                        None => return Err("No SYMBOL parameter".into()),
                    };
                    println!("sub2 SYMBOL={}", sym);
                }
                // Occurs if incorrect or incomplete match list, programming error!
                // For instance if "sub1" or "sub2" was misspelled or missing.
                _ => return Err(format!(
                    "Could not match subcommand {}, Report BUG: match list, version: {}, file: {}, line: {}",
                    subcmd.name,
                    VERSION,
                    file!(),
                    line!()
                )
                .into()),
            }
        }
    }

    Ok(())
}
