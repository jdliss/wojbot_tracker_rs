extern crate twitter_stream;

use twitter_stream::{Token, TwitterStreamBuilder};
use twitter_stream::rt::{self, Future, Stream};
use std::env;

fn from_env(key: &str) -> String {
    let os_str = env::var_os(key).expect("Could not get key from environment");
    os_str.into_string().expect("Could not convert os string into String.")
}

fn main() {
    let consumer_key = from_env("CONSUMER_KEY");
    let consumer_secret = from_env("CONSUMER_SECRET");
    let access_key = from_env("ACCESS_KEY");
    let access_secret = from_env("ACCESS_SECRET");

    let token = Token::new(
        consumer_key,
        consumer_secret,
        access_key,
        access_secret
    );

    let future = TwitterStreamBuilder::filter(&token)
        .track(Some("@wojespnbot"))
        .listen()
        .flatten_stream()
        .for_each(|json| {
            println!("{}", json);
            Ok(())
        })
        .map_err(|e| println!("error: {}", e));

    rt::run(future);
}
