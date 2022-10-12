# In memory DB with Rust

Run the Docker image with the following command:

```
docker build -t inmemodb .
docker run --name inmemodb -p3000:3000 --rm inmemodb
```

```telnet localhost 3000```

Interface is similar to Redis.
```
QUIT : closes client connection
SET KEY VALUE # where VALUE may be anything
GET KEY # returns the value associated with the KEY
```

TODO: implement a simple client implementation.