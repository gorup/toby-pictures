#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use serde_json::json;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

lazy_static! {
    static ref PICS: HashMap<String, &'static str> = {
        let mut m = HashMap::new();
        m.insert("bed".to_string(), "Toby is getting sleepy on the bed!");
        m.insert("beachdad".to_string(), "Toby and his pops hanging out on the Washington coast.");
        m.insert("beach".to_string(), "Toby loves to swim");
        m.insert("frontdoor".to_string(), "Toby enjoys looking out the front door to look for people and crows.");
        m
    };
}


#[get("/")]
async fn index() -> Template {
    Template::render(
        "index",
        json!({
            "now": now(),
            "hostname": hostname(),
        }),
    )
}

#[get("/about")]
async fn about() -> Template {
    Template::render(
        "about",
        json!({
            "now": now(),
            "hostname": hostname(),
        }),
    )
}


#[get("/pic/<name>")]
async fn largepic(name: String) -> Option<Template> {
    let desc = match PICS.get(&name) {
        Some(d) => d,
        None => return None,
    };

    Some(Template::render(
        "bigpic",
        json!({
            "name": name,
            "desc": desc,
            "now": now(),
            "hostname": hostname(),
        }),
    ))
}

fn now() -> usize {
    let now = SystemTime::now();
    now.duration_since(UNIX_EPOCH).unwrap().as_millis() as usize
}

fn hostname() -> String {
    hostname::get().unwrap().into_string().unwrap()
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    rocket::ignite()
        .mount("/static", StaticFiles::from("static/"))
        .attach(Template::fairing())
        .mount("/", routes![
            index,
            largepic,
            about,
        ])
        .launch().await.unwrap();

    Ok(())
}