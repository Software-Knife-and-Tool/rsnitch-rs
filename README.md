## rsnitch-rs

Rust iced-rs UI like sntop, but with buttons you can poke.

Filter view by group.

RSNITCH_RS environment points to hosts JSON file, defaults to `~/.rsnitch-rs/hosts.json`

```
[
        {
            "group": group designator filter
            "host": DNS host name
            "label": button label
        },
        ...
]
```

