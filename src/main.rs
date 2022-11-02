use clap::Parser;
use extd::extract_td;
use extd::ExtractResult;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    link: String,
}

fn main() {
    let args = Cli::parse();
    println!("hold on...");
    let link = args.link.as_str();
    let result = extract_td(link);
    match result {
        Ok(result) => print_result(link, result),
        Err(err) => println!("err: {:?}", err),
    }
}

fn print_result(link: &str, result: ExtractResult) {
    println!("{link}'s title and description is: ");
    println!("title: {}", result.title);
    println!("description: {}", result.desc);
}
