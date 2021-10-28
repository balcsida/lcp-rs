#[macro_use]
extern crate ini;
extern crate dirs;
use std::path::Path;
extern crate clap;
use clap::{Arg, App, SubCommand, crate_version, crate_authors, crate_description};


fn main() {
    let matches = App::new("Alternative Liferay Cloud Platform CLI (alcp)")
                          .version(crate_version!())
                          .author(crate_authors!())
                          .about(crate_description!())
                          .arg(Arg::with_name("config")
                              .short("c")
                              .long("config")
                              .value_name("FILE")
                              .help("Sets a custom config file")
                              .takes_value(true))
                          .arg(Arg::with_name("v")
                              .short("v")
                              .multiple(true)
                              .help("Sets the level of verbosity"))
                          .subcommand(SubCommand::with_name("delete")
                                      .about("Deletes service(s)")
                                      .arg(Arg::with_name("debug")
                                          .short("d")
                                          .help("print debug information verbosely")))
                          .get_matches();
    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    let path = Path::new(dirs::home_dir().unwrap().to_str().unwrap()).join(".lcp");
    let map = ini!(path.to_str().unwrap());
    // Proceed to use normal HashMap functions on the map:
    let lcp_token = map["remote \"lcp\""]["token"].clone().unwrap();
    println!("{}", lcp_token);

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("delete") {
        if matches.is_present("debug") {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }
}