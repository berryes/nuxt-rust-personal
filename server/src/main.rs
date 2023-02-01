use rocket::{response::status,form::Form, http::{RawStr, Status}};
use std::{path::Path, fs::File};
mod db;

#[macro_use] extern crate rocket;

#[derive(FromForm)]
struct UserInput<'r> {
    markdown: &'r str,
    created: String,
    edited: String,
}

#[post("/post/add", data = "<user_input>") ]
fn add_post(user_input: Form<UserInput<'_>> ) -> Status<String> {
    
    let conn: sqlite::Connection = sqlite::open("./data.db")
    .expect("Failed to open database");
    
    if user_input.markdown.len() == 0 {
        status::BadRequest( Some("Markdwon is empty") );
    }
    if user_input.created // check date


    let query:String = format!("INSERT INTO posts VALUES ('{}', '{}', '{}', '{}'); ",);


    status::Accepted(Some("Post added"))
}


#[launch]
fn rocket() -> _ {
    
    if !Path::new("data.db").exists() {
        File::create("data.db")
        .expect("Error creating database");  
        
        
        let conn: sqlite::Connection = sqlite::open("./data.db")
        .expect("Failed to open database");
        
        // TODO: result< bool str>
        db::init(conn);

}



    rocket::build().mount("/", routes![add_post])
}
