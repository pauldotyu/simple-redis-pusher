# simple-redis-pusher

A simple app to push random words to a Redis list.

To use this app, you need to have Redis installed and running.

Create a new `.env` file in the root of the project directory and add the following:

```text
REDIS_HOST="<YOUR_REDIS_HOST>"
REDIS_PORT="<YOUR_REDIS_PORT>"
REDIS_KEY="<YOUR_REDIS_KEY>"
ITEM_COUNT=<NUMBER_OF_WORDS_TO_PUSH>
```

Run the app using the `cargo run` command.

To verify that the items were pushed to Redis, you can use the `redis-cli` tool:

```bash
source .env
docker run -it --rm redis redis-cli -h $REDIS_HOST -a $REDIS_KEY llen words
```

To run this in a container, you can use the following command to build and run the image:

```bash
docker build -t simple-redis-pusher:v1 .

source .env
docker run -e REDIS_HOST=$REDIS_HOST -e REDIS_PORT=$REDIS_PORT -e REDIS_KEY=$REDIS_KEY -e ITEM_COUNT=$ITEM_COUNT simple-redis-pusher:v1
```