A server that offers Pokemon descriptions written
using Shakespeare's style.

## How to run

### From sources

Install [rustup](https://www.rust-lang.org/tools/install)
if not installed

    https://www.rust-lang.org/tools/install

Compile and run the project with

    cargo run --release

### With Docker

Clean the target directory (if needed) and build the Docker image

    cargo clean && docker build -t shakemon:latest .

Run with

    docker run shakemon:latest

## Customization

### Compile-time customization

Three variables are available for compile-time customization:
- `LOG_ENV_VAR` → The name of the environment variable which controls the log level
- `CONFIGURATION_FILE` → The path to the configuration file which is parsed on startup
- `HOST` → The host that the server will listen to

They can be found in [main.rs](src/main.rs).

### Environment variable customization

The `RUST_LOG` variable is the environment variable that by default controls
the log level. Please see
[the documentation of env_logger](https://docs.rs/env_logger/0.7.1/env_logger/index.html)
for a description of the values that it accepts.

By default the log level is `info`.

### Configuration file

The application by default expects a `configuration.yml` file in the same
folder from where it is launched. Please see the bundled
[configuration file](configuration.yml) for
a description of the available configuration properties.

## Project description

The server is built on the `actix_web` crate and listens to `localhost:8000`
by default.

There are two endpoints, the `/health_check` endpoint that can be
used to check that the server is healthy and the `/pokemon/{pokemon_name}`
endpoint that can be used to get the description of `pokemon_name` written
in Shakespeare's style. The latter returns a JSON payload with
name and description, for example:

```json
{
    "description": "Ditto rearranges its cell structure to transform itself into other shapes. However,  if 't be true 't tries to transform itself into something by relying on its memory,  this pokémon manages to receiveth details wrong.",
    "name": "ditto"
}
```

This endpoint returns 404 if a Pokemon description is not found
and 429 if too many requests are issued in a short amount of time.

Since the [funtranslations API](https://funtranslations.com/api/)
that the we use to translate
descriptions has a rate limit of 5 requests per hour, this may
happen often.

### Cache

We use a basic
[LRU cache](https://en.wikipedia.org/wiki/Cache_replacement_policies#Least_recently_used_(LRU))
to speed description retrieval up and to work around rate limits.

### How a description is chosen

The [PokeAPI API](https://pokeapi.co/) that we use to find
Pokemon descriptions returns many different descriptions for the same
Pokemon in general. As a simple heuristic, we choose the longest one
in an attempt to select the most interesting.

### Error management

We have defined our custom error which is mapped automatically to
`actix`'s `HttpResponse` error. Error logging is centralized inside
this mapping method, in order to reduce the noise in the rest of
the application.

### Tests

We have a few black-box tests that verify the server behavior
using mocked HTTP servers to simulate *PokeAPI* and *funtranslations*.

Tests can be run with `cargo test` or, if using Docker,
`docker run shakemon:latest cargo test --release` (you need to build
the image first).

## What could be improved

There are many opportunities for improvement, so this is a
non-exhaustive and randomly-ordered list:

- The cache should at least provide an expiration time for
  the entries (which is currently missing)
- Tests and documentation could be expanded
- A replacement for *funtranslations* could be considered,
  if available, in order to work around the rate limit
- Telemetry relies on simple logs and could benefit from a more
  structured solution
- The Docker image is very large and there are various ways
  to optimize it
