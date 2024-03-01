use database::{X1, PersonWithId};
use rocket::{response, State};

use crate::database::Database;
mod database;
#[macro_use]
extern crate rocket;

#[get("/")]
fn world() -> &'static str{
    "hello world"
}

use rocket::serde::json::Json;
#[get("/t")]
async fn sel(db:&State<Database>) -> Json<Vec<PersonWithId>> {
    let x = db.getf().await;
    // let x1 = x.c;
    dbg!(&x);
    x
}

#[get("/getusers")]
async fn sel1(db:&State<Database>) -> Json<Vec<X1>> {
    let x = db.getx().await;
    // let x1 = x.c;
    dbg!(&x);
    x
}

#[get("/getbyid?<idx>")]
async fn getuserbyid(db: &State<Database>,idx:String) -> Json<Vec<X1>>{
    let x = db.user_from_id(idx).await;
    x
}


#[post("/table?<user>")]
async fn ptable(db: &State<Database>,user:String) ->Result<response::content::RawJson<String>, response::status::Custom<String>>{
    match db.cptable(user).await {
        Ok(_) => Ok(response::content::RawJson(format!("table create"))),
        // Ok(_) => Ok(response::content::Json(format!("Table created successfully"))),
        Err(err) => Err(response::status::Custom(rocket::http::Status::InternalServerError, format!("Failed to create table: {}", err))),
    }
}



#[launch]
async fn rocket() -> _ {
    let db = database::Database::new().await.unwrap();
    rocket::build()
    .manage(db)
    .mount("/",routes![world,sel])
    .mount("/create",routes![ptable])
    .mount("/ft", routes![getuserbyid,sel1])

}