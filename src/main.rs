use clap::Parser;
use dyn_fmt::AsStrFormatExt;
use futures::TryStreamExt;
use pulsar::{ConsumerOptions, message::proto::command_subscribe::SubType, Pulsar, TokioExecutor};
use pulsar::Consumer;
use pulsar::consumer::InitialPosition;
use serde::{Deserialize, Serialize};
use tokio::time::{Duration, timeout};

use findify_index_checker::findify::sync::*;

#[derive(Parser, Serialize, Deserialize, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    #[arg(short, long, env)]
    pub merchant_id: String,
    #[arg(long, env)]
    pub pulsar_service_url: String,
    #[arg(short, long, env)]
    pub environment: String,
    #[arg(short, long, default_value = "persistent://findify/index-updates-product-{}/{}")]
    pub pulsar_product_topic_template: String,
    #[arg(short, long, value_parser = parse_duration, default_value = "5")]
    pub timeout: Duration,
}

fn parse_duration(arg: &str) -> Result<std::time::Duration, std::num::ParseIntError> {
    let seconds = arg.parse()?;
    Ok(Duration::from_secs(seconds))
}

impl Arguments {
    pub fn product_index_updates_topic(&self) -> String {
        self.pulsar_product_topic_template.format(&[self.environment.as_str(), self.merchant_id.as_str()])
    }

    pub fn subscription_name(&self) -> String {
        format!("index-discovery-for-{}-{}", self.environment, self.merchant_id)
    }

    pub fn consumer_name(&self) -> String {
        format!("index-checker")
    }
}

async fn process_messages(timeout_duration: Duration, mut consumer: Consumer<IndexPublished, TokioExecutor>) -> Result<IndexPublished, pulsar::Error> {
    timeout(timeout_duration, async {
        let message = consumer.try_next().await.expect("No message");
        match message {
            Some(v) => {
                match v.deserialize() {
                    Ok(v) => Ok(v),
                    Err(e) => Err(pulsar::Error::Custom(format!("Deserialization problem - {}", e)))
                }
            }
            None => Err(pulsar::Error::Custom("No message".to_string()))
        }
    }).await.unwrap_or_else(|e| Err(pulsar::Error::Custom(format!("Crashed with {:?}", e))))
}


#[tokio::main]
async fn main() -> Result<(), pulsar::Error> {
    let arguments = &Arguments::parse();

    let pulsar: Pulsar<_> = Pulsar::builder(&arguments.pulsar_service_url, TokioExecutor).build().await?;

    println!(
        "Pulsar service URL: {}, Environment: {}, Topic: {}, Timeout: {:#?}",
        arguments.pulsar_service_url,
        arguments.environment,
        arguments.product_index_updates_topic(),
        arguments.timeout
    );

    let mut consumer: Consumer<IndexPublished, _> = pulsar
        .consumer()
        .with_topic(&arguments.product_index_updates_topic())
        .with_consumer_name(arguments.consumer_name())
        .with_subscription_type(SubType::Exclusive)
        .with_subscription(arguments.subscription_name())
        .with_options(
            ConsumerOptions::default()
                .with_initial_position(InitialPosition::Latest)
                .durable(false)
                .read_compacted(true)
        )
        .build()
        .await?;

    let index_published = process_messages(arguments.timeout, consumer).await?;
    println!("Found index (there could be more or never): {}", index_published.manifest.version);

    Ok(())
}
