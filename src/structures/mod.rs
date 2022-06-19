pub mod project_structs;
pub mod tag_structs;
pub mod user_structs;
pub mod version_structs;

pub type Datetime = chrono::DateTime<chrono::Utc>;
pub type Number = usize;
pub type URL = url::Url;
/// A base 62 number stored as a string
pub type ID = String;
