# 🦀 Rust HTTP Protocol Server

A partial HTTP/1.1 server in Rust, built as a learning project through
CodeCrafters' "Build Your Own HTTP Server" challenge. No web frameworks,
no async runtimes — raw TCP sockets, manual HTTP parsing, and
thread-per-connection handling.

## Features

- **HTTP/1.1 request parsing** — method, path, version, headers, and body
- **Multithreaded** — each connection runs in its own OS thread
- **Persistent connections** — loops on the same stream until the client
  sends `Connection: close`
- **Gzip compression** — negotiated via `Accept-Encoding` (uses `flate2`).
  Brotli, Deflate, and Zstd are recognized in the encoding header but not
  yet implemented.
- **File serving** — `GET` and `POST` under a configurable directory
- **Echo and User-Agent endpoints** — return path segments and request
  headers respectively

[... keep your existing Project Structure (corrected), Getting Started,
Endpoints, Examples, How It Works, and Dependencies sections ...]

## Status / not implemented

- Chunked transfer encoding
- HTTP/2 and TLS
- Brotli / Deflate / Zstd response compression (header parsed only)
- Path traversal protection on `/files/`
- Concurrent-write protection on POSTs

## Acknowledgments

Built through the [CodeCrafters](https://codecrafters.io) "Build Your Own
HTTP Server" Rust track. AI tools (Claude, Cursor) were used during
development for concept explanation, code suggestions, and debugging.
