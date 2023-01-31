use anyhow::{Error, Result};
use headless_chrome::types::PrintToPdfOptions;
use headless_chrome::{Browser, LaunchOptionsBuilder};
use std::io::{ErrorKind, Write};
use std::{fs, io};
use tempfile::NamedTempFile;
use tera::Context;
use tera::Tera;

fn to_html_file(context: &Context) -> String {
    let mut tera = Tera::default();
    tera.add_template_file("src/templates/resume/basic.jinja.html", Some("basic"))
        .unwrap();
    let html = tera.render("basic", context).unwrap();

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

pub fn to_pdf(context: &Context, out: &Option<String>) -> Result<(), Error> {
    let html_path = to_html_file(context);
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

pub mod elements {
    use serde::Serialize;

    #[derive(Debug, Serialize)]
    pub struct Icons<'a> {
        pub mail: &'a str,
        pub web: &'a str,
    }
    pub const ICONS: Icons = Icons {
        mail: r#"
            <svg width="16" height="16" viewBox="0 0 32 32" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
                <g id="icomoon-ignore">
                </g>
                <path d="M28.244 7.47h-25.572v17.060h26.656v-17.060h-1.084zM27.177 8.536l-10.298 10.298c-0.47 0.47-1.289 0.47-1.759 0l-10.3-10.298h22.356zM3.738 8.961l6.923 6.922-6.923 6.923v-13.846zM4.589 23.464l6.827-6.826 2.951 2.95c0.436 0.436 1.016 0.677 1.633 0.677s1.197-0.241 1.633-0.677l2.951-2.951 6.826 6.826h-22.822zM28.262 22.807l-6.923-6.924 6.923-6.924v13.848z" fill="\#000000">
                </path>
            </svg>
        "#,
        web: r#"
            <svg fill="\#000000" width="16" height="16" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                <path d="M3.51211712,15 L8.17190229,15 C8.05949197,14.0523506 8,13.0444554 8,12 C8,10.9555446 8.05949197,9.94764942 8.17190229,9 L3.51211712,9 C3.18046266,9.93833678 3,10.9480937 3,12 C3,13.0519063 3.18046266,14.0616632 3.51211712,15 L3.51211712,15 Z M3.93551965,16 C5.12590433,18.3953444 7.35207678,20.1851177 10.0280093,20.783292 C9.24889451,19.7227751 8.65216136,18.0371362 8.31375067,16 L3.93551965,16 L3.93551965,16 Z M20.4878829,15 C20.8195373,14.0616632 21,13.0519063 21,12 C21,10.9480937 20.8195373,9.93833678 20.4878829,9 L15.8280977,9 C15.940508,9.94764942 16,10.9555446 16,12 C16,13.0444554 15.940508,14.0523506 15.8280977,15 L20.4878829,15 L20.4878829,15 Z M20.0644804,16 L15.6862493,16 C15.3478386,18.0371362 14.7511055,19.7227751 13.9719907,20.783292 C16.6479232,20.1851177 18.8740957,18.3953444 20.0644804,16 L20.0644804,16 Z M9.18440269,15 L14.8155973,15 C14.9340177,14.0623882 15,13.0528256 15,12 C15,10.9471744 14.9340177,9.93761183 14.8155973,9 L9.18440269,9 C9.06598229,9.93761183 9,10.9471744 9,12 C9,13.0528256 9.06598229,14.0623882 9.18440269,15 L9.18440269,15 Z M9.3349823,16 C9.85717082,18.9678295 10.9180729,21 12,21 C13.0819271,21 14.1428292,18.9678295 14.6650177,16 L9.3349823,16 L9.3349823,16 Z M3.93551965,8 L8.31375067,8 C8.65216136,5.96286383 9.24889451,4.27722486 10.0280093,3.21670804 C7.35207678,3.81488234 5.12590433,5.60465556 3.93551965,8 L3.93551965,8 Z M20.0644804,8 C18.8740957,5.60465556 16.6479232,3.81488234 13.9719907,3.21670804 C14.7511055,4.27722486 15.3478386,5.96286383 15.6862493,8 L20.0644804,8 L20.0644804,8 Z M9.3349823,8 L14.6650177,8 C14.1428292,5.03217048 13.0819271,3 12,3 C10.9180729,3 9.85717082,5.03217048 9.3349823,8 L9.3349823,8 Z M12,22 C6.4771525,22 2,17.5228475 2,12 C2,6.4771525 6.4771525,2 12,2 C17.5228475,2 22,6.4771525 22,12 C22,17.5228475 17.5228475,22 12,22 Z"/>
            </svg>
        "#,
    };
}
