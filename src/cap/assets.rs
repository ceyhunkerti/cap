use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "src/assets/"]
pub struct Assets;

#[derive(RustEmbed)]
#[folder = "src/assets/templates/"]
#[include = "*.html"]
pub struct Templates;

pub fn templates() -> Vec<String> {
    Templates::iter()
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
