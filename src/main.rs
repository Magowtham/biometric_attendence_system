mod config;
mod mqtt;
mod db;
mod models;

use std::error::Error;

use crate::config::Settings;
use mqtt::operations::MqttService;
use db::operations::MongoDBService;


#[tokio::main]
async fn main()->Result<(),Box<dyn Error>> {

        let config_settings=Settings::new()?;

        let mut mqtt_service=MqttService::init(&config_settings.mqtt).await?;
        let db_service=MongoDBService::init(&config_settings.database).await?;


        loop {
           if let Ok(notification)=mqtt_service.event_loop.poll().await {
            if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(packete))=notification {
                mqtt_service.message_handler(&db_service,packete).await;
            }
           }
        }

    
}