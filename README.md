## rsnitch-rs

Under development. Usable, but unpretty.

Rust *iced-rs* UI like *sntop*, but with buttons you can poke.

Filter view by group.

RSNITCH_RS environment variable points to a hosts JSON file, defaults to 

`~/.rsnitch-rs/hosts.json`

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



Building *rsnitch*

------

You can build *rsnitch* through the usual mechanism.

```
cargo build
```

The included `makefile` has some convenience functions

```
make help
```

will tell you about them. For example,

```
make commit
```

runs the formatter and linters.



Running *rsnitch*

------

Create a `~/.config/.rsnitch-rs`directory and put your `hosts.json` in it. You can use a `hosts.json` by pointing directly to it with the RSNITCH_HOSTS environment variable.

```
RSNITCH_HOSTS=path_to_host.json cargo run
```

