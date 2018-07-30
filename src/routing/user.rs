use rocket_contrib::Json;
use rocket::response::status::{ BadRequest, NotFound };

use super::super::entity::user::{ User, NewUser };

use schema::users::dsl::*;
use schema::users;

use diesel::prelude::*;

#[get("/")]
pub fn list() -> Json<Vec<User>> {
    let connection = super::super::establish_connection();

    let results = users.filter(
        deleted_date.is_null()
    )
    .limit(25)
    .load::<User>(&connection)
    .expect("Could not load users");

    Json(results)
}

#[get("/<user_name>")]
pub fn get_user(mut user_name: String) -> Result<Json<Vec<User>>, NotFound<String>> {
    let not_found = Err(NotFound("user not found".to_string()));

    if user_name.remove(0) != '@' {
        return not_found;
    }

    let connection = super::super::establish_connection();

    let results = users.filter(
            username.eq(user_name)
        )
        .load::<User>(&connection)
        .expect("Error loading user");

    let mut users_vec: Vec<User> = vec![];

    for user in results {
        users_vec.push(user);
    };

    if users_vec.len() < 1 {
        return not_found;
    }

    Ok(Json(users_vec))
}

#[post("/", format = "application/json", data = "<user>")]
pub fn create(user: Json<NewUser>) -> Result<Json<User>, BadRequest<String>> {
    match User::new(user.into_inner()) {
        Ok(usr) => {
            let connection = super::super::establish_connection();

            let result: QueryResult<User> = diesel::insert_into(users::table)
                                                    .values(&usr)
                                                    .get_result(&connection);

            match result {
                Ok(usr) => Ok(Json(usr)),
                Err(_err) => Err(BadRequest(Some("Something went wrong creating a user".to_string())))
            }
        },
        Err(_err) => Err(BadRequest(Some("Something went wrong creating a user".to_string())))
    }
}