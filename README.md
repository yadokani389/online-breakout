# online-breakout

This is an online game using peer-to-peer connections and iroh without a server.

This project incorporates features from iroh to enable peer-to-peer connections, inspired by the implementation in [extreme_bevy](https://github.com/johanhelsing/extreme_bevy) and [matchbox](https://github.com/johanhelsing/matchbox).

## native

To run the native version, execute the following commands:

```sh
nix develop
cargo run
```

After running, retrieve the `iroh id` from the log.

```sh
cargo run -- -i xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

If you are using Nix, you can also run the application without clone:


```sh
nix run github:yadokani389/online-breakout
```
This can be used instead of `cargo run`.

## web

To run the web version, execute the following commands:

```sh
nix develop
cargo run --target wasm32-unknown-unknown
```

Once running, access the application via the provided URL (e.g., <http://127.0.0.1:1334/>).

To connect to the application, append the `#` and the `iroh id` to the URL.
(e.g., <http://127.0.0.1:1334/#xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx>)
