My plan is to implement a simple threaded web server in rust. This is one of my first rust projects and the purpose is to a) learn rust and b) demostrate to myself (and others) my technical knowledge and ability. That said, this may be rough becuase I am learning the rust language as I go.

Notably there is a lot of clutter because I do not understand the ideal solution to deal with handling of `Result` and also the ideal understanding to handle ownership. For that reason, when in doubt I'm doing a lot of random `unwrap`, `expect` and `clone` to deal with that. Shoot first ask questions later.

Also I'm unsure at this moment whether I want to be using `String` or `str` and so that is also adding to code confusion and reduced code quality.

At this point, I'm not planning on going back over it and making high code quality, rather, I'm using this as a learning through mistakes project (since it's my first project).

### features
1. simple server returns text HTTP reponse
2. serve files based on the URL path
3. parse HTTP request headers
4. generate and return HTTP response headers with requested file contents
5. currently the server returns 200 Ok if it finds the file, and 404 Not Found if it doesn't
6. an example of forwarding is implemented. Proxying would be pretty similar.

### Things that could be added
1. add gzip to compress body - using library - https://github.com/sile/libflate
2. add encryption - using library - https://github.com/ctz/rustls/blob/master/examples/tlsserver.rs
    - this would require certs and I have too much repressed pain from that from my day job to want to implement it here...
3. refactor and more satisfactorly factor out a request and response structure

4. outline how a webapp could interface - but no implementation
    - DONE!


### demo path:

try loading "127.0.0.1:8000/" -> 404 file not found
try loading "127.0.0.1:8000/example.html" -> 200 Ok
try loading "127.0.0.1:8000/forward/" -> forwards to google.com

### for pseudocode for how a webapp could be implmented, see the commented out function "example_web_application_interface"
- Note this is pseudo code