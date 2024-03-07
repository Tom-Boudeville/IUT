use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "voting-system")]
pub struct Configuration {
    #[structopt(short, long)]
    pub candidates: Vec<String>,
}
