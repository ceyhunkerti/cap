use clap::{Parser, Subcommand};

/// cli for your next job!
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Config file
    #[arg(short, long, global = true)]
    pub config: Option<String>,
    /// resume command
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Resume(Resume),
    Template(Template),

    /// initialize config file
    Init(Init),
}

#[derive(Debug, Parser)]
pub struct Resume {
    #[command(subcommand)]
    pub sub: ResumeSub,
}

/// html resume template commands
#[derive(Debug, Parser)]
pub struct Template {
    #[command(subcommand)]
    pub sub: TemplateSub,
}

/// resume related commands
#[derive(Subcommand, Debug)]
pub enum ResumeSub {
    /// list resume entries in your config file
    List,
    /// generate new resume
    Gen(ResumeGenSub),
}

#[derive(Debug, Parser)]
pub struct ResumeGenSub {
    /// resume key in your config file
    #[arg(short, long)]
    pub name: String,
    /// template name or local path to the your custom template file
    #[arg(short, long)]
    pub template: Option<String>,
    /// output pdf file name eg. my_resume.pdf
    #[arg(short, long, global = true)]
    pub out: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum TemplateSub {
    /// list available templates
    List,
}

/// initialize config file
#[derive(Debug, Parser)]
pub struct Init {
    /// path to config file. eg. [~/documents, ./my_config.yml]
    #[arg(short, long)]
    pub path: Option<String>,
}
