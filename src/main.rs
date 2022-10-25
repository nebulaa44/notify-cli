// main.rs
// cli handling

mod notify;

use clap::Parser;

// TODO: Replace with a clap implementation that doesn't derive
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
