use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Book {
    pub asin: String,
    pub authors: Vec<Author>,
    pub description: String,
    pub format_type: String,
    pub image: String,
    pub language: String,
    pub narrators: Vec<Narrator>,
    pub publisher_name: String,
    pub rating: String,
    pub release_date: String,
    pub runtime_length_min: i64,
    pub subtitle: String,
    pub summary: String,
    pub title: String,
    pub genres: Vec<Genre>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub asin: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Narrator {
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Genre {
    pub asin: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}
