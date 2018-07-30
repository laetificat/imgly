use chrono::prelude::*;
use super::album::Album;
use schema::pictures;
use helper::time;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Identifiable, Insertable, Queryable, Associations, Debug)]
#[belongs_to(Album)]
#[table_name = "pictures"]
pub struct Picture {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub score: i32,
    pub file: String,
    pub album_id: Option<String>,
    pub created_date: NaiveDateTime,
    pub last_modified_date: NaiveDateTime,
    pub deleted_date: Option<NaiveDateTime>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewPicture {
    pub name: String,
    pub description: Option<String>,
    pub file: String,
    pub album_id: Option<String>
}

impl Picture {
    pub fn new(new_picture: NewPicture) -> Result<Picture, PictureCreationError> {
        let timestamp: NaiveDateTime = time::get_current_naive_date_time();

        let picture: Picture = Picture {
            id: Uuid::new_v4().to_string(),
            name: new_picture.name.clone(),
            description: new_picture.description.clone(),
            score: 0,
            file: new_picture.file.clone(),
            album_id: new_picture.album_id.clone(),
            created_date: timestamp,
            last_modified_date: timestamp,
            deleted_date: None
        };

        Ok(picture)
    }
}

pub struct PictureCreationError {}