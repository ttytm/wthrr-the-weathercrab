# 🦀 wthrr-the-weathercrab

`wthrr` lives in your terminal and her passion is meteorology.

If you spend time in the TUI, you'll have a little companion nearby who knows about the weather.

## Contents

-  [How to use?](https://github.com/tobealive/wthrr-the-weathercrab#how-to-use)
-  [Showcase](https://github.com/tobealive/wthrr-the-weathercrab#showcase)
-  [Config](https://github.com/tobealive/wthrr-the-weathercrab#config)
-  [Installation](https://github.com/tobealive/wthrr-the-weathercrab#installation)
-  [Outlook](https://github.com/tobealive/wthrr-the-weathercrab#outlook)
-  [Credits](https://github.com/tobealive/wthrr-the-weathercrab#credits)

## How to use?

**Just call**

```
wthrr
```

With no further arguments added, wthrr will use the [config](https://github.com/tobealive/wthrr-the-weathercrab#config) that's saved as default.<br>
If you haven't configured anything yet, wthrr can try to search for a weather station near you and save the searched location as default.

**Its always possible to specify an address.** E.g.,

```
wthrr melbourne
```

Depending on the place you are looking for, you might need to be be more specific.
For example, the above call will get Melbourne in Australia. If you are aiming for Melbourne in the US, ask for `melbourne,florida`.
If the address contains spaces, separate them with a hyphen or enclose them in quotation marks (e.g., "new-york" or "new york").

If a default address is configured, but you want to explicitly search for a weather station nearby, call

```
wthrr auto
```

Example that uses Fahrenheit and mph as units and adds the hourly forecast for the day

```
wthrr -u f,mph -f -d
```

### Find further usage parameters in the help information

```
> wthrr -h

Usage: wthrr [OPTIONS] [ADDRESS] [COMMAND]

Commands:
  forecast, -f  Include the weather forecast
  help          Print this message or the help of the given subcommand(s)

Arguments:
  [ADDRESS]  Address to check the weather for

Options:
  -u, --units [<UNITS>]      Units for temperature and/or speed [possible values: (c)elsius, (f)ahrenheit, kmh, mph, (kn)ots, ms]
  -l, --language <LANGUAGE>  Output language
  -g, --greeting             Toggle greeting message
  -s, --save                 Save the supplied values as default
  -r, --reset                Wipe wthrr's configuration data
  -h, --help                 Print help information
  -V, --version              Print version information
```

## Showcase

<table>
  <tr>
    <th align="center">First Run</th>
    <th align="center">Hourly Forecast</th>
  </tr>
  <tr>
    <td align="center">
      <img alt="" width="400" src="preview/first-run.png" />
    </td>
    <td align="center">
      <img alt="" width="400" src="preview/hourly.png" />
    </td>
  </tr>
  <tr>
    <th align="center">Week Forecast</th>
    <th align="center">*Terminal Colors</th>
  </tr>
  <tr>
    <td align="center">
      <img alt="" width="400" src="preview/week.png" />
    </td>
    <td align="center">
      <img alt="" width="400" src="preview/yakuake.png" />
    </td>
  </tr>
</table>

<sup>\*Rendering and colors are influenced by the terminal used and its theme and font.<br>
For example, the first screenshots show wthrr in nvim(toggleterm) inside kitty with a Dracula theme and JetBrainsMono Nerd Font. The last screenshot shows wthrr in Yakuake / Konsole, also with a Dracula color scheme.</sup>

## Config

Values such as address and units can be stored as defaults in wthrr's config file by adding the `-s` flag to a run.

E.g., on GNU/LinuxThe the location usually is `~/.config/weathercrab/wthrr.toml`

```toml
address = 'berlin' # Address to check the weather for
greeting = true # Greeting message display [true | false]
language = 'en' # Country code of the output language ['de' | 'pl' | ...]

[units]
temperature = 'celsius' # Temperature units [celsius | fahrenheit]
speed = 'kmh' # (Wind)speed units [kmh | mph | knots | ms]
```

## Installation

Wthrr is published as a crate on [crates.io](https://crates.io/crates/wthrr) and can be installed via rusts package manager.

```
cargo install wthrr
```

For NetBSD users there's a package available from the [official repositories](https://pkgsrc.se/misc/wthrr).

```
pkgin install wthrr
```

Requirements and other installations methods can be found in [`INSTALL.md`](https://github.com/tobealive/wthrr-the-weathercrab/blob/main/INSTALL.md).

## Outlook

The [issues](https://github.com/tobealive/wthrr-the-weathercrab/issues) section lists some of the features that are being worked on.

Contributions like 🐛bug reports, ⭐️stars and 💡suggestions are welcome alike!

## Credits

-  The app uses the open-source weather API for non-commercial use provided by [Open Meteo](https://open-meteo.com/en)
