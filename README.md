# turtle-server
An intentionally slow http server. Useful For testing purposes.
You can customize how long it takes to sends a response. It also focuses on one clinet at a time. No parallel waiting here! It's like a queue.

The HTTP response it sends looks something like this: ```'alpha' sent you this message at 17:29:18 ```

## Running it

### Pre-compiled
Grab a build release and run it like this:

```
turtle-server -port 8080 -slowness 5 -name alpha
```
This runs it on port ```8080```. The reponse time will be ```5``` seconds and the http response will say the server is called ```alpha```
Adjust the values to your needs.

If your number of keystrokes matter, try it like this.
```
turtle-server -p 8080 -s 5 -n alpha
```
### Source

Alternatively, you can also run it from source code.

```
cargo run -- -p 8080 -s 5 -n alpha
```
