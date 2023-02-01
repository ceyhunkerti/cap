extern crate markdown;
extern crate tera;

mod cap;
mod cli;
mod config;
mod console;

use clap::Parser;
use cli::{GenSub, ResumeSub, TemplateSub};

use crate::cli::{Args, Commands};
use crate::config::Config;
use cap::html::template::{templates, to_pdf};

fn main() {
    let args = Args::parse();

    match &args {
        Args { config, command } => match command {
            Some(Commands::Resume(resume)) => match &resume.sub {
                ResumeSub::List => {
                    console::print(
                        Config::new(config.to_owned())
                            .resumes()
                            .iter()
                            .map(|(d, r)| vec![if *d { "*" } else { "" }, r])
                            .collect(),
                    );
                }
                ResumeSub::Gen(GenSub { name, out }) => {
                    let config = Config::new(config.to_owned());
                    let resume = config.resume(name).expect("resume `{name}` not found");
                    to_pdf(&resume, out, &config.template).expect("failed to create pdf");
                }
            },
            Some(Commands::Template(template)) => match &template.sub {
                TemplateSub::List => {
                    console::print(templates().iter().map(|f| vec![f]).collect());
                }
            },
            _ => {}
        },
    }
}
