#[macro_use]
extern crate clap;
extern crate yaml_rust;

mod app;
mod configuration;
mod error;
mod message;
mod run;

fn main() {
    let args = app::parse_args();
    let configuration = configuration::load(args.value_of("file").expect("file argument"))
        .unwrap_or_else(|error| {
            message::msg(
                &message::MessageType::Fail,
                &format!("Failed to load configuration: {}", error),
                false,
            );
            std::process::exit(1)
        });
    configuration.run();
}
