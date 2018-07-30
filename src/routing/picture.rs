use rocket_contrib::Json;
use rocket::response::status::{ BadRequest, NotFound };

use entity::picture::{ Picture, NewPicture };

use schema::pictures::dsl::*;
use schema::pictures;

use diesel::prelude::*;

#[get("/<user_name>/albums/<albumid>/pictures")]
pub fn list(mut user_name: String, albumid: String) -> Result<Json<Vec<Picture>>, NotFound<String>> {
    let not_found = Err(NotFound("album not found".to_string()));

    if user_name.remove(0) != '@' {
        return not_found;
    }

    let connection = super::super::establish_connection();

    let results = pictures.filter(
        album_id.eq(albumid)
    )
    .limit(10)
    .load::<Picture>(&connection)
    .expect("Could not load pictures");

    Ok(Json(results))
}

#[post("/<user_name>/albums/<albumid>/pictures", format = "application/json", data = "<pictures_json>")]
pub fn upload(user_name: String, albumid: String, pictures_json: Json<Vec<NewPicture>>) -> Result<Json<Vec<Picture>>, BadRequest<String>> {
    let mut pictures_vector: Vec<Picture> = vec![];
    let mut failed_pictures: Vec<&NewPicture> = vec![];

    for pic in pictures_json.into_iter() {
        pic.album_id = Some(albumid);

        match Picture::new(pic) {
            Ok(p) => pictures_vector.push(p),
            Err(err) => {
                failed_pictures.push(&pic)
            }
        }
    }

    if failed_pictures.len() > 0 {
        println!("{:?}", failed_pictures);
        return Err(BadRequest(Some("Could not save images".to_string())));
    }

    let connection = super::super::establish_connection();

    for pic in pictures_vector {
        let result: QueryResult<Picture> = diesel::insert_into(pictures::table)
                                            .values(&pic)
                                            .get_result(&connection);
    }

    Ok(Json(pictures_vector))
}