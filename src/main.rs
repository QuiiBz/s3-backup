use structopt::StructOpt;

mod cli;

fn main() {

    let opt = cli::Cli::from_args();
    println!("{:?}", opt);
}
