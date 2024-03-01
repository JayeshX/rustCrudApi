// use std::fmt::format;

use rocket::serde::json::Json;
use serde::Deserialize;
use serde::Serialize;
use surrealdb::engine::remote::ws::Client;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Error;
use surrealdb::Surreal;
#[derive(Debug, Serialize, Deserialize)]
pub struct PersonWithId {
    id: Thing,
    x1: String,
    x2: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct X1 { 
    name: String,
    occ: String,
}
pub struct Database{
    pub db: Surreal<Client>,
}

impl Database{
    pub async fn new() -> Result<Self,Error>{
        let db = Surreal::new::<Ws>("127.0.0.1:8001").await?;
        db.signin(Root{
            username: "root",
            password: "root"
        })
        .await?;
    db.use_ns("todo").use_db("todo").await?;
    Ok(Database{db})
    }
    // pub async fn create_table(&self) -> Result<(), Error> {
    //     self.db.query("DEFINE TABLE reading;").await?;
    //     Ok(())
    // }
    pub async fn cptable(&self,name:String) -> Result<(),Error>{
        self.db.query(format!("DEFINE TABLE {name};")).await?;
        Ok(())
    }

    pub async fn getf(&self)->Json<Vec<PersonWithId>>{
        let query = format!("SELECT * from reading;");
        let mut response = self.db.query(query).await.unwrap();
        let users: Vec<PersonWithId> = response.take(0).unwrap();
        Json(users)
    }
    pub async fn getx(&self)->Json<Vec<X1>>{
        let query = format!("SELECT * from reading;");
        let mut response = self.db.query(query).await.unwrap();
        let user: Vec<X1> = response.take(0).unwrap();
        Json(user)
    }

    pub async fn user_from_id(&self,id:String)->Json<Vec<X1>>{
        let query = format!("select name,occ from reading where id={id};");
        let mut response = self.db.query(query).await.unwrap();
        let user: Vec<X1> = response.take(0).unwrap();
        Json(user)
    }
}

