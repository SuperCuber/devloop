extern crate colored;
extern crate env_logger;
extern crate yaml_rust;

#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;

mod app;
mod configuration;
mod error;
mod message;
mod run;

fn main() {
    env_logger::init().expect("initialize logger");
    let args = app::parse_args();
    let configuration = configuration::load(args.value_of("file").expect("file argument"))
        .unwrap_or_else(|error| {
            error!("Failed to load configuration: {}", error);
            std::process::exit(1)
        });
    configuration.run();
}
