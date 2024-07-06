use rumqttc::{AsyncClient, ClientError, EventLoop, MqttOptions, Publish, QoS};
use std::time::Duration;
use mongodb::bson::{doc,oid};

use crate::config::MqttSettings;
use crate::db::operations::MongoDBService;

pub struct MqttService {
    pub client:AsyncClient,
    pub event_loop:EventLoop
}

impl MqttService {
    pub async fn init(config_settings:&MqttSettings)->Result<Self,ClientError> {

        let mut mqtt_options=MqttOptions::new(&config_settings.client_id, &config_settings.host, config_settings.port);
        mqtt_options.set_keep_alive(Duration::from_secs(config_settings.keep_alive));

        let (client,event_loop)=AsyncClient::new(mqtt_options, 10);

        client.subscribe(&config_settings.subscribe_topic,QoS::AtLeastOnce).await?;

        return Ok(Self {
            client,
            event_loop
        });
    }

    pub async fn message_handler(self:&Self,db:&MongoDBService,packete:Publish) {
        
        let id=String::from_utf8(packete.payload.to_vec()).unwrap();

        let doc_id=oid::ObjectId::parse_str(id).unwrap();
        
        let filter_query=doc! {"_id":doc_id};

        let update_query=doc! {
            "$inc":{
                "attended_classes":1
            }
        };

        let update_result=db.student_collection.update_one(filter_query,update_query).await.unwrap();
                                                               
        println!("{:?}",update_result);
        
    }
}