# turtle-server
An intentionally slow http server. Useful for testing purposes.
It also focuses on one client at a time. No parallel waiting, it's like a queue.

You can customize 
- port
- slowness / response time (in seconds)
- name displayed in HTTP response

The HTTP response looks something like this: ```'alpha' sent you this message at 17:29:18 ```

## Running it

### Pre-compiled
Grab a release build and run it like this:

```
turtle-server -port 8080 -slowness 5 -name alpha
```
This runs it on port ```8080```. The reponse time will be ```5``` seconds and the http response will say the server is called ```alpha```
Adjust the values to your needs.

If a low number of keystrokes matters, try it like this.
```
turtle-server -p 8080 -s 5 -n alpha
```
### Source

Alternatively, you can also run it from source code. (Requires the Rust Toolchain)

```
cargo run -- -p 8080 -s 5 -n alpha
```




