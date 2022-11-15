# Webserver

This is a webserver built in rust using the [tokio](https://docs.rs/tokio/latest/tokio/) and the [warp](https://docs.rs/warp/latest/warp/) crates.
Tokio is an asynchronous runtime required to fullfill our multi-threaded necessities.
Warp is a simple webserver framework that allows us to build the API.
These are both pretty fast tools and so should the webserver, provided my code does not server as a bottleneck.

---

## To do

* [x] Handle multiple requests simultaneously
* [-] Receive http requests with the following variables Latitude, Longitude, Speed, Time
    * [x] Handle put_data GET requests (string queries)
    * [ ] Handle put_data POST requests (multipart data)
* [ ] Provide an api that returns all the data already received
* [ ] API description
* [ ] **Bonus**: add persistence
