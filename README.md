# Webserver

This is a containerized webserver built in rust using the [tokio](https://docs.rs/tokio/latest/tokio/) and the [warp](https://docs.rs/warp/latest/warp/) crates.
Tokio is an asynchronous runtime required to fullfill our multi-threaded necessities.
Warp is a simple webserver framework that allows us to build the API.
These are both pretty fast tools and so should the webserver, provided my code does not serve as a bottleneck.

The Webserver can simply be initialized with the command:
```bash
cargo run
```
or via its [container images](#running-images).

# API
The API handles:
* string queries (GET requests)
* multipart/form-data queries (POST requests)
* application/x-www-form-urlencoded (POST requests)
* get_data requests

## /put_data
The API is pretty simple, the put_data requests take in any amount of parameters and store them and their values as rust Strings.
The curl POST requests have a limit of 1KiB per value of each parameter (1024 characters), the curl GET requests and the [webserver_tester](https://gitlab.com/psem/recruitment-software/recruitment-tasks/-/blob/main/resources/webserver_tester.py)'s requests have a greater limit.

In the future Time will be a mandatory parameter.

The API returns the registered parameters and values in a random order in a json format:
```json
{
  "Time": "12:35:13",
  "Longitude": "-91389050",
  "Speed": "57",
  "Latitude": "387365578"
}
```
### /put_data GET requests
These are simple string queries structured in the following way:  
http://localhost:5000/put_data?Parameter=value

You can make string queries either on your browser or with curl:
```bash
curl "http://localhost:5000/put_data?Latitude=387365578&Longitude=-91389050&Time=12:35:13&Speed=57"
```

### /put_data POST requests (curl)
The following sends a POST request with the multipart/form-data format:
```bash
curl -X POST -F 'Latitude=387365578' -F 'Longitude=-91389050' -F 'Time=12:35:13' -F 'Speed=57' http://localhost:5000/put_data
```

### /put_data POST requests (the script)
The [python script](https://gitlab.com/psem/recruitment-software/recruitment-tasks/-/blob/main/resources/webserver_tester.py) sends POST requests with the application/x-www-form-urlencoded format.  
This can be sent with curl using the following command:
```bash
curl --data-urlencode "Latitude=387365578" \
     --data-urlencode "Longitude=-91389050" \
     --data-urlencode "Time=12:35:13" \
     --data-urlencode "Speed=57" \
     http://localhost:5000/put_data
```

## /get_data
This path receives GET requests and sends back the stored data formatted as json, like so:
```json
{
  "Latitude": [
    387366743, 
    387366468, 
    387366292
  ],
  "Longitude": [
    -91398238, 
    -91398209, 
    -91398214
  ],
  "Speed": [
    25, 
    30, 
    35
  ],
  "Time": [
    "12:35:13", 
    "12:35:14", 
    "12:35:15"
  ]
}
```

# Docker

There are two dockerfiles for the alpine and debian images of rust, the alpine image for the server is 20% the size of debian's but it relies on rust-musl-builder instead of rust's official image which might mean some functionality does not work (issues with OpenSSL seem to be common) but there seems to be no issue for the things we need.

##<a name="running-images"></a> Running the images

The alpine image has been pushed into the container registry so it can be run in few steps.
1.  Make sure that you are logged into the registry in docker, you can do so with:
```bash
docker login registry.gitlab.com
```
2. Now you just need to download and run the image which can be done with the command:
```bash
docker run --init -p 5000:5000 registry.gitlab.com/psem/recruitment-software/srgesus/rust-webserver/webserver-alpine
```

## Building the images
The images can also be both built but it takes more time and space.
Make sure you're inside the root directory of this repository.
Then you can do:
```bash
docker build -t rust-alpine -f ./alpine/Dockerfile .
```

```bash
docker build -t rust-debian -f ./debian/Dockerfile .
```
This will create a alpine and debian image of the server respectively and save it in your system under the names rust-alpine and rust-debian.

Then you can run them, respectively, with the commands:
```bash
docker run --init -p 5000:5000 rust-alpine
```
```bash
docker run --init -p 5000:5000 rust-debian
```

---

## To do - Webserver

* [x] Handle multiple requests simultaneously
* [x] Receive http requests with the following variables Latitude, Longitude, Speed, Time
    * [x] Handle put_data GET requests (string queries)
    * [x] Handle put_data POST requests 
        * [x] Handle curl's POST requests (multipart/form-data)
        * [x] Handle script's POST requests (application/x-www-form-urlencoded)
* [x] Provide an api that returns all the data already received - get_data GET requests
* [x] API description
* [ ] **Bonus**: add persistence
    * [ ] Send values to Influxdb

## To do - Docker

 * [x] Create dockerfile for webserver
 * [ ] Create docker compose for application
 * [ ] Have persistence with docker volumes
