# diary.computer

A mood tracking / short journaling web app built with Rust and Svelte

## Getting Started

### setup.sh

You can run `setup.sh` which will install (most of) the required dependencies, setup the database, and create required `.env` files (does not create `.env.docker` for running with docker)

You can also opt for [manual setup](#manual-setup) if you know what you are doing

> ⚠️ **WARNING:** This will overwrite any existing `.env` files

```zsh
$ sh ./setup.sh
```

### Building images

To build the project docker image use the `build.sh` script

```zsh
$ sh ./build.sh
```

#### Running containers

To run the project using the `.env.docker` file use the `docker-run.sh` script

```zsh
$ sh ./docker-run.sh
```

### Diesel/Postgres

This project uses [Diesel](https://diesel.rs/) and Postgres

Running this project requires installation and setup of both `diesel_cli` and `postgresql`

### Manual Setup

If you're using Windows you're on your own

#### Linux & WSL

```bash
$ sudo apt-get install postgresql postgresql-contrib libpq-dev
$ sudo -u postgres createuser <username>
$ sudo -u postgres createdb <database>
```

#### MacOS

On MacOS you might need to install libpq and add it to PATH first

```bash
$ brew install libpq && brew link --force libpq
$ echo 'export PATH="/usr/local/opt/libpq/bin:$PATH"' >> ~/.zshrc
```

and then install diesel cli

```bash
$ cargo install diesel_cli --no-default-features --features postgres
```

#### Creating a user and database

```sql
$ sudo -u postgres psql
psql=# ALTER USER <username> WITH PASSWORD <password>;
psql=# GRANT ALL PRIVILEGES ON DATABASE <database> TO <username>;
```

#### Diesel

Installing `diesel_cli` and running setup, requires `.env`, or you can create a new `.env` file

```bash
$ echo DATABASE_URL=postgres://<username>:<password>@<host>/<database> > .env
```

```bash
$ cargo install diesel_cli --no-default-features --features postgres
$ diesel setup
```

#### Migrations

You can manually run migrations with

```bash
$ diesel migration run
```

But migrations are also run automatically on server startup

##### Redoing migrations

Should you need to redo all migrations

```bash
$ diesel migration redo --all
```

## Development

### Backend

#### Linting and building

In order to keep code style consistent, run `cargo fmt` and `cargo clippy` to verify code formatting and linting

```bash
$ cargo fmt
$ cargo clippy
```

Building the backend

```bash
$ cargo build
```

Or for production/release pass the `--release` flag

```bash
$ cargo build --release
```

#### Running the app

If the frontend is build, the backend will serve the static files from `www`

```bash
$ cargo run
```

#### Tests

Running tests

```bash
$ cargo test
```

GitHub actions will run `cargo test ci --verbose` on commit to `main` or when creaing a pull request

In order to have a backend test run using GitHub actions, include `ci` in the test name

> **Note:** We currently have an open issue [#5 run all tests on github actions](https://github.com/alexampersandria/diary.computer/issues/5) to run all tests on GitHub actions

As an example `util::unix_time::ci_unit::positive` could be defined as:

```rust
#[cfg(test)]
mod ci_unit {
  use super::*;

  #[test]
  fn positive() {
    let unix_time = unix_time();

    assert!(unix_time > 0, "unix_time should be positive");
  }
}
```

### Frontend

This project uses [Bun](https://bun.sh/)

```bash
$ bun install
```

Building the frontend, the frontend builds to `www` and will be served by the backend server at `/`

```bash
$ bun run build
```

To run the dev server

```bash
$ bun run dev
```

And to run tests (you guessed it, it's `bun run [blank]`)

```bash
$ bun run test
```

## Commits and Versioning

This project uses [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) to manage commits

Use the following format when committing:

```bash
$ git commit -m "type: message"
```

### Commit Types

| Type       | Description                                                   |
| ---------- | ------------------------------------------------------------- |
| `feat`     | A new feature                                                 |
| `fix`      | A bug fix                                                     |
| `docs`     | Documentation only changes                                    |
| `style`    | Changes that do not affect the meaning of the code            |
| `refactor` | A code change that neither fixes a bug nor adds a feature     |
| `perf`     | A code change that improves performance                       |
| `test`     | Adding missing tests or correcting existing tests             |
| `build`    | Changes that affect the build system or external dependencies |
| `ci`       | Changes to our CI configuration files and scripts             |
| `chore`    | Other changes that don't modify src or test files             |
| `revert`   | Reverts a previous commit                                     |

### Versioning

Versioning is done manually for now, but should follow [Semantic Versioning](https://semver.org/) (for 1.0.0 or above, for 0.x.y it's not as important), we have an open issue [#10 generate version + release from commits](https://github.com/alexampersandria/diary.computer/issues/10) to automate this in the future
