# 🦀 Rust HTTP Protocol Server

A partial HTTP/1.1 server in Rust, built as a learning project through
CodeCrafters' "Build Your Own HTTP Server" challenge. No web frameworks,
no async runtimes — raw TCP sockets, manual HTTP parsing, and
thread-per-connection handling.

---

## Features

- **HTTP/1.1 request parsing** — method, path, version, headers, and body
- **Multithreaded** — each connection runs in its own OS thread via
  `std::thread::spawn`
- **Persistent connections** — loops on the same stream until the client
  sends `Connection: close`
- **Gzip compression** — negotiated via `Accept-Encoding` (uses `flate2`).
  Brotli, Deflate, and Zstd are recognized in the encoding header but not
  yet implemented.
- **File serving** — `GET` and `POST` support under a configurable directory
- **Echo and User-Agent endpoints** — return path segments and request
  headers respectively

---

## Project Structure

```
src/
├── main.rs               # Entry point — binds TCP listener, spawns threads
├── server/
│   └── connection.rs     # Connection loop — reads, parses, and routes requests
└── http/
    ├── request.rs        # HTTP request parser (headers, body, Content-Length)
    ├── response.rs       # Response builders (text, file, encoded)
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

Incoming bytes are buffered until a complete HTTP request is detected.
Completeness is determined by finding the `\r\n\r\n` header terminator and
verifying that the buffer contains the full body as specified by the
`Content-Length` header. This allows the server to correctly handle
pipelined and partial TCP reads without blocking.

### Connection Lifecycle

The server supports persistent connections. After each response, it checks
whether the request included a `Connection: close` header. If so, it
appends a matching `Connection: close` header to the response and closes
the stream. Otherwise, it loops and waits for the next request on the
same connection.

### Compression

When a client sends an `Accept-Encoding: gzip` header, the `/echo/`
endpoint compresses the response body using `flate2` and sets the
appropriate `Content-Encoding: gzip` header with the compressed
`Content-Length`.

---

## Dependencies

```toml
[dependencies]
flate2 = "1"
```

Otherwise uses only the Rust standard library.

---

## Status / Not Implemented

- Chunked transfer encoding
- HTTP/2 and TLS
- Brotli / Deflate / Zstd response compression (header parsed only)
- Path traversal protection on `/files/`
- Concurrent-write protection on POSTs

---

## Acknowledgments

Built through the [CodeCrafters](https://codecrafters.io) "Build Your Own
HTTP Server" Rust track. AI tools (Claude, Cursor) were used during
development for concept explanation, code suggestions, and debugging.
