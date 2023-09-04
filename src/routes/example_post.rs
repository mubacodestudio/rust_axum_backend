pub async fn example_get() -> String {
    "Hello, World".to_owned()
}

pub async fn example_post(body: String) -> String {
    body
}
