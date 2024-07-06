use mongodb::{Client, Database,Collection};
use mongodb::error::Error;

use crate::config::DatabaseSettings;
use crate::models::Student;

pub struct MongoDBService {
    pub db:Database,
    pub student_collection:Collection<Student>
}


impl MongoDBService {

    pub async fn init(config_settings:&DatabaseSettings)->Result<Self,Error>{

    let client=Client::with_uri_str(&config_settings.uri).await?;

    let database=client.database(&config_settings.database_name);

    let student_collection=database.collection(&config_settings.collection_name);

    println!("connected to mongodb");

    return  Ok(Self {
        db:database,
        student_collection,
        });
    }

    pub async fn update_student_doc(self:&Self){
        
    }
}