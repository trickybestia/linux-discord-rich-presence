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

### With AUR helper (for ArchLinux-based distros)

```sh
yay -Syu linux-discord-rich-presence
```

### Manually

1. [Install Rust](https://rustup.rs/).
2. Run the following shell script:

```sh
git clone https://github.com/trickybestia/linux-discord-rich-presence.git
cd linux-discord-rich-presence
cargo build --release
sudo mv -f ./target/release/linux-discord-rich-presence /bin/linux-discord-rich-presence
cd ..
rm -rf ./linux-discord-rich-presence
```

## Configuration

Create `~/.config/linux-discord-rich-presencerc` from the [template](./doc/configs/all-in-one.py) and make it executable. IT IS JUST TEMPLATE. To make it working see [configuration guide](./doc/configuration.md).

## How to use

Run the following command:

```sh
linux-discord-rich-presence -c ~/.config/linux-discord-rich-presencerc
```

You also can add this command to autostart in your DE settings.
