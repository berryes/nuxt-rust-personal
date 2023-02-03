use rocket::{form::Form, http::{Status,},  serde::{json::Json, Deserialize}};
use serde::Serialize;
use std::{path::Path, fs::File};
mod db;
use uuid::Uuid;
 
#[macro_use] extern crate rocket;

#[derive(Debug,FromForm)]
struct PostInput {
    name: String,
    markdown: String,
}
#[post("/post/create", data = "<post_input>") ]
fn add_post(post_input: Form<PostInput> ) -> Status {
    
    let conn: sqlite::Connection = sqlite::open("./data.db")
    .expect("Failed to open database");


    if post_input.markdown.contains("'") {
        return  Status::BadRequest;
    }

    let query:String = format!("INSERT INTO posts VALUES (
        '{}', 
        '{}',
        '{}',
        '{}',
        '{}'
        ); ",

        Uuid::new_v4(), // uuid
        post_input.name, // alias or name given by user
        post_input.markdown,
        chrono::offset::Local::now(),
        chrono::offset::Local::now(),
    );

    conn.execute(query).unwrap();

    // returning 200 OK http status
    return Status::Accepted;
}

#[derive(Serialize)]
struct Post {
    id: String,
    name: String,
    markdown: String,
    created: String,
    edited: String   
}

#[get("/posts") ]
fn get_post() -> Json<Vec<Post>>{
    let conn: sqlite::Connection = sqlite::open("./data.db")
    .expect("Failed to open database");

    let query = "SELECT * FROM posts";

    let mut posts: Vec<Post> = Vec::new();

    conn
        .iterate(query, |pairs| {
            for &(name, value) in pairs.iter() {
                let mut post: Post = Post { 
                    id: String::new(),
                    name: String::new(),
                    markdown: String::new(),
                    created: String::new(),
                    edited: String::new() 
                };

                println!("{} > {:?}",name,value);

                match name.to_owned().as_str() {
                    "id" => post.id = value.unwrap().to_string(),
                    "name" => post.name = value.unwrap().to_string(),
                    "markdown" => post.markdown = value.unwrap().to_string(),
                    "created" => post.created = value.unwrap().to_string(),
                    "edited" => post.edited = value.unwrap().to_string(),
                    _ => print!("butt")
                }

                posts.push(post)
            }

            true
        })
        .unwrap();

        Json(posts)

        
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



    rocket::build().mount("/", routes![add_post,get_post])
}
