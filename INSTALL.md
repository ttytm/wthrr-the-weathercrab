# Installation

## Contents

- [Requirements](https://github.com/ttytm/wthrr-the-weathercrab/blob/main/INSTALL.md#requirements)
  - [Fonts](https://github.com/ttytm/wthrr-the-weathercrab/blob/main/INSTALL.md#fonts)
  - [Other requirements](https://github.com/ttytm/wthrr-the-weathercrab#other-requirements)
- [Alternative installation methods](https://github.com/ttytm/wthrr-the-weathercrab/blob/main/INSTALL.md#alternative-installation-methods)
- [Build from source](https://github.com/ttytm/wthrr-the-weathercrab/blob/main/INSTALL.md#build-from-source)

## Requirements

This app uses font symbols and Unicode characters. Since it runs in the terminal, it depends on the fonts available on the system and the terminal's font configuration.

On windows it is advised to use [Windows Terminal](https://apps.microsoft.com/store/detail/windows-terminal/9N0DX20HK701) instead of the standard terminal.

### Fonts

#### Nerd Font

A nerd font is usually a regular font that is patched to include additional glyphs.
The nerd-fonts github repository ([ryanoasis/nerd-fonts](https://www.nerdfonts.com/font-downloads)) or the nerdfonts website ([nerdfonts.com/font-downloads](https://github.com/ryanoasis/nerd-fonts)) provide a number of patched fonts. Download and install a font from the above mentioned sources or via your systems package manager if it makes the fonts available.

**Make sure to configure your terminal to use the installed font.**

- On macOS using `brew`

  ```sh
  brew tap homebrew/cask-fonts   # This is only required once
  brew install font-jetbrains-mono-nerd-font  # Or any other nerd-font
  ```

- On Windows, you can follow the installation steps of [Oh My Posh](https://ohmyposh.dev/docs/installation/windows) to nerdify your power shell and install a Nerd Font.

#### Unicode symbol font

A Unicode symbol font("emoji-font") needs to be available on the system.
It is required for emojis and things like line characters in the daily weather graph to be displayed properly on your system.
Noto font packages are usually available via your distribution's package manager.

It's enough to install the font, there is no need for configuration changes.

- macOS

  ```sh
  brew install font-noto-sans-symbols-2  # Required when using e.g., iterm2 / alacritty
  ```

- Debian based distros

  ```sh
  sudo apt install fonts-noto-core
  ```

If you still encounter problems with the graph in the used terminal: Instead of searching for the correct font package, you can also try setting a different graph style in the [config](https://github.com/ttytm/wthrr-the-weathercrab#config).

### Other requirements

- Debian based distros

  ```sh
  sudo apt install libssl-dev pkg-config
  ```

  When using the binaries from the release page, you may need to add libssl manually

  ```sh
  wget http://nz2.archive.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.1_1.1.1f-1ubuntu2.16_amd64.deb ; sudo dpkg -i libssl1.1_1.1.1f-1ubuntu2.16_amd64.deb
  ```

## Alternative installation methods

There are several alternatives to the installation via `cargo`.

- Prebuilt binaries for GNU/Linux, macOS and Windows can be downloaded from the the [GitHub release page](https://github.com/ttytm/wthrr-the-weathercrab/releases).

- If you are on NetBSD, a package is available from the official repositories.
  To install it, simply run
  ```sh
  pkgin install wthrr
  ```
- A Nix flake is also available
  ```sh
  nix profile install "github:ttytm/wthrr-the-weathercrab"
  ```
  ```sh
  nix run "github:tobealive/wthrr-the-weathercrab"
  ```
- On Arch Linux `wthrr` can be installed from the [AUR](https://aur.archlinux.org/packages?O=0&SeB=nd&K=wthrr&outdated=&SB=p&SO=d&PP=50&submit=Go) using an [AUR helper](https://wiki.archlinux.org/title/AUR_helpers). For example:
  ```
  paru -S wthrr
  ```

### Build from source

To compile the app yourself, clone the repository and build the release version.

```sh
git clone https://github.com/ttytm/wthrr-the-weathercrab.git
cd wthrr-the-weathercrab
cargo build --release
```

When the build has finished, you will find the `wthrr` binary inside the `./target/release` directory.
