# Configuration

## How it works

At first, let's understand how linux-discord-rich-presence configuration system works.

When you start linux-discord-rich-presence the following steps are executed:

1. linux-discord-rich-presence determines if specified config file is executable or not.

### If config is executable

1. The config is executed as if you have executed it from the shell via `~/.config/linux-discord-rich-presencerc` as if it was any other application. Let's call started process as "Config Process".
2. linux-discord-rich-presence connects to stdout (standard output) of the Config Process.
3. linux-discord-rich-presence listens Config Process' stdout and waits for updates. Update message is serialized as JSON, schema can be found [here](./update.schema.json).
4. linux-discord-rich-presence parses update message and updates your Discord Rich Presence status according to it.
5. Execution goes back to step 3.

### If config is not executable

it is treated as single update message serialized in JSON.

Atfer reading this the following conclusions can be made:

* Your configuration file is valid as far as it sends at least one update message. It can be a Python, Bash, Perl (name all of them) script or even a binary.
* Your Config Process can enable or disable different Rich Presence applications in your status during the time.

## Creating Discord Application

One of the important steps to get linux-discord-rich-presence working is creating Discord Application and uploading all required assets to it.

To do it you need to follow these steps:

1. Go to [Discord Developer Portal](https://discord.com/developers/applications).
2. Create a new Application (name can be changed at any time).
3. Select it in the list of your applications and go to `Rich Presence` -> `Art Assets`. Upload here images that you will use in your status.
4. Go to `Rich Presence` -> `Visualizer`. Here you can preview your configuration.

![Rich Presence Visualizer](./images/rich_presence_visualizer.png)

## Writing config

To write your own config you can check one of the following templates:

* [Python template](./configs/all-in-one.py) (with comments)
* [Shell template](./configs/all-in-one.sh)
* [Static template](./configs/static.json)
