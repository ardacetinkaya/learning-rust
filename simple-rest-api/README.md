# Simple REST API

This project implements a simple REST API using `actix-web`.

## Features
- **GET /**: Returns a JSON greeting.
- **POST /echo**: Returns whatever you send in the body.

## How to Run

1. Navigate to the project directory:
   ```bash
   cd simple-rest-api
   ```
2. Start the server:
   ```bash
   cargo run
   ```

## Verification
You can verify the endpoints using `curl` in another terminal:

### Test GET Endpoint
```bash
curl http://127.0.0.1:8080/
```
Output:
```json
{"message":"Hello, Rust!"}
```

### Test POST Endpoint
```bash
curl -X POST -d "Hello" http://127.0.0.1:8080/echo
```
Output:
```text
Hello
```
