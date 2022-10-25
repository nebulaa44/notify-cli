// main.rs
// cli handling

mod notify;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args 
{
    /// Title of the notification
    #[arg(short = 't', long = "title", default_value_t = String::new())]
    pub notif_title: String
}

fn main() 
{
    let args = Args::parse();

    notify::notify(&args);
}
