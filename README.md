# Webserver / Docker

This is a webserver built in rust using the [tokio](https://docs.rs/tokio/latest/tokio/) and the [warp](https://docs.rs/warp/latest/warp/) crates.
Tokio is an asynchronous runtime required to fullfill our multi-threaded necessities.
Warp is a simple webserver framework that allows us to build the API.
These are both pretty fast tools and so should the webserver, provided my code does not serve as a bottleneck.

---

## To do - Webserver

* [x] Handle multiple requests simultaneously
* [x] Receive http requests with the following variables Latitude, Longitude, Speed, Time
    * [x] Handle put_data GET requests (string queries)
    * [x] Handle put_data POST requests 
        * [x] Handle curl's POST requests (multipart/form-data)
        * [x] Handle script's POST requests (application/x-www-form-urlencoded)
* [x] Provide an api that returns all the data already received - get_data GET requests
* [ ] API description
* [ ] **Bonus**: add persistence

## To do - Docker

 * [x] Create dockerfile for webserver
 * [ ] Create docker compose for application
 * [ ] Have persistence with docker volumes
