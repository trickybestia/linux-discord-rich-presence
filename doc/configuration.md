# Configuration

## How it works

At first, let's understand how linux-discord-rich-presence configuration system works.

When you start linux-discord-rich-presence the following steps are executed:

1. The config is executed as if you have executed it from the shell via `~/.config/linux-discord-rich-presencerc` as if it was any other application. Let's call started process as "Config Process".
2. linux-discord-rich-presence connects to itself stdin (standard input) and stdout (standard output) of the Config Process.
3. linux-discord-rich-presence periodically sends `update` command to the Config Process' stdin and waits for response. Response is serialized as JSON, schema can be found [here](./response.schema.json).
4. linux-discord-rich-presence parses response and updates your Discord Rich Presence status according to it.
5. Execution goes back to step 3.

Atfer reading this the following conclusions can be made:

* Your configuration file is valid as far as it processes linux-discord-rich-presence's commands. It can be a Python, Bash, Perl (name all of them) script or even a binary.
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

We have understood how linux-discord-rich-presence can be configured, so let's write our own config! The example is written in Python, but if you are unfamiliar with it, you can use something else.

To write your own config you should check [the template](./configs/all-in-one.py) and edit it as you want.
