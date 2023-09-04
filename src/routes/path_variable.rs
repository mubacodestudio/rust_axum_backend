use axum::extract::Path;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestPath {
    id: i32,
    slug: String,
}

pub async fn path_variable(Path(path) : Path<RequestPath>) -> String {
    dbg!("{} {}", path.id, path.slug);
    todo!()
}