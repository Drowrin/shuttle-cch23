<div align="center">

# Christmas Code Hunt 2023
My solutions to the [Shuttle.rs](https://www.shuttle.rs) [Christmas Code Hunt 2023](https://www.shuttle.rs/cch)

</div>

## Usage
This repo uses [just](https://github.com/casey/just) to run common commands.
You'll need to install the following in order to use the `just` commands:
- [cargo-shuttle](https://github.com/shuttle-hq/shuttle) - the shuttle.rs CLI
- [cargo-watch](https://github.com/watchexec/cargo-watch) - runs commands when files change
- [cch23-validator](https://crates.io/crates/cch23-validator) - official challenge validator

With everything installed, you can run the following commands:
```sh
# Compile, run, and test endpoints
just test

# Watch for file changes and run `just test` as needed
just watch
```

### Project Layout
Each challenge day has a separate file in [./src/](./src/).
Each of these files exposes a `get_routes() -> axum::Router` function, including all of the endpoints for that day.
These routes all include their day's path number, as required by Shuttle.rs CCH.
[./src/main.rs](./src/main.rs) uses all of these `get_routes` functions to build the complete `Router`, and has the `#[shuttle_runtime::main]` attribute applied.
