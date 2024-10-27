# clogger

A comfy CLI Container logger.

Scans in intervals for a given search-term matching a container-name or an image used for the container.
If a matching container is found, clogger attaches to the logs and prints it to the screen with given keywords be highlighted until the container stops.

If the `--time-tile` parameter is given, clogger will print a tiling message when the container does not log anything for around 2 seconds to make the log-reading a bit more comfy.

## What is it for?

To catch up logs from containers with random names (or random name suffixes). Also very useful if you have containers that instantly die on start and you need the logs.

## How to use

try `clogger --help` to see the the help.

For exmaple:
```bash
clogger --keywords warning,error nginx
```

will search for a container named `*nginx*` or a container that was created from `*nginx*`. If found, the log output will be started.

If a container stops, clogger will start scanning again.

## How to build

```bash
# rust must be already installed
cargo build --release
```

the binary should be located in `./target/release` then. Copy it to you favourite location (eG /usr/local/bin).

## License

[MIT](./LICENSE)