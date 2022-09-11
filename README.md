# 🦀 wthrr-the-weathercrab

When spending a lot of time in the CLI it can be handy to have a little companion in there who knows about the weather.

That's where `wthrr` comes in. She lives in your terminal and her passion is the weather.

Even when you don't spend a lot of time in the terminal. She has a sunny personality, so you can visit her once in a while.<br>
<sub>_Little tip: ask her about the weather._</sub>

## Contents

-  [How to use?](https://github.com/tobealive/wthrr-the-weathercrab#how-to-use)
-  [Requirements](https://github.com/tobealive/wthrr-the-weathercrab#requirements)
-  [Installation](https://github.com/tobealive/wthrr-the-weathercrab#installation)
-  [Outlook](https://github.com/tobealive/wthrr-the-weathercrab#outlook)

## How to use?

**Just call**

```
wthrr
```

When no address is specified wthrr will use your default [config](https://github.com/tobealive/wthrr-the-weathercrab#config).<br>
If you don't have a configuration yet, wthrr can try to search for a weather station close to you and
save the last searched location as default.

**You can always specify any address with your call.** E.g.,

``` 
wthrr melbourne
```

Depending on the location you search for, you need to be more specific.
For example, the call above will give you Melbourne in Australia. If you are aiming for Melbourne in the US, ask for `melbourne,florida`.
For cities containing spaces, write it separated with a dash or wrap it in quotation marks (e.g., `new-york` or `'new york'`).

If there is a default address configured, but you want wthrr to explicitly search for a nearby weather station, you can do so by calling:

```
wthrr auto
```

### The help information contains further usage parameters

```
> wthrr -h

USAGE:
    wthrr [ADDRESS] [OPTIONS]

ARGS:
    <ADDRESS>    Address to check the weather for

OPTIONS:
    -f, --forecast               Include the forecast for one week
    -g, --greeting               Toggle greeting message
    -h, --help                   Print help information
    -l, --language <LANGUAGE>    Output language [default: 'en']
    -r, --reset-config           Wipe wthrr's configuration data
    -s, --save-config            Save the supplied values as default
    -u, --unit <UNIT>            Unit of measurement ['c' (°Celsius) | 'f' (°Fahrenheit)]
    -V, --version                Print version information
```

---

_First run example asking for the forecast of the week_
<img src="preview/first-run-example.png" />

---

### Config

Adding the `-s` flag will save the values from the run as default.
E.g., on GNU/Linux the location of the config file usually is: `~/.config/weathercrab/wthrr.toml`

You probably don't have to bother with the config file itself, as you can save your defaults directly via the terminal.
For the sake of completeness, the config contents are as follows.

```toml
# Address to check the weather for
address = 'berlin,germany'
# Temperature unit: 'celsius' || 'fahrenheit'
unit = 'celsius'
# Greeting message display: true || false 
greeting = true
# Language of the output: 'de' || 'pl' ...
language = 'en'
```

## Requirements

This app uses font icons and emojis. Therefore, a nerd variant of your font is required to correctly display the output.
https://github.com/ryanoasis/nerd-fonts

<sub>If you are using brew, this gist contains easily digestible copy-pasta for nerd-font installation.<br>
https://gist.github.com/davidteren/898f2dcccd42d9f8680ec69a3a5d350e</sub>

Some terminal emulators might require to additionally add/prioritize emojis in their font config.

## Installation

Without the rust toolchain installed, grabbing a binary from the [release](https://github.com/tobealive/wthrr-the-weathercrab/releases) page might be your way to go.

Otherwise, rusts package manager is a simple way to install the binary crate:

```
cargo install wthrr
```

If you are on NetBSD, a package is available from the official repositories.
To install it, simply run:

```
pkgin install wthrr
```

A Nix flake is also avaiable:

```
nix profile install "github:tobealive/wthrr-the-weathercrab"
```

```
nix run "github:tobealive/wthrr-the-weathercrab"
```

### Build from source

Another way is to compile the app yourself.
Assuming the rust toolchain is installed on your system, just clone the repo and build the release version.

```
git clone https://github.com/tobealive/wthrr-the-weathercrab.git
cd wthrr-the-weathercrab
cargo build --release
```

And you'll find the `wthrr` binary inside the `./target/release` directory

## Outlook

- [x] Multilang support
- [ ] Allow to show more / customized meteorological data
- [ ] Custom number of forecast days
- [ ] Theme variants

Contributions like 🐛bug reports, ⭐️stars and 💡suggestions are welcome alike.

## Credits

-  The app uses the open-source weather API for non-commercial use provided by [Open Meteo](https://open-meteo.com/en)

