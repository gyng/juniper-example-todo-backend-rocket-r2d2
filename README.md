# Rust, GraphQL, SQLite, Rocket, r2d2 example

[![Build Status](https://travis-ci.org/gyng/juniper-example-todo-backend-rocket-r2d2.svg?branch=master)](https://travis-ci.org/gyng/juniper-example-todo-backend-rocket-r2d2)

Based off https://github.com/mhallin/juniper-example-todo-backend, but uses Rocket and adds a r2d2 connection pool.

Requires nightly and diesel-cli.

## Usage

```
diesel migration run
cargo +nightly run
open http://localhost:8000
```
