extern crate markdown;
extern crate tera;

mod cap;
mod cli;
mod config;
mod console;

use clap::Parser;
use cli::{ResumeGenSub, ResumeSub, TemplateSub};

use crate::cli::{Args, Commands};
use crate::config::Config;
use cap::assets::templates;
use cap::html::template::to_pdf;

fn main() {
    let args = Args::parse();

    match &args {
        Args { config, command } => match command {
            Some(Commands::Init(init)) => cap::init(init.path.to_owned()),
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
                ResumeSub::Gen(ResumeGenSub {
                    name,
                    out,
                    template,
                }) => {
                    let config = Config::new(config.to_owned());
                    let resume = config.resume(name).expect("resume `{name}` not found");
                    to_pdf(
                        &resume,
                        out,
                        template
                            .to_owned()
                            .unwrap_or(config.template.to_owned())
                            .as_str(),
                    )
                    .expect("failed to create pdf");
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
