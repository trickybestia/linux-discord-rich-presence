# Changelog

## v3.1.0 (2022-08-23)

* Added support for static JSON configs.
* Now linux-discord-rich-presence attempts to update user's activities every 10 seconds. It leads to faster reconnection time when connection to Discord is lost.

## v3.0.0 (2022-02-28)

* Configuration protocol was reviewed and changed.

## v2.0.1 (2021-10-31)

* Fix bug with `clap` version.

## v2.0.0 (2021-10-17)

* Use any count of Rich Presence statuses.

## v1.0.0 (2021-09-20)

The first release.

* Set Discord Rich Presence Activity's state, details, large image, large image hover text, small image, small image hover text, start and end timestamps.
* Config file in any format.
* Dynamic config file reloading.
