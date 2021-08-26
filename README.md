# linux-discord-rich-presence

**Customizable Discord Rich Presence client for Linux**

---

## Showcase

<div style="display: flex; flex-wrap: nowrap; gap: 1rem;">
    <img src="./assets/1.png" style="width: 50%; height: 50%" />
    <img src="./assets/2.png" style="width: 50%; height: 50%" />
</div>

## Features

* Set Discord Rich Presence Activity's state, details, large image, large image hover text, small image, small image hover text, start and end timestamps.
* Config file in .sh format.
* Dynamic config file reloading.

## How to download

1. Go to the [Releases page](https://github.com/trickybestia/linux-discord-rich-presence/releases), find the latest release and download `linux-discord-rich-presence` asset from it.
2. Move it to the `/bin/` and make it executable.

## How to build

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

## How to install 

Create `~/.config/linux-discord-rich-presencerc` from the following template:
```sh
#!/bin/sh

application_id () {
    echo your_application_id_here
}

update_delay () {
    echo 10
}

state () {
    uname -r
    # or
    # echo
}

details () {
    uname -n
    # or
    # echo
}

large_image_key () {
    echo your_large_image_key_here
    # or
    # echo
}

large_image_text () {
    echo your_large_image_text_here
    # or
    # echo
}

small_image_key () {
    echo your_small_image_key_here
    # or
    # echo
}

small_image_text () {
    echo your_small_image_text_here
    # or
    # echo
}

start_timestamp () {
    date -d "$(uptime -s)" +%s
    # or
    # echo
}

end_timestamp () {
    echo
}

buttons () {
    # \u0091 is a delimiter
    echo -e 'button1\u0091https://button1\u0091button2\u0091https://button2'
    # or
    # echo -e 'button1\u0091https://button1'
    # or
    # echo
}

echo
```

## How to use

Run the following command:
```sh
linux-discord-rich-presence -c ~/.config/linux-discord-rich-presencerc
```
