use clap::{App, Arg, SubCommand};

fn main() {
    let app = App::new("nbot")
        .version("0.1.0")
        .author("n0pj")
        .about("nbot is a scraping bot")
        .subcommand(SubCommand::with_name("twitter"));

    let matches = app.get_matches();

    if let Some(matches) = matches.subcommand_matches("twitter") {
        println!("twitter");
    }
}
