use shared::response_models::{Response, ResponseBody}
use app::post::{read};
use domain::models::{Post};
use rocket::{get};
use rocket::response::status::{NotFound};
use rocket::serde::json::Json;

#[get("/")]
pub fn list_posts_handler() -> String {
  todo!()
}

#[get("/posts/<post_id>")]
pub fn list_post_handler(post_id: i32) -> Result<String, NotFound<String>> {
  todo!()
}