use async_graphql::Object;

use crate::api::sample::photo::{Photo, PHOTOS};

pub struct DigimonMutation;

#[Object]
impl DigimonMutation {

    async fn post_photo(&self, name: String, description: String) -> bool {
        let photo = Photo { name, description };
        PHOTOS.lock().unwrap().push(photo);
        true
    }
}