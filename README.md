# Korpling Room Planner (roompla)

## How to build

You need to have [Rust](https://www.rust-lang.org/tools/install) and [Parcel](https://parceljs.org/getting_started.html) installed.

The bundled web application is located inside the `webapp` folder and can be compiled with Parcel:
```bash
cd webapp/
parcel build src/index.html
cd ..
```

TODO: The build webapp will be served by in the roompla  server

Build the sever with cargo:
```bash
cargo build --release
```
You can run the server with 
```bash
./target/release/roompla
```

