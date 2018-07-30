use chrono;
use super::picture::Picture;
use super::user::User;
use schema::thumbnails;

#[derive(Identifiable, Queryable, Associations, Debug)]
#[belongs_to(Picture)]
#[belongs_to(User)]
#[table_name = "thumbnails"]
pub struct Thumbnail {
    pub id: String,
    pub small: String,
    pub normal: String,
    pub big: String,
    pub picture_id: String,
    pub user_id: String,
    pub created_date: chrono::NaiveDateTime,
    pub last_modified_date: Option<chrono::NaiveDateTime>,
    pub deleted_date: Option<chrono::NaiveDateTime>
}