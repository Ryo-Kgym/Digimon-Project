use async_graphql::Object;

use crate::photo::PHOTOS;

pub struct MyQuery;

#[Object]
impl MyQuery {
    async fn total_photos(&self) -> usize {
        PHOTOS.lock().unwrap().len()
    }

    async fn test_query(&self) -> String {
        String::from("test_query")
    }
}
