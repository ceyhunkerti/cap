extern crate markdown;
extern crate tera;

mod cap;
mod cli;
mod config;
mod console;

use clap::Parser;
use cli::{GenSub, ResumeSub};

use crate::cli::{Args, Commands};
use crate::config::Config;
use cap::html::to_pdf as html_to_pdf;
use tera::Context;

fn main() {
    let args = Args::parse();

    match &args {
        Args { config, command } => match command {
            Some(Commands::Resume(resume)) => match &resume.sub {
                ResumeSub::List => {
                    console::print(
                        Config::new(config.to_owned())
                            .resume_list()
                            .iter()
                            .map(|(d, r)| vec![if *d { "*" } else { "" }, r])
                            .collect(),
                    );
                }
                ResumeSub::Gen(GenSub { name, out }) => {
                    let mut config = Config::new(config.to_owned());
                    let resume = config.resume(name).expect("resume `{name}` not found");
                    let mut context = Context::new();
                    context.insert("resume", resume);
                    context.insert("ICONS", &cap::html::elements::ICONS);
                    html_to_pdf(&context, out).expect("failed to create pdf");
                }
            },
            _ => {}
        },
    }
}
