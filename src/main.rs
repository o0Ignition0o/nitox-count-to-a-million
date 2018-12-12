extern crate futures;
extern crate nitox;
extern crate tokio;
use futures::{future::ok, prelude::*};
use nitox::{commands::*, NatsClient, NatsClientOptions, NatsError};

fn connect_to_nats() -> impl Future<Item = NatsClient, Error = NatsError> {
    let connect_cmd = ConnectCommand::builder().build().unwrap();
    let options = NatsClientOptions::builder()
        .connect_command(connect_cmd)
        .cluster_uri("127.0.0.1:4222")
        .build()
        .unwrap();

    NatsClient::from_options(options)
        .and_then(|client| client.connect())
        .and_then(|client| ok(client))
}

fn unsubscribe_from_race(client: NatsClient) -> impl Future<Item = NatsClient, Error = NatsError> {
    client
        .unsubscribe(UnsubCommand::from(
            SubCommand::builder().subject("Race").build().unwrap(),
        ))
        .and_then(|_| ok(client))
}

fn handle_message_stream(
    message_stream: impl Stream<Item = Message, Error = NatsError> + Send + 'static,
) -> impl Future<Item = (), Error = NatsError> {
    tokio::spawn(
        message_stream
            .for_each(move |msg| {
                println!("Received a message ! {:?}", msg);
                ok(())
            })
            .into_future()
            .map_err(|_| ()),
    );
    ok(())
}

fn subscribe_to_race(client: NatsClient) -> impl Future<Item = NatsClient, Error = NatsError> {
    client
        .subscribe(SubCommand::builder().subject("Race").build().unwrap())
        .and_then(move |message_stream| handle_message_stream(message_stream))
        .and_then(|_| ok(client))
}

fn publish_to_race(client: NatsClient) -> impl Future<Item = NatsClient, Error = NatsError> {
    client
        .publish(
            PubCommand::builder()
                .subject("Race")
                .payload("Hello world !")
                .build()
                .unwrap(),
        )
        .and_then(|_| ok(client))
}

fn get_tasks_to_perform() -> impl Future<Item = (), Error = ()> {
    connect_to_nats()
        .and_then(subscribe_to_race)
        .and_then(publish_to_race)
        .and_then(unsubscribe_from_race)
        .and_then(|_| {
            println!("Done !");
            ok(())
        })
        .map_err(|_| println!("Something terrible happened !"))
}

fn main() {
    let tasks = get_tasks_to_perform();
    tokio::run(tasks);
}
