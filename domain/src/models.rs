use crate::schemas::posts;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::cmp::{Ord, Eq, PartialOrd, PartialEq};

#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct Post {
  pub id: i32,
  pub title: String,
  pub body: String,
  pub genre: String,
  pub published: bool,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = posts)]
pub struct NewPost {
  pub title: String,
  pub body: String,
  pub genre: String,
}