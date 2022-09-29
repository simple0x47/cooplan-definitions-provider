pub mod config;
pub mod definition;
pub mod error;
pub mod git;

use std::io::{Error, ErrorKind};

use cooplan_definitions_lib::definition::Definition;
use definition::downloader_state::DownloaderState;
use definition::file_reader::FileReader;
use definition::git_downloader::GitDownloader;
use definition::reader_state::ReaderState;
use definition::{
    downloader_async_wrapper::DownloaderAsyncWrapper, rabbitmq_output::RabbitMQOutput,
};
use lapin::protocol::channel;
use tokio::{sync::watch, task};

#[tokio::main]
async fn main() -> Result<(), Error> {
    match simple_logger::SimpleLogger::new().env().init() {
        Ok(_) => (),
        Err(error) => {
            return Err(Error::new(
                ErrorKind::Interrupted,
                format!("failed to initialize logger: {}", error),
            ));
        }
    }

    run_definition_downloader().await;

    Ok(())
}

async fn run_definition_downloader() -> Result<(), Error> {
    let config = crate::config::config_reader_builder::default().read()?;

    let definition_downloader_state = DownloaderState::new(false);

    let (downloader_state_sender, mut downloader_state_receiver) =
        watch::channel(definition_downloader_state);

    let definition_reader_state = ReaderState::new_not_available();
    let (reader_state_sender, mut reader_state_receiver) = watch::channel(definition_reader_state);
    tokio::spawn(async move {
        let mut reader = FileReader::new(
            String::from("./categories/"),
            reader_state_sender,
            downloader_state_receiver,
        );

        reader.run().await;
    });

    let connection_uri = std::env::var("AMQP_CONNECTION_URI").unwrap();
    let channel_name = config.amqp_channel();

    tokio::spawn(async move {
        let mut output = RabbitMQOutput::new(connection_uri, channel_name);

        match output.connect().await {
            Ok(_) => {
                if reader_state_receiver.borrow().available {
                    let categories = reader_state_receiver.borrow().categories();
                    match output
                        .set(Definition::new("1".to_string(), categories))
                        .await
                    {
                        Ok(_) => (),
                        Err(error) => {
                            println!("error: {}", error);
                            return;
                        }
                    }
                }
            }
            Err(error) => {
                println!("error: {}", error);
                return;
            }
        }

        loop {
            reader_state_receiver.changed().await;

            if reader_state_receiver.borrow().available {
                let categories = reader_state_receiver.borrow().categories();
                match output
                    .set(Definition::new("1".to_string(), categories))
                    .await
                {
                    Ok(_) => (),
                    Err(error) => {
                        println!("error: {}", error);
                        return;
                    }
                }
            }
        }
    });

    let definition_downloader_config = config.definition_downloader();
    let git_config = config.git();

    let download = task::spawn(async move {
        let definition_git_downloader = GitDownloader::new(git_config);
        let mut definition_wrapper = DownloaderAsyncWrapper::new(
            definition_git_downloader,
            definition_downloader_config,
            downloader_state_sender,
        );

        definition_wrapper.run().await;
    });

    download.await;

    Ok(())
}
