// main.rs
// cli handling

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args 
{
    /// Title of the notification
    #[arg(short = 't', long = "title", default_value_t = String::new())]
    notif_title: String
}

fn main() 
{
    let args = Args::parse();

    println!("Title of notification: {}", args.notif_title);
}
