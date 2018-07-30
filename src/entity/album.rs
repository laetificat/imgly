use chrono::prelude::*;
use schema::albums;
use uuid::Uuid;
use helper::time;
use entity::user::User;

#[derive(Serialize, Deserialize, Insertable, Identifiable, Queryable, Associations, Debug)]
#[belongs_to(User, foreign_key = "username")]
#[table_name = "albums"]
pub struct Album {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub cover_image: Option<String>,
    pub username: Option<String>,
    pub created_date: NaiveDateTime,
    pub last_modified_date: NaiveDateTime,
    pub deleted_date: Option<NaiveDateTime>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewAlbum {
    pub name: String,
    pub description: Option<String>,
    pub cover_image: Option<String>,
    pub username: Option<String>
}

impl Album {
    pub fn new(new_album: NewAlbum) -> Result<Album, AlbumCreationError> {
        let timestamp: NaiveDateTime = time::get_current_naive_date_time();

        let album: Album = Album {
            id: Uuid::new_v4().to_string(),
            name: new_album.name,
            description: new_album.description,
            cover_image: new_album.cover_image,
            username: new_album.username,
            created_date: timestamp,
            last_modified_date: timestamp,
            deleted_date: None
        };

        Ok(album)
    }
}

pub struct AlbumCreationError {}