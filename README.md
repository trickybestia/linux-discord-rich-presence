# linux-discord-rich-presence

Customizable Discord Rich Presence client for Linux

## Showcase

![Showcase 1](./doc/images/showcase_1.png)

![Showcase 2](./doc/images/showcase_2.png)

## Features

* Set Discord Rich Presence Activity's state, details, large image, large image hover text, small image, small image hover text, start and end timestamps.
* Use any count of Rich Presence statuses.
* Config file in any format.
* Dynamic config file reloading.

## Installation

### Building from source

#### With AUR helper (for ArchLinux-based distros)

```sh
yay -Syu linux-discord-rich-presence
```

#### Manually

1. [Install Rust](https://rustup.rs/).
2. Run the following shell script:

```sh
git clone https://github.com/trickybestia/linux-discord-rich-presence.git
cd linux-discord-rich-presence
cargo build --release
cp ./target/release/linux-discord-rich-presence ../linux-discord-rich-presence-bin
cd ..
rm -rf ./linux-discord-rich-presence
strip linux-discord-rich-presence-bin
sudo mv -f ./linux-discord-rich-presence-bin /bin/linux-discord-rich-presence
```

### Downloading binaries

1. Go to the [Releases page](https://github.com/trickybestia/linux-discord-rich-presence/releases), find the latest release and download `linux-discord-rich-presence` asset from it.
2. Move it to the `/bin/` and make it executable.

## Configuration

Create `~/.config/linux-discord-rich-presencerc` from the [template](./doc/configs/all-in-one.py) and make it executable. IT IS JUST TEMPLATE. To make it working see [configuration guide](./doc/configuration.md).

## How to use

Run the following command:

```sh
linux-discord-rich-presence -c ~/.config/linux-discord-rich-presencerc
```

You also can add this command to autostart in your DE settings.
