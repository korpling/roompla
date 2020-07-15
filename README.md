# Korpling Room Planner (roompla)

## How to build

You need to have [Rust](https://www.rust-lang.org/tools/install) and [Parcel](https://parceljs.org/getting_started.html) installed.

The bundled web application is located inside the `webapp` folder and can be compiled with Parcel:
```bash
cd webapp/
parcel build --public-url "/app" src/index.html
cd ..
```

The build webapp will be served by in the roompla  server

Build the sever with cargo:
```bash
cargo build --release
```
You can run the server with 
```bash
./target/release/roompla run
```

### Regenerationg REST client code

We use OpenAPI to describe the REST API. 
The TypeScript code can be regenerated with the [OpenAPI generator](https://openapi-generator.tech/docs/installation).

```bash
java -jar openapi-generator-cli.jar generate -i src/openapi.yml -o webapp/src -g typescript-fetch -c webapp/openapi-codegen-config.json

```