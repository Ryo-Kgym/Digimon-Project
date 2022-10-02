use async_graphql::Object;

use crate::api::sample::photo::PHOTOS;

pub struct DigimonQuery;


#[Object]
impl DigimonQuery {
    async fn total_photos(&self) -> usize {
        PHOTOS.lock().unwrap().len()
    }

    async fn test_query(&self) -> String {
        String::from("test_query")
    }
}