use std::env;
use clap::{Arg, App};


fn main() {

    let matches = App::new("fast-magic")
        .author("https://github.com/bdlmt")
        .about("Concurrent file search, using libmagic and regex.")
        .arg(Arg::with_name("directory")
            .short("d")
            .long("directory")
            .takes_value(true)
            .help("Directory to walk"))
        .arg(Arg::with_name("regex")
            .short("r")
            .long("regex")
            .takes_value(true)
            .help("Regex pattern to match"))
        .get_matches();


    let n_workers = num_cpus::get();
    println!("{}", n_workers);


}
