# 🦀 wthrr-the-weathercrab

`wthrr` lives in your terminal and her passion is meteorology.

If you spend time in the TUI, you'll have a little companion nearby who knows about the weather.

## Contents

- [How to use?](https://github.com/tobealive/wthrr-the-weathercrab#how-to-use)
- [Showcase](https://github.com/tobealive/wthrr-the-weathercrab#showcase)
- [Config](https://github.com/tobealive/wthrr-the-weathercrab#config)
- [Installation](https://github.com/tobealive/wthrr-the-weathercrab#installation)
- [Outlook](https://github.com/tobealive/wthrr-the-weathercrab#outlook)
- [Credits](https://github.com/tobealive/wthrr-the-weathercrab#credits)

## How to use?

**Just call**

```
wthrr
```

Without having added an address or options, wthrr uses the [config](https://github.com/tobealive/wthrr-the-weathercrab#config) saved as default.<br>
If you haven't configured anything as default yet, wthrr can try to search for a weather station near you and save the searched location as default.

**It's always possible to specify an address.** E.g.,

```
wthrr melbourne
```

Depending on the place you are looking for, you might need to be more specific.
For example, the above call will get Melbourne in Australia. If you are aiming for Melbourne in the US, ask for `melbourne,florida`.
If the address contains spaces, separate them with a hyphen or enclose them in quotation marks (e.g., "new-york" or "new york").

To search explicitly for a weather station in the vicinity, call

```
wthrr auto
```

As a final example, we instruct wthrr to use Fahrenheit and mph as units and add the hourly forecast for the day

```
wthrr -u f,mph -f d
```

### Find further usage parameters in the help information

```
> wthrr -h

Usage: wthrr [OPTIONS] [ADDRESS]

Arguments:
  [ADDRESS]
          Address to check the weather

Options:
  -f, --forecast <FORECAST,...>
          [e.g.: -f w,d] [possible values: disable, (w)eek, to(d)ay, mo, tu, we, th, fr, sa, su]
  -F, --historical-weather <%Y-%m-%d,...>
          [e.g.: -F 2021-12-31]
  -u, --units <UNIT,...>
          [e.g.: -u f,12h,in] [possible values: (c)elsius, (f)ahrenheit, kmh, mph, (kn)ots, ms, 12h, 24h, %, mm, (in)ch]
  -l, --language <LANGUAGE>
          Output language [e.g.: en_US]
  -s, --save
          Save the supplied values as default
  -r, --reset
          Wipe wthrr's configuration data
  -h, --help
          Print help
  -V, --version
          Print version
```

## Showcase

<table>
  <tr>
    <th align="center">First Run</th>
    <th align="center">Hourly Forecast</th>
  </tr>
  <tr>
    <td align="center">
      <a href="https://user-images.githubusercontent.com/34311583/219735581-8036590f-8354-47fb-a31f-055be79c9229.png" target="_blank">
        <img alt="" width="400" src="https://user-images.githubusercontent.com/34311583/219735581-8036590f-8354-47fb-a31f-055be79c9229.png" />
      </a>
    </td>
    <td align="center">
      <a href="https://user-images.githubusercontent.com/34311583/219735474-d8e2899d-c209-46d3-a5cd-bea4ed41ac3c.png" target="_blank">
        <img alt="" width="400" src="https://user-images.githubusercontent.com/34311583/219735474-d8e2899d-c209-46d3-a5cd-bea4ed41ac3c.png" />
      </a>
    </td>
  </tr>
  <tr>
    <th align="center">Week Forecast</th>
    <th align="center">*Terminal Colors</th>
  </tr>
  <tr>
    <td align="center">
      <a href="https://user-images.githubusercontent.com/34311583/219735452-9766d692-a79b-4a5a-a903-30a3339cc684.png" target="_blank">
        <img alt="" width="400" src="https://user-images.githubusercontent.com/34311583/219735452-9766d692-a79b-4a5a-a903-30a3339cc684.png" />
      </a>
    </td>
    <td align="center">
      <a href="https://user-images.githubusercontent.com/34311583/219735417-6376c599-4b90-4066-8808-d9bd8649ae64.png" target="_blank">
        <img alt="" width="400" src="https://user-images.githubusercontent.com/34311583/219735417-6376c599-4b90-4066-8808-d9bd8649ae64.png" />
      </a>
    </td>
  </tr>
</table>

<sup>\*Rendering and colors are influenced by the terminal used and its theme and font.<br>
E.g., the first of the above screenshots show wthrr in nvim(toggleterm) using kitty as terminal with a Dracula theme and JetBrainsMono Nerd font. The last screenshot shows wthrr in Yakuake/Konsole, also with a Dracula color scheme.</sup>

## Config

The address, units and default forecast can be saved as default values in wthrr's config file by adding the `-s` flag to a run. This will save the config in `wthrr.ron`.

**Platform locations:**<br>
Lin: `~/.config/weathercrab/`<br>
Mac: `~/Library/Application Support/weathercrab/`<br>
Win: `C:\Users\<user>\AppData\Roaming\weathercrab\`

**Default values**

```rust
(
    address: "", // Address to check the weather, e.g.: "Berlin,DE"
    language: "en_US", // Language code of the output language
    forecast: [], // Forecast to display without adding the `-f` option: `[day]` | `[week]` | `[day, week]`
    units: (
        temperature: celsius, // Temperature units: `celsius` | `fahrenheit`
        speed: kmh, // (Wind)speed units: `kmh` | `mph` | `knots` | `ms`
        time: military, // Time Format: `military` | `ap_pm`
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
        greeting: true, // Display greeting message: `true` | `false`
    ),
)
```

## Installation

<a href="https://crates.io/crates/wthrr" target="_blank">
  <img alt="crates.io" src="https://img.shields.io/crates/v/wthrr?style=flat-square" />
</a>
<br>
<br>

Use rusts package manger to install wthrr in its latest version from source.

```
cargo install --git  https://github.com/tobealive/wthrr-the-weathercrab.git
```

Requirements and other installations methods can be found in [`INSTALL.md`](https://github.com/tobealive/wthrr-the-weathercrab/blob/main/INSTALL.md).

## Outlook

The [issues](https://github.com/tobealive/wthrr-the-weathercrab/issues) section lists some of the features that are being worked on.

Contributions like 🐛bug reports, ⭐️stars and 💡suggestions are welcome alike!

A simple changelog can be found on the [releases page](https://github.com/tobealive/wthrr-the-weathercrab/releases).

## Disclaimer

Until a stable version 1.0 is available, new features will be introduced, existing ones may change, or breaking changes may occur in minor(`0.<minor>.*`) versions.

## Credits

- The app uses the open-source weather API for non-commercial use provided by [Open Meteo](https://open-meteo.com/en)

<br>

##

<div align="right">
<a href='https://ko-fi.com/O4O4IOTYR' target='_blank'><img height='36' style='border:0px;height:36px;' src='https://storage.ko-fi.com/cdn/kofi5.png?v=3' border='0' alt='Buy Me a Coffee at ko-fi.com' /></a>
</div>
