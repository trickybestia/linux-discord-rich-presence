# linux-discord-rich-presence

**Customizable Discord Rich Presence client for Linux**

---

## Showcase

![](./assets/1.png)

![](./assets/2.png)

## Features

* Set Discord Rich Presence Activity's state, details, large image, large image hover text, small image, small image hover text, start and end timestamps.
* Config file in any format.
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

Create `~/.config/linux-discord-rich-presencerc` from the following template (do not forget to make it executable!):
##### Python template
```python
#!/bin/python

import json
from os import popen
from contextlib import suppress


def cmd(program):
    with popen(program) as process:
        return process.read()[0:-1]


def update():
    return {'update_delay': 10, 'items': [{
        'application_id': 000000000000000000,
        'state': cmd('uname -r'),
        'details': cmd('uname -n'),
        'large_image': {'key': 'some_image', 'text': None},
        'small_image': None,
        'start_timestamp': cmd('date -d "$(uptime -s)" +%s'),
        'end_timestamp': None,
        'buttons': [{'label': 'some_button',
                    'url': 'https://example.com/'
                     }],
    }]}


with suppress(EOFError):
    while True:
        command = input()

        if command == 'update':
            print(json.dumps(update()))
        else:
            print()
```


## How to use

Run the following command:
```sh
linux-discord-rich-presence -c ~/.config/linux-discord-rich-presencerc
```
