use async_graphql::Object;

use crate::photo::{Photo, PHOTOS};

pub struct MyMutation;


#[Object]
impl MyMutation {
    async fn post_photo(&self, name: String, description: String) -> bool {
        let photo = Photo { name, description };
        PHOTOS.lock().unwrap().push(photo);
        true
    }
}