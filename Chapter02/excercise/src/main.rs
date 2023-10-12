use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "id_program", about = "A program that takes an ID as a command line argument")]
struct Opt {
    /// ID
    #[structopt(short = "i", long = "id")]
    id: String,
}

fn main() {
    let opt = Opt::from_args();

    println!("ID provided: {}", opt.id);
}