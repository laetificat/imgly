use chrono::prelude::*;
use schema::users;
use uuid::Uuid;
use helper::time;

#[primary_key(username)]
#[derive(Serialize, Deserialize, Insertable, Identifiable, Queryable, PartialEq, Debug)]
pub struct User {
    pub username: String,
    pub display_name: String,
    pub bio: Option<String>,
    pub display_image: String,
    pub password: String,
    pub created_date: NaiveDateTime,
    pub last_modified_date: NaiveDateTime,
    pub deleted_date: Option<NaiveDateTime>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub display_image: Option<String>
}

impl User {
    pub fn new(new_user: NewUser) -> Result<User, UserCreationError> {
        let now_naive: NaiveDateTime = time::get_current_naive_date_time();

        let user: User = User {
            username: new_user.username.clone(),
            display_name: get_display_name(&new_user),
            bio: new_user.bio.clone(),
            display_image: get_display_image(&new_user),
            password: new_user.password.clone(),
            created_date: now_naive,
            last_modified_date: now_naive,
            deleted_date: None
        };

        Ok(user)
    }
}

fn get_display_name(new_user: &NewUser) -> String {
    match new_user.display_name {
        Some(ref name) => name.to_string(),
        None => new_user.username.clone()
    }
}

fn get_display_image(new_user: &NewUser) -> String {
    match new_user.display_image {
        Some(ref image) => image.to_string(),
        None => "/path/to/default/image.png".to_string()
    }
}

pub struct UserCreationError {}