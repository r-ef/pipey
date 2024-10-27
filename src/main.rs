use anyhow::Result;
use futures_util::StreamExt;
use redis::{aio::MultiplexedConnection, AsyncCommands, Client, Commands, Connection};

#[tokio::main]
async fn main() {
    let client = Client::open("redis://127.0.0.1:6379/").unwrap();
    let mut con = client.get_multiplexed_async_connection().await.unwrap();

    // Spawn the subscriber in a separate task
    let client_clone = client.clone();
    let subscriber = tokio::spawn(async move {
        pubsub_sub(client_clone).await.unwrap();
    });

    // Small delay to ensure subscriber is ready
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Send message
    pubsub_send(con).await.unwrap();

    // Wait for the subscriber
    subscriber.await.unwrap();
}

async fn pubsub_sub(client: Client) -> Result<()> {
    let (mut sink, mut stream) = client.get_async_pubsub().await?.split();
    sink.subscribe("channel_1").await?;
    loop {
        let msg = stream.next().await.unwrap();
        let payload: String = msg.get_payload().unwrap();
        println!("channel '{}': {}", msg.get_channel_name(), payload);
    }
}

async fn pubsub_send(mut con: MultiplexedConnection) -> Result<()> {
    // con.publish("channel_1", "Hello, Redis!").await?;
    for _ in 0..10 {
        con.publish("channel_1", "Hello, Redis!").await?;
    }
    Ok(())
}
