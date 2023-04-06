# Building

The easiest method to get going is to install [`cargo-make`](https://github.com/sagiegurari/cargo-make).

After `cargo-make is installed`...

- Setup your environment with

`cargo make pre-build` to create a `.env` file from the [template](.env.example).

- After that's setup, run:

`cargo make migration-up` to generate your migrations

- Finally, run `cargo make dev` to spin up the server in watch mode
