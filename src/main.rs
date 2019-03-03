
#[macro_use]
mod util;
mod args;
mod config;
mod rule;



fn main() {
    println!("Hello, world!");

    let args = args::get_args();

    Debug!("Args = {:?}", args);

    config::maybe_create_config(args.configpath.as_str()).expect("Cannot create config");

    let conf = config::parse_config(&args).unwrap();


}
