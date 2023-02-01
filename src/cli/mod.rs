use clap::{Parser, Subcommand};

// cli for your next job!
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    // Config file
    #[arg(short, long, global = true)]
    pub config: Option<String>,

    // resume command
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Resume(Resume),
    Template(Template),
}

#[derive(Debug, Parser)]
pub struct Resume {
    #[command(subcommand)]
    pub sub: ResumeSub,
}

#[derive(Debug, Parser)]
pub struct Template {
    #[command(subcommand)]
    pub sub: TemplateSub,
}

#[derive(Subcommand, Debug)]
pub enum ResumeSub {
    List,
    Gen(GenSub),
}

#[derive(Debug, Parser)]
pub struct GenSub {
    #[arg(short, long)]
    pub name: String,

    #[arg(short, long, global = true)]
    pub out: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum TemplateSub {
    List,
}
