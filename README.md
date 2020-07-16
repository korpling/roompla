# Korpling Room Planner (roompla)

## How to build and run

You need to have [Rust](https://www.rust-lang.org/tools/install).

Build the server with cargo:
```bash
cargo build --release

```

### Run server

You can run the server with 
```bash
./target/release/roompla run
```

To change the configuration file roompla uses (per default `roompla.toml` in the working directory), use the `ROOMPLA_CONFIG` environment variable.

```bash
ROOMPLA_CONFIG=roompla.example.toml ./target/release/roompla run
```

An example file (`roompla.example.toml`) with the various configuration options is included in this repository.

If you want to develop the web application and don't want to recompile the rust service on every change, you
can start a live server for the web application.

The bundled web application is located inside the `webapp` folder and can be compiled with [Parcel](https://parceljs.org/getting_started.html):
```bash
cd webapp/
parcel build --public-url "/app" src/index.html
cd ..
```


### Regenerationg REST client code

We use OpenAPI to describe the REST API. 
The TypeScript code can be regenerated with the [OpenAPI generator](https://openapi-generator.tech/docs/installation).

```bash
java -jar openapi-generator-cli.jar generate -i src/openapi.yml -o webapp/src -g typescript-fetch -c webapp/openapi-codegen-config.json

```
