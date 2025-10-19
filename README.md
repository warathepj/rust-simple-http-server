# ซอร์สโค้ดนี้ ใช้สำหรับเป็นตัวอย่างเท่านั้น ถ้านำไปใช้งานจริง ผู้ใช้ต้องจัดการเรื่องความปลอดภัย และ ประสิทธิภาพด้วยตัวเอง

# Simple HTTP Server in Rust

This project implements a basic HTTP server in Rust. It's designed to be lightweight and demonstrate core concepts of handling HTTP requests and serving static files.

## Features

- Handles basic GET requests.
- Serves static files from a specified directory.
- Simple, single-threaded architecture (for now).

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

You will need to have Rust and Cargo installed. If you don't have them, you can install them from the official Rust website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### Building the Project

Navigate to the `simple_http_server` directory and build the project using Cargo:

```
cd <repo_name>
cargo build
```

### Running the Server

After building, you can run the server. By default, it will listen on `127.0.0.1:8080`.

```
cargo run
```

You can also specify the port and content directory (this feature might be added later, currently it's hardcoded or uses defaults).
