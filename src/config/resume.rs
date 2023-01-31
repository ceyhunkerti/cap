use anyhow::Ok;
use serde::Serialize;
use serde::{de::Error, Deserialize, Deserializer};

fn md_deserializer<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(markdown::to_html(&s)).map_err(D::Error::custom)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resume {
    pub default: Option<bool>,
    pub profile: Profile,
    #[serde(deserialize_with = "md_deserializer")]
    pub summary: String,
    pub experiences: Vec<Experience>,
    pub education: Vec<Education>,
    pub skills: Vec<Skill>,
    pub languages: Vec<Language>,
    pub projects: Vec<Project>,
    pub custom_sections: Option<Vec<Section>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    name: String,
    title: String,
    email: String,
    links: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Experience {
    company: String,
    start_date: String,
    end_date: Option<String>,
    title: String,
    tags: Option<Vec<String>>,
    #[serde(deserialize_with = "md_deserializer")]
    summary: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Education {
    degree: String,
    school: String,
    field: String,
    start_date: String,
    end_date: Option<String>,
    website: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Skill {
    name: String,
    level: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    name: String,
    level: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    name: String,
    #[serde(deserialize_with = "md_deserializer")]
    description: String,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Section {
    name: String,
    #[serde(deserialize_with = "md_deserializer")]
    summary: String,
}
