use std::{collections::BTreeMap, option};

use cooplan_definitions_lib::definition::Definition;
use lapin::{
    auth::Credentials,
    options::{BasicPublishOptions, QueueDeclareOptions},
    protocol::basic::AMQPProperties,
    types::{AMQPValue, FieldTable, LongString, ShortString},
    BasicProperties, Channel, Connection, ConnectionProperties, Queue,
};

use crate::{
    config::output_config::OutputConfig,
    error::{Error, ErrorKind},
};

pub struct RabbitMQOutput {
    connection_uri: String,
    connected: bool,
    output_config: OutputConfig,
    channel: Option<Channel>,
}

impl RabbitMQOutput {
    pub fn new(connection_uri: String, output_config: OutputConfig) -> RabbitMQOutput {
        RabbitMQOutput {
            connection_uri,
            connected: false,
            output_config,
            channel: None,
        }
    }

    pub async fn connect(&mut self) -> Result<(), Error> {
        let connection_options = ConnectionProperties::default()
            .with_executor(tokio_executor_trait::Tokio::current())
            .with_reactor(tokio_reactor_trait::Tokio);

        match Connection::connect(self.connection_uri.as_str(), connection_options).await {
            Ok(connection) => match connection.create_channel().await {
                Ok(channel) => {
                    let mut options = QueueDeclareOptions::default();
                    options.durable = true;
                    options.exclusive = false;
                    options.auto_delete = false;

                    let mut map: BTreeMap<ShortString, AMQPValue> = BTreeMap::new();
                    map.insert(
                        ShortString::from("x-queue-type"),
                        AMQPValue::LongString(LongString::from("stream")),
                    );

                    let arguments = FieldTable::from(map);

                    match channel
                        .queue_declare(
                            self.output_config.amqp_channel_name.as_str(),
                            options,
                            arguments,
                        )
                        .await
                    {
                        Ok(_) => {
                            self.connected = true;
                            self.channel = Some(channel);

                            Ok(())
                        }
                        Err(error) => Err(Error::new(
                            ErrorKind::ConnectionFailure,
                            format!("failed to create queue: {}", error).as_str(),
                        )),
                    }
                }
                Err(error) => Err(Error::new(
                    ErrorKind::ConnectionFailure,
                    format!("failed to connect: {}", error).as_str(),
                )),
            },
            Err(error) => Err(Error::new(
                ErrorKind::ConnectionFailure,
                format!("failed to connect: {}", error).as_str(),
            )),
        }
    }

    pub async fn set(&self, definition: Definition) -> Result<(), Error> {
        match serde_json::to_string(&definition) {
            Ok(serialized_definition) => match &self.channel {
                Some(channel) => {
                    match channel
                        .basic_publish(
                            "",
                            self.output_config.amqp_channel_name.as_str(),
                            BasicPublishOptions::default(),
                            serialized_definition.as_bytes(),
                            BasicProperties::default(),
                        )
                        .await
                    {
                        Ok(_) => Ok(()),
                        Err(error) => Err(Error::new(
                            ErrorKind::DataWritingFailure,
                            format!("failed to set the new definition: {}", error).as_str(),
                        )),
                    }
                }
                None => Err(Error::new(
                    ErrorKind::ChannelNotAvailable,
                    "channel is not available",
                )),
            },
            Err(error) => Err(Error::new(
                ErrorKind::SerializationFailure,
                format!("failed to serialize definition: {}", error).as_str(),
            )),
        }
    }
}
