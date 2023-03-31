use dotenv::dotenv;
use redis::Commands;
use std::env;

fn main() {
    dotenv().ok();

    // set redis host name
    let redis_host = env::var("REDIS_HOST").expect("REDIS_HOST must be set");

    // set redis port
    let redis_port = env::var("REDIS_PORT").expect("REDIS_PORT must be set");

    // set redis key
    let redis_key = env::var("REDIS_KEY").expect("REDIS_KEY must be set");

    // set redis list
    let redis_list = env::var("REDIS_LIST").expect("REDIS_LIST must be set");

    // connect to redis
    let client = redis::Client::open(format!(
        "redis://:{}@{}:{}/",
        redis_key, redis_host, redis_port
    ))
    .unwrap();

    // add random words to a list
    let mut con = client.get_connection().unwrap();

    // number of words to put in the list
    let count = env::var("ITEM_COUNT")
        .expect("ITEM_COUNT must be set")
        .parse::<i32>()
        .unwrap();

    // add words to the list
    for i in 0..count {
        let word = random_word::gen();
        println!("{} {}", i + 1, word);
        let _: () = con.rpush(&redis_list, word).unwrap();
    }
}
