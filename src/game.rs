mod process;

use lapin::{
    message::Delivery,
    options::*,
    publisher_confirm::PublisherConfirm,
    types::{AMQPValue, FieldTable},
    BasicProperties, Channel, Connection,
};
use process::players::get_players;

pub async fn tick(conn: &Connection) {
    // process tiles
    //// process rocks
    //// process trees
    //// process herbs
    //// process fruits
    //
    //// process player actions
    get_players().iter().for_each(|player| {
        player.process();
    });

    let channel = match conn.create_channel().await {
        Ok(channel) => channel,
        Err(e) => panic!("Error creating channel: {:?}", e),
    };

    // let mut consumer = match channel
    //     .basic_consume(
    //         "to_process",
    //         "test",
    //         BasicConsumeOptions::default(),
    //         FieldTable::default(),
    //     )
    //     .await
    // {
    //     Ok(consumer) => consumer,
    //     Err(e) => {
    //         println!("Error consuming message: {:?}", e);
    //         return ();
    //     }
    // };

    loop {
        let delivery = match channel
            .basic_get("to_process", BasicGetOptions::default())
            .await
        {
            Ok(Some(delivery)) => delivery.delivery,
            Ok(None) => break,
            Err(e) => {
                println!("Error receiving message: {:?}", e);
                break;
            }
        };

        println!(
            "Received message: {}",
            match std::str::from_utf8(&delivery.data) {
                Ok(s) => s,
                Err(_) => "Invalid UTF-8",
            }
        );

        match process_task(&channel, delivery).await {
            Ok(_) => println!("Processed message"),
            Err(_) => {
                println!("Error processing message");
                break;
            }
        };
    }
}

pub async fn publish_task_with_timestamp(
    channel: &Channel,
    task_data: &str,
    timestamp: u64,
) -> Result<PublisherConfirm, lapin::Error> {
    let mut headers = FieldTable::default();
    headers.insert(
        "x-delayed-type".into(),
        AMQPValue::LongString("direct".into()),
    );
    headers.insert("x-delay".into(), AMQPValue::ShortInt(3000));
    let properties = BasicProperties::default()
        .with_timestamp(timestamp)
        .with_headers(headers);

    channel
        .basic_publish(
            "process_exchange",
            "process",
            BasicPublishOptions::default(),
            task_data.as_bytes(),
            properties,
        )
        .await
}

pub async fn process_task(channel: &Channel, delivery: Delivery) -> Result<(), ()> {
    let Some(timestamp) = delivery.properties.timestamp() else {
        return Err(());
    };
    let current_tick = chrono::Local::now().timestamp() as u64;

    println!("Timestamp: {}", timestamp);
    println!("Current tick: {}", current_tick);

    // Check if the message is too new (e.g., created on a tick we have not yet processed)
    if timestamp >= &current_tick {
        // Reject and requeue the message
        match channel
            .basic_reject(delivery.delivery_tag, BasicRejectOptions { requeue: true })
            .await
        {
            Ok(_) => {
                println!("Successfully rejected message due to it being too new");
                return Err(());
            }
            Err(e) => {
                println!("Error rejecting message: {:?}", e);
                return Err(());
            }
        }
    } else {
        // Acknowledge the message
        match channel
            .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
            .await
        {
            Ok(_) => return Ok(()),
            Err(e) => {
                println!("Error acknowledging message: {:?}", e);
                return Err(());
            }
        }
    }
}
