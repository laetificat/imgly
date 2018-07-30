use rocket::response::content;

#[get("/")]
pub fn index() -> content::Json<&'static str> {
    content::Json("{ \"name\": \"Imgly\", \"version\": \"0.1.0\" }")
}