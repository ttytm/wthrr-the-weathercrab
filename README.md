# ww

[![][ci_shield]](https://github.com/andygeorge/ww/actions/workflows/ci.yml?query=branch%3Amain)
[![][last_commit_shield]](https://github.com/andygeorge/ww/commits/main)
[![][crates_io_shield]](https://crates.io/crates/ww)
[![][msrv_shield]](https://github.com/andygeorge/ww)

<div align="center">

[![][preview]][preview]

</div>

`ww` lives in your terminal and her passion is meteorology.

If you spend time in the TUI, you'll have a little companion nearby who knows about the weather.

## Contents

- [How to use?](https://github.com/andygeorge/ww#how-to-use)
- [Showcase](https://github.com/andygeorge/ww#showcase)
- [Config](https://github.com/andygeorge/ww#config)
- [Installation](https://github.com/andygeorge/ww#installation)
- [Outlook](https://github.com/andygeorge/ww#outlook)
- [Credits](https://github.com/andygeorge/ww#credits)

## How to use?

**Just call**

```
ww
```

Without having added an address or options, ww uses the [config](https://github.com/andygeorge/ww#config) saved as default.<br>
If you haven't configured anything as default yet, ww can try to search for a weather station near you and save the searched location as default.

**It's always possible to specify an address.** E.g.,

```
ww melbourne
```

Depending on the place you are looking for, you might need to be more specific.
For example, the above call will get Melbourne in Australia. If you are aiming for Melbourne in the US, ask for `melbourne,florida`.
If the address contains spaces, separate them with a hyphen or enclose them in quotation marks (e.g., `new-york` or `"new york"`).

To search explicitly for a weather station in the vicinity, call

```
ww auto
```

As a final example, we instruct ww to use Fahrenheit and mph as units and add the hourly forecast for the day

```
ww -u f,mph -f d
```

### Find further usage parameters in the help information

```
> ww -h

Usage: ww [OPTIONS] [ADDRESS]

Arguments:
  [ADDRESS]
          Address to check the weather

Options:
  -f, --forecast <FORECAST,...>
          [e.g.: -f w,d] [possible values: disable, (w)eek, to(d)ay, (t)omorrow, mo, tu, we, th, fr, sa, su]
  -F, --historical-weather <%Y-%m-%d,...>
          [e.g.: -F 2021-12-31]
  -u, --units <UNIT,...>
          [e.g.: -u f,12h,in] [possible values: (c)elsius, (f)ahrenheit, kmh, mph, (kn)ots, ms, 12h, 24h, %, mm, (in)ch]
  -l, --language <LANGUAGE>
          Output language [e.g.: en_US]
  -s, --save
          Save the supplied values as default
  -r, --reset
          Wipe ww's configuration data
  -h, --help
          Print help
  -V, --version
          Print version
```

## Showcase

|                                         |                                         |
| :-------------------------------------: | :-------------------------------------: |
|              **First Run**              |           **Hourly Forecast**           |
|       [![][first_run]][first_run]       | [![][hourly_forecast]][hourly_forecast] |
|            **Week Forecast**            |          **\*Terminal Colors**          |
| [![][weekly_forecast]][weekly_forecast] | [![][terminal_colors]][terminal_colors] |
|                                         |                                         |

<sup>\*Rendering and colors are influenced by the terminal used and its theme and font.<br>
E.g., the first of the above screenshots show ww in nvim(toggleterm) using kitty as terminal with a Dracula theme and JetBrainsMono Nerd font. The last screenshot shows ww in Yakuake/Konsole, also with a Dracula color scheme.</sup>

## Config

The address, units and default forecast can be saved as default values in ww's config file by adding the `-s` flag to a run. This will save the config in `ww.ron`.

**Platform locations:**<br>
Lin: `~/.config/ww/`<br>
Mac: `~/Library/Application Support/ww/`<br>
Win: `%USERPROFILE%\AppData\Roaming\ww\`

**Default values**

```rust
(
    address: "", // Address to check the weather, e.g.: "Berlin,DE"
    language: "en_US", // Language code of the output language
    forecast: [], // Forecast to display without adding the `-f` option: `[day]` | `[week]` | `[day, week]`
    units: (
        temperature: celsius, // Temperature units: `celsius` | `fahrenheit`
        speed: kmh, // (Wind)speed units: `kmh` | `mph` | `knots` | `ms`
        time: military, // Time Format: `military` | `am_pm`
        precipitation: probability, // Precipitation units: `probability` | `mm` | `inch`
    ),
    gui: (
        border: rounded, // Border style: `rounded` | `single` | `solid` | `double`
        color: default, // Color: `default` | `plain`
        graph: (
            // Graph style: lines(solid) | lines(slim) | lines(dotted) | dotted | custom((char; 8))
            // `custom` takes exactly 8 chars. E.g. using a set of 4 chars: `custom(('⡀','⡀','⠄','⠄','⠂','⠂','⠁','⠁'))`,
            style: lines(solid),
            rowspan: double, // Graph height: `double` | `single`
            time_indicator: true, // Indication of the current time in the graph: `true` | `false`
        ),
        greeting: false, // Display greeting message: `true` | `false`
    ),
)
```

## Installation

Use rusts package manger to install ww.


**From crates.io**

|             |                       |
| ----------- | --------------------- |
| **Version** | **Command**           |
| release     | `cargo install ww` |
|             |                       |
| development | _not available_       |
|             |                       |

**From git source**

|             |                                                                                   |
| ----------- | --------------------------------------------------------------------------------- |
| **Version** | **Command**                                                                       |
| release     | `cargo install --git https://github.com/andygeorge/ww --tag v1.1.1` |
|             |                                                                                   |
| development | `cargo install --git https://github.com/andygeorge/ww`              |
|             |                                                                                   |

**Requirements and alternative, platform-specific installation instructions can be found in [`INSTALL.md`](https://github.com/andygeorge/ww/blob/main/INSTALL.md).**

> **Important**
> To display symbols correctly, the used terminal must be configured to use a NerdFont.

## Outlook

The [issues](https://github.com/andygeorge/ww/issues) section lists some of the features that are being worked on.

Contributions like 🐛bug reports, ⭐️stars and 💡suggestions are welcome alike!

A simple changelog can be found on the [releases page](https://github.com/andygeorge/ww/releases).

## Contributors

<a href="https://github.com/andygeorge/ww/graphs/contributors">
  <img height='48' src="https://contrib.rocks/image?repo=andygeorge/ww&columns=24" />
</a>

## Credits

- The app uses the open-source weather API for non-commercial use provided by [Open Meteo](https://open-meteo.com/en)

<br>

<!-- Images -->

[preview]: https://github.com/andygeorge/ww/assets/34311583/58780205-816b-4cfd-95f8-9453e754eb94
[crates_io_shield]: https://img.shields.io/crates/v/ww?style=flat-square&color=DEA584
[ci_shield]: https://img.shields.io/github/actions/workflow/status/andygeorge/ww/ci.yml?branch=main&style=flat-square
[last_commit_shield]: https://img.shields.io/github/last-commit/andygeorge/ww?style=flat-square
[msrv_shield]: https://img.shields.io/badge/MSRV-1.74.0-DEA584?style=flat-square
[first_run]: https://user-images.githubusercontent.com/34311583/219735581-8036590f-8354-47fb-a31f-055be79c9229.png
[hourly_forecast]: https://user-images.githubusercontent.com/34311583/219735474-d8e2899d-c209-46d3-a5cd-bea4ed41ac3c.png
[weekly_forecast]: https://user-images.githubusercontent.com/34311583/219735452-9766d692-a79b-4a5a-a903-30a3339cc684.png
[terminal_colors]: https://user-images.githubusercontent.com/34311583/219735417-6376c599-4b90-4066-8808-d9bd8649ae64.png
