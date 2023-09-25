# Loginator

This project acts as a bridge between a VEX robot and an mqtt broker. The output from this broker can then be sent to listened to by a Grafana instance. 

You can find more information about those services here:
- https://grafana.com/docs/
- https://mosquitto.org/documentation/

## Installation

You will need both Grafana and Mosquitto instance running.

Instructions to install Mosquitto can be found [here](https://mosquitto.org/download/).

Instructions to install Grafana can be found [here](https://grafana.com/grafana/download).

Currently cargo install is the only supported method for installation. This may change once there is a stable release.

```sh
cargo install --git https://github.com/BattleCh1cken/loginator
```
Make sure that `$HOME/.cargo` is added to `$PATH`.

You can then run it like so:

```sh
loginator
```

## Roadmap
This project is very much still in development. The following features are planned:
- A CLI with the [CLAP framework](https://github.com/clap-rs/clap)
- File based configuration
- Integration with [LemLib](https://github.com/LemLib/LemLib)
