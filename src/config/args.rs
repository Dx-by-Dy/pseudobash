use clap::command;

#[derive(clap::Parser, Debug)]
#[command(about, long_about = None)]
pub struct Args {
    /// The config file.
    #[arg(short, long, default_value_t = ("./config.json").to_string())]
    pub config_file: String,
}
