### features
1. Simple server returns text HTTP reponse
2. Serve files based on the URL path
3. Parse HTTP request headers
4. Generate and return HTTP response headers with requested file contents
5. Currently the server returns 200 Ok if it finds the file, and 404 Not Found if it doesn't
6. An example of forwarding is implemented. Proxying would be pretty similar.
7. An example of how a webapp could be integrated is included in commented out code.




### run demo:
run: `cargo run`

try loading "127.0.0.1:8000/" -> 404 file not found
try loading "127.0.0.1:8000/example.html" -> 200 Ok
try loading "127.0.0.1:8000/forward/" -> forwards to google.com



### Things that could be added
1. More closly test and adhere to the HTTP spec.
1. Add gzip to compress body - using library - https://github.com/sile/libflate
2. Add encryption - using library - https://github.com/ctz/rustls/blob/master/examples/tlsserver.rs
    - this would require certs and I have too much repressed pain from that from my day job to want to implement it here...
3. Refactor and more satisfactorly factor out a request and response structure