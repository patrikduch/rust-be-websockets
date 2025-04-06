# Overview

`rust-be-websockets` is a boilerplate template for a websocket backend service written in Rust. It provides a solid foundation with essential dependencies and configurations to streamline websocket backend development.

## Features


- **Actix-Web** for building RESTful APIs
- **GitHub Actions CI** with offline SQLx query validation and test runner


## Getting Started

### 1. Clone the Repository

```sh
git clone https://github.com/patrikduch/rust-be-template.git
cd rust-be-template
```

### 2. Set Up Environment Variables

Copy the example `.env.example` file and configure it:

```sh
cp .env.example .env
```

Modify the `.env` file with your database credentials and other configurations.


### 3. Build and Run the Application

```sh
cargo build
cargo run
```


### 4. Dockerization

```sh
docker-compose up -d
```
