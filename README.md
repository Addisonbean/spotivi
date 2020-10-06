# spotivi

Spotivi is a command line Spotify client, inspired by vim.

It's currently in progress and missing quite a few features and not ready for use.

# Usage

Compiling Spotivi requires [Rust](https://www.rust-lang.org/).

Once Rust is installed, Spotivi can be built and run using `cargo run`. 

You'll need to create a Spotify app on the [Spotify Developer site](https://developer.spotify.com) in order for Spotivi to work.

Then create a file named `~/.config/spotivi/config` with the following contents:

```
api_client_id = <client_id>
api_client_secret = <client_secret>
```

Replace `<client_id>` and `<client_secret>` with the values you got from the Spotify developer app you created.
