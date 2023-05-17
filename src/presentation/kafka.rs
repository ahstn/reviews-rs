use rdkafka::consumer::stream_consumer::StreamConsumerContext;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::Message;

pub async fn start_listener(brokers: &str, topic: &str) {
    let consumer: StreamConsumer<StreamConsumerContext> = StreamConsumer::from_hosts(vec![brokers])
        .with_group_id("my-group")
        .with_topic(topic)
        .create()
        .unwrap();

    println!("Started Kafka listener on topic {}", topic);

    for message in consumer.stream().await {
        match message {
            Ok(m) => {
                let key = match m.key_view::<str>() {
                    None => "".to_string(),
                    Some(Ok(s)) => s.to_string(),
                    Some(Err(_)) => "".to_string(),
                };
                let payload = match m.payload_view::<str>() {
                    None => "".to_string(),
                    Some(Ok(s)) => s.to_string(),
                    Some(Err(_)) => "".to_string(),
                };
                println!(
                    "key: '{}', payload: '{}', topic: {}, partition: {}, offset: {}",
                    key,
                    payload,
                    m.topic(),
                    m.partition(),
                    m.offset()
                );
            }
            Err(e) => println!("Kafka error: {}", e),
        }
    }
}
