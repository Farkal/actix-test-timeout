# Server
`RUST_LOG=actix=trace,actix_web=trace,actix_test=trace cargo run`

# Benchmark
```
npm install autocannon
node canon.js
```

# Observation
Lot of 408 errors when a set of request end (for exemple the first 8 request finish, we will get 200 for the 8 and some 408 for others)

# Fix
Using web::block work because we are not blocking the actix thread and we move heavy computing in another thread.  
I let this repo if someone want to check!