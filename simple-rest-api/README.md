# Simple REST API

This project implements a simple REST API using `actix-web`.

## Features
- **GET /**: Returns a JSON greeting.
- **POST /echo**: Returns whatever you send in the body.
- **GET /games/{season}/{code}**: Fetches a Euroleague game by season and code, parses the XML from the Euroleague API, and returns it as a JSON object.

## Configuration
The application is configured via `Settings.toml`. You can set the server host/port and the Euroleague API base URL.

```toml
[server]
host = "127.0.0.1"
port = 8080

[euroleague]
base_url = "https://api-live.euroleague.net/"
```

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

### Test Euroleague Game Endpoint
```bash
curl http://127.0.0.1:8080/games/E2023/1
```
Output:
```json
{
  "@code": 1,
  "@seasoncode": "E2023",
  "@cetdate": "2023-10-05T19:00:00",
  "localclub": {
    "@code": "RED",
    "@name": "Crvena Zvezda Meridianbet Belgrade",
    "@score": 94
  },
  "roadclub": {
    "@code": "ASV",
    "@name": "LDLC ASVEL Villeurbanne",
    "@score": 73
  }
}
```
