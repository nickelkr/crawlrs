# Crawlrs

This is a simple crawler using Rust. It fetches pages and extracts any http(s) links found in `a href` elements.

### Requirements
Rust and Cargo

### Running
To run the crawler provide the URL to crawl
`cargo run -- --url=https://example.com`

It also accepts a optional timeout (default 1000 ms) to wait for any new pages
`cargo run -- --url=https://example.com --timeout=800`

### Tests
To execute the tests
`cargo test`

If you want to see STDOUT during the test run
`cargo test -- --nocapture`
