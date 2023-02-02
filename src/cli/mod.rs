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
    Init(Init),
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
    Gen(ResumeGenSub),
}

#[derive(Debug, Parser)]
pub struct ResumeGenSub {
    #[arg(short, long)]
    pub name: String,
    #[arg(short, long)]
    pub template: Option<String>,
    #[arg(short, long, global = true)]
    pub out: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum TemplateSub {
    List,
}

#[derive(Debug, Parser)]
pub struct Init {
    #[arg(short, long)]
    pub path: Option<String>,
}
