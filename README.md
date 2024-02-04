# linux-discord-rich-presence

Customizable Discord Rich Presence client for Linux

## Showcase

![Showcase 1](./doc/images/showcase_1.png)

![Showcase 2](./doc/images/showcase_2.png)

## Features

* Set Discord Rich Presence Activity's state, details, large image, large image hover text, small image, small image hover text, current and max party size, start and end timestamps.
* Use any count of Rich Presence statuses.
* Config file in any format.
* Dynamic config file reloading.

## Installation

### With AUR helper (for ArchLinux-based distros)

```sh
yay -Syu linux-discord-rich-presence
```

### Manually compile from source

1. [Install Rust](https://rustup.rs/).
2. Run the following shell script:

```sh
git clone https://github.com/trickybestia/linux-discord-rich-presence.git
cd linux-discord-rich-presence
cargo build --release
sudo install -Dm0755 -t /usr/bin/ ./target/release/linux-discord-rich-presence
sudo install -Dm0755 -t /usr/bin/ ./doc/linux-discord-rich-presence-desktop-wrapper
sudo install -Dm0644 -t /usr/share/applications/ ./doc/linux-discord-rich-presence.desktop
sudo install -Dm0644 -t /usr/share/applications/ ./doc/linux-discord-rich-presence-minimized.desktop
```

## Configuration

Create `~/.config/linux-discord-rich-presencerc` from [one of the templates](./doc/configs/) and make it executable. THEY ARE JUST TEMPLATES. To make them work see [configuration guide](./doc/configuration.md).

## How to use

Choose `Discord (linux-discord-rich-presence)` entry in your application launcher or run the following command:

```sh
linux-discord-rich-presence -c ~/.config/linux-discord-rich-presencerc
```

You also can add this command or `Discord (linux-discord-rich-presence) (minimized)` to autostart in your DE settings.

## License

Licensed under [GNU GPLv3](COPYING) only.
