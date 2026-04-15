
# 🦀 Rust HTTP Protocol Server

A lightweight HTTP/1.1 server built from scratch in Rust — no web frameworks, no async runtimes. Just raw TCP sockets, manual HTTP parsing, and multithreaded connection handling.

---

## Features

- **HTTP/1.1 request parsing** — method, path, version, headers, and body
- **Multithreaded** — each connection is handled in its own OS thread via `std::thread::spawn`
- **Persistent connections** — supports keep-alive with graceful `Connection: close` handling
- **Gzip compression** — negotiates encoding via `Accept-Encoding` and compresses responses with `flate2`
- **File serving** — `GET` and `POST` support under a configurable directory
- **Echo endpoint** — reflects path segments back as a plain-text response
- **User-Agent endpoint** — reads and returns the `User-Agent` request header

---

## Project Structure

```
src/
├── main.rs               # Entry point — binds TCP listener, spawns threads
├── server/
│   └── connection.rs     # Connection loop — reads, parses, and routes requests
└── http/
    ├── request.rs        # HTTP request parser (headers, body, Content-Length)
    ├── response.rs       # Response builders (text, file, encoded, etc.)
    └── encoding.rs       # Content-Encoding negotiation and Gzip compression
```

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)

### Build & Run

```bash
# Clone the repo
git clone https://github.com/BeeAssis/rust-http-protocol-server.git
cd rust-http-protocol-server

# Run the server (binds to 127.0.0.1:4221)
cargo run

# Run with a file-serving directory
cargo run -- --directory /tmp/files
```

The server listens on **port 4221** by default.

---

## Endpoints

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/` | Returns `200 OK` |
| `GET` | `/echo/{text}` | Returns `{text}` as plain text (Gzip if accepted) |
| `GET` | `/user-agent` | Returns the value of the `User-Agent` header |
| `GET` | `/files/{filename}` | Serves a file from the configured directory |
| `POST` | `/files/{filename}` | Writes the request body to a file in the configured directory |

### Examples

```bash
# Basic health check
curl http://localhost:4221/

# Echo endpoint
curl http://localhost:4221/echo/hello

# Echo with Gzip compression
curl --compressed -H "Accept-Encoding: gzip" http://localhost:4221/echo/hello

# User-Agent
curl http://localhost:4221/user-agent

# Upload a file
curl -X POST --data "hello world" http://localhost:4221/files/hello.txt

# Download a file
curl http://localhost:4221/files/hello.txt
```

---

## How It Works

### Request Parsing

Incoming bytes are buffered until a complete HTTP request is detected. Completeness is determined by finding the `\r\n\r\n` header terminator and verifying that the buffer contains the full body as specified by the `Content-Length` header. This allows the server to correctly handle pipelined and chunked reads without blocking.

### Connection Lifecycle

The server supports persistent connections. After each response, it checks whether the request included a `Connection: close` header. If so, it appends a matching `Connection: close` header to the response and closes the stream. Otherwise, it loops and waits for the next request on the same connection.

### Compression

When a client sends an `Accept-Encoding: gzip` header, the `/echo/` endpoint compresses the response body using `flate2` and sets the appropriate `Content-Encoding: gzip` header with the compressed `Content-Length`.

---

## Dependencies

```toml
[dependencies]
flate2 = "1"
```

---

## Motivation

Started as a CodeCrafters challenge and extended with production-minded features: a buffered request pipeline that handles partial TCP reads, content-encoding negotiation, and an architecture designed to support additional compression schemes.
