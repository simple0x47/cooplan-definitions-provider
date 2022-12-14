pub mod config;
pub mod definition;
pub mod error;

use std::io::{Error, ErrorKind};

use cooplan_definition_git_downloader::downloader::Downloader;
use cooplan_definition_git_downloader::version_detector::VersionDetector;
use definition::downloader_state::DownloaderState;
use definition::file_reader::FileReader;
use definition::output_async_wrapper::OutputAsyncWrapper;
use definition::reader_state::ReaderState;
use definition::{
    downloader_async_wrapper::DownloaderAsyncWrapper, rabbitmq_output::RabbitMQOutput,
};
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

    match run_definition_downloader().await {
        Ok(_) => (),
        Err(error) => {
            log::error!("{}", error);
        }
    }

    Ok(())
}

async fn run_definition_downloader() -> Result<(), Error> {
    let config = match crate::config::config_reader_builder::default().read() {
        Ok(config) => config,
        Err(error) => {
            return Err(error);
        }
    };

    let definition_downloader_state = DownloaderState::new(false);

    let (downloader_state_sender, mut downloader_state_receiver) =
        watch::channel(definition_downloader_state);

    let definition_reader_state = ReaderState::new_not_available();
    let (reader_state_sender, mut reader_state_receiver) = watch::channel(definition_reader_state);

    let version_detector_repository_local_dir = config.git().repository_local_dir;
    tokio::spawn(async move {
        let version_detector = VersionDetector::new(version_detector_repository_local_dir);

        let mut reader = FileReader::new(
            String::from("./categories/"),
            reader_state_sender,
            downloader_state_receiver,
            version_detector,
        );

        reader.run().await;
    });

    let connection_uri = match std::env::var("AMQP_CONNECTION_URI") {
        Ok(connection_uri) => connection_uri,
        Err(error) => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("failed to retrieve connection uri: {}", error),
            ))
        }
    };

    let output_config = config.output();

    tokio::spawn(async move {
        let mut output =
            RabbitMQOutput::new(connection_uri, output_config.amqp_channel_name.clone());

        let mut output_wrapper = OutputAsyncWrapper::new(output_config, output);

        output_wrapper.try_connect().await;

        if reader_state_receiver.borrow().available {
            let optional_definition = reader_state_receiver.borrow().definition();
            match optional_definition {
                Some(definition) => output_wrapper.try_set(definition).await,
                None => log::warn!("reader state is available, but definition is none"),
            }
        }

        loop {
            reader_state_receiver.changed().await;

            if reader_state_receiver.borrow().available {
                let optional_definition = reader_state_receiver.borrow().definition();
                match optional_definition {
                    Some(definition) => output_wrapper.try_set(definition).await,
                    None => log::warn!("reader state is available, but definition is none"),
                }
            }
        }
    });

    let definition_downloader_config = config.definition_downloader();
    let git_config = config.git();

    let download = task::spawn(async move {
        let definition_git_downloader = Downloader::new(git_config);
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
