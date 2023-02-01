use anyhow::{Error, Result};
use headless_chrome::types::PrintToPdfOptions;
use headless_chrome::{Browser, LaunchOptionsBuilder};
use rust_embed::RustEmbed;
use std::io::{ErrorKind, Write};
use std::path::Path;
use std::{fs, io};
use tempfile::NamedTempFile;
use tera::{Context, Tera};

use crate::config::resume::Resume;

#[derive(RustEmbed)]
#[folder = "src/templates/"]
#[include = "*.html"]
struct Template;

pub fn templates() -> Vec<String> {
    Template::iter()
        .map(|f| {
            f.as_ref()
                .to_string()
                .split(".")
                .next()
                .unwrap_or("")
                .to_owned()
        })
        .collect()
}

fn read(template: &str) -> String {
    // check if it's a file path in local file system
    if Path::new(format!("{template}").as_str()).is_file() {
        let content = fs::read_to_string(template).expect("Unable to read template");
        content
    } else {
        // read from the embedded template
        let f = Template::get(format!("{template}.html").as_str()).unwrap();
        std::str::from_utf8(f.data.as_ref()).unwrap().to_owned()
    }
}

fn to_html(context: &Context, template: &str) -> String {
    let mut tera = Tera::default();
    tera.add_raw_template(template, &read(template)).unwrap();
    let html = tera.render(template, context).unwrap();

    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "{}", html).unwrap();
    file.flush().unwrap();

    let path = dunce::canonicalize(file.path().to_str().unwrap()).unwrap();
    let ospath = path
        .as_os_str()
        .to_str()
        .ok_or_else(|| io::Error::from(ErrorKind::InvalidInput))
        .unwrap();

    let html_path = format!("{}.html", ospath);

    file.persist(&html_path).unwrap();
    html_path
}

pub fn to_pdf(resume: &Resume, out: &Option<String>, template: &str) -> Result<(), Error> {
    let mut context = Context::new();
    context.insert("resume", resume);
    context.insert("ICONS", &super::elements::ICONS);

    let html_path = to_html(&context, template);
    let options = LaunchOptionsBuilder::default()
        .build()
        .expect("Default should not panic");

    let browser = Browser::new(options).expect("failed to initiate browser");
    let tab = browser
        .wait_for_initial_tab()
        .expect("failed to initiate browser tab");

    let path = format!("file://{html_path}");

    tab.navigate_to(path.as_str())
        .unwrap()
        .wait_until_navigated()
        .unwrap();

    let pdf = tab.print_to_pdf(Some(PrintToPdfOptions::default()));
    fs::write(
        out.to_owned().unwrap_or("my_resume.pdf".to_string()),
        pdf.unwrap(),
    )?;
    Ok(())
}
