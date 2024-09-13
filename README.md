# twipv

Twitch TUI client, paired with mpv.
You need an [app access token](https://dev.twitch.tv/docs/authentication/#app-access-tokens) to use Twitch API.

## Features
- List VODs from streamers in TOML config file
- List VODs from a searched streamer
- Watch selected VOD and save playback position on quit
- Display numbers of current streams (with `twipv number_lives`)
- Script rofi to display current streams (with `twipv rofi`)

## Config file
```
favorites = [
"streamer_1",
"streamer_2",
]

twitch-api-client-id = ""
twitch-api-client-secret = ""
twitch-api-client-token = ""
```

## Dependencies
- libmpv
- openssl

Use XDG Base Directory so it should be only compatible with Linux/BSD.
