# Rust, GraphQL, SQLite, Rocket, r2d2 example

Based off https://github.com/mhallin/juniper-example-todo-backend, but uses Rocket and adds a r2d2 connection pool.

Requires nightly and diesel-cli.

## Usage

```
diesel migration run
cargo +nightly run
open http://localhost:8000
```
