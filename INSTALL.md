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

#### 1. Nerd Font

A nerd font is usually a regular font that is patched to include additional glyphs.
The [nerd-fonts github repository](https://www.nerdfonts.com/font-downloads) and the [nerdfonts website](https://github.com/ryanoasis/nerd-fonts) make a number of patched fonts available. Download and install a font from these sources, or use your system's package manager if it provides the fonts.

- On macOS using `brew`

  ```sh
  brew tap homebrew/cask-fonts   # This is only required once
  brew install font-jetbrains-mono-nerd-font  # Or any other nerd-font
  ```

- On Linux, it depends on the used distribution if fonts are available via the package mangager.\
  If you have `subversion` installed, you can download single files and directories directly from a Github repository on any distro.

  ```sh
  # Example downloading a font into the font directory, make sure to update paths accodringly
  cd ~/.local/share/fonts
  svn export "https://github.com/ryanoasis/nerd-fonts/trunk/patched-fonts/JetBrainsMono/Ligatures" NerdFonts/JetBrainsMono
  fc-cache -fv # Update font cache
  ```

- On Windows, install the font directly or follow the installation steps of [Oh My Posh](https://ohmyposh.dev/docs/installation/windows) to nerdify your power shell and install a Nerd Font.

**Make sure to configure your terminal to use the installed font.**

#### 2. Unicode symbol font

A Unicode symbol font("emoji-font") needs to be available on the system.
It will allow to display unicode characters like emojis and line characters that are used in the daily weather graph on your system.
Noto font packages are usually available via your distribution's package manager.

- macOS

  ```sh
  brew install font-noto-sans-symbols-2  # Required when using e.g., iterm2 / alacritty
  ```

- Debian based distros

  ```sh
  sudo apt install fonts-noto-core
  ```

It's enough to install the font, there is no need for configuration changes.

Depending on the used system and terminal another font package might be necessary. If you encounter missing glyphs in the the graph: Instead of searching for the correct font package, you can also try setting a different graph style in the [config](https://github.com/ttytm/wthrr-the-weathercrab#config).

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
