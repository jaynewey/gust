<div align="center">

<img width="64" src="assets/icon-256.png">

# Gust

</div>

> 💨 Wasm powered weather app

View the [Live deployment](https://jaynewey.github.io/gust).

## Features

- Hourly temperature and weather conditions
- Hourly precipitation probability
- Hourly wind speeds and direction

## Contributing

Contributions are appreciated. Please see the following to get started:

Download the [Tailwind standalone CLI](https://tailwindcss.com/blog/standalone-cli) and add it to your PATH, e.g:

```sh
curl -sL https://github.com/tailwindlabs/tailwindcss/releases/download/v3.3.3/tailwindcss-linux-x64 -o tailwindcss
chmod +x tailwindcss
mv tailwindcss /usr/local/bin/
```

Build and serve the app: (This will require `trunk`: `cargo install trunk`)

```sh
trunk serve
```

Serve the development API with: (This will require `rust-script`: `cargo install rust-script`)

```sh
rust-script scripts/server.rs
```

The app should now be visible at `127.0.0.1:8080/gust/`.

To test the app on the live API, serve the app with:

```sh
FORECAST_ENDPOINT=https://api.open-meteo.com/v1 GEOCODING_ENDPOINT=https://geocoding-api.open-meteo.com/v1 trunk serve
```

### Formatting and Linting

`rustfmt`, [`leptosfmt`](https://github.com/bram209/leptosfmt) and [`rustywind`](https://github.com/avencera/rustywind) are used to format source files:

```sh
cargo fmt
leptosfmt src
rustywind src
```

`clippy` lints are also enforced:

```sh
cargo clippy
```

---

[Weather data by Open-Meteo.com](https://open-meteo.com/)
