// main.rs
// cli handling

mod notify;

use clap::Parser;

/// A struct that gets the command line arguments via clap.
#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args 
{
    /// Title of the notification
    #[arg(short = 't', long = "title", value_name = "TITLE", default_value_t = String::new())]
    pub notif_title: String,
}

fn main() 
{
    let args = Args::parse();

    notify::notify(&args);
}
