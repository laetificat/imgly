use rocket_contrib::Json;
use rocket::response::status::{ BadRequest, NotFound };

use super::super::entity::album::{ Album, NewAlbum };

use schema::albums::dsl::*;
use schema::albums;

use diesel::prelude::*;

#[get("/<user_name>/albums")]
pub fn list(mut user_name: String) -> Result<Json<Vec<Album>>, NotFound<String>> {
    let not_found = Err(NotFound("album not found".to_string()));

    if user_name.remove(0) != '@' {
        return not_found;
    }

    let connection = super::super::establish_connection();

    let results = albums.filter(
        username.eq(user_name)
    )
    .limit(10)
    .load::<Album>(&connection)
    .expect("Could not load albums");

    Ok(Json(results))
}

#[get("/<user_name>/albums/<album_id>")]
fn get_album(mut user_name: String, album_id: String) -> Result<Json<Vec<Album>>, NotFound<String>> {
    let not_found = Err(NotFound("album not found".to_string()));

    if user_name.remove(0) != '@' {
        return not_found;
    }

    let connection = super::super::establish_connection();

    let results = albums.filter(
        id.eq(album_id)
    )
        .load::<Album>(&connection)
        .expect("Error loading album");

    let mut albums_vec: Vec<Album> = vec![];

    for album in results {
        albums_vec.push(album);
    };

    if albums_vec.len() < 1 {
        return not_found;
    }

    Ok(Json(albums_vec))
}

#[post("/<user_name>/albums", format = "application/json", data = "<album>")]
fn create(mut album: Json<NewAlbum>, mut user_name: String) -> Result<Json<Album>, BadRequest<String>> {
    if user_name.remove(0) != '@' {
        return Err(BadRequest(Some("Could not create album".to_string())));
    }

    album.username = Some(user_name);

    match Album::new(album.into_inner()) {
        Ok(albm) => {
            let connection = super::super::establish_connection();

            let result: QueryResult<Album> = diesel::insert_into(albums::table)
                                            .values(&albm)
                                            .get_result(&connection);

            match result {
                Ok(albm) => Ok(Json(albm)),
                Err(err) => Err(BadRequest(Some("Could not create album".to_string())))
            }
        },
        Err(err) => Err(BadRequest(Some("Could not create album".to_string())))
    }
}