extern crate futures;
extern crate nitox;
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

fn main() {
    let _ = connect_to_nats()
        .and_then(|client| {
            client.subscribe(SubCommand::builder().subject("Race").build().unwrap());
            ok(client)
        })
        .and_then(|client| {
            client.publish(
                PubCommand::builder()
                    .subject("Race")
                    .payload("1")
                    .build()
                    .unwrap(),
            )
        })
        .and_then(|()| {
            println!("Done !");
            ok(())
        });
}
