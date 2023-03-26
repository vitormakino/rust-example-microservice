# Microservice with Rust
An example of microservice using actix-web, a very fast web framework for Rust.

## Quickstart

### Clone the repository
```shell
git clone https://github.com/vitormakino/rust-example-microservice.git
```

### Go into the project directory
 
```shell
cd rust-example-microservice
```

### Build and run the project

```shell
cargo run
```

### After starting the server, you may run the following pages:
- http://localhost:8080
- http://localhost:8080/hello/YOUR_NAME_HERE
 
## Running with Docker

### Build image

```shell
docker build -t rust-example-microservice .
```

### Run built image

```shell
docker run -d -p 8080:8080 rust-example-microservice
# and the server should start instantly
curl http://localhost:8080
```

