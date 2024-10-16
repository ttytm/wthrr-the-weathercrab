# Installation

## Contents

- [Requirements](https://github.com/andygeorge/ww/blob/main/INSTALL.md#requirements)
  - [Fonts](https://github.com/andygeorge/ww/blob/main/INSTALL.md#fonts)
  - [Other requirements](https://github.com/andygeorge/ww#other-requirements)
- [Alternative installation methods](https://github.com/andygeorge/ww/blob/main/INSTALL.md#alternative-installation-methods)
  - [Build from source](https://github.com/andygeorge/ww/blob/main/INSTALL.md#build-from-source)

# Requirements

This app uses font symbols and Unicode characters. Since it runs in the terminal, it depends on the fonts available on the system and the terminal's font configuration.
This is not a pre-installation requirement. If fonts appear to be missing, you can also install them after the app was installed.

## Fonts

A Nerd Font must be used as terminal font and a Unicode symbol font must be installed on the system.

### Direct Download

To download font files directly for manual installation, use the links below. Platform-specific instructions can be found in the following togglable section.

1. [JetBrainsMono.zip](https://github.com/ryanoasis/nerd-fonts/releases/download/v3.0.2/JetBrainsMono.zip) - A font that is patched to include Nerd icons (other patched fonts would work as well). It should be configured as terminal font.

2. [NotoColorEmoji.ttf](https://raw.githack.com/googlefonts/noto-emoji/main/fonts/NotoColorEmoji.ttf), [NotoSansSymbols2-Regular.ttf](https://cdn.jsdelivr.net/gh/notofonts/notofonts.github.io/fonts/NotoSansSymbols2/unhinted/ttf/NotoSansSymbols2-Regular.ttf) - Fonts that add Unicode support. They should be installed on the System - no configuration changes are required.

<details>
<summary><kbd>toggle</kbd> <h3>Font download instructions with additional and platform-specific information</h3></summary>

#### 1. Nerd Font

A nerd font is usually a regular font that is patched to include additional glyphs.
The usage is not bound to a single font. Every font that is patched to include nerd icons can work.
This example uses the Nerd Font version of JetBrains Mono.

The nerd-fonts [github repository](https://github.com/ryanoasis/nerd-fonts) and [website](https://www.nerdfonts.com/font-downloads) make a number of patched fonts available and provide several installation options for different platforms.

Package manager installation examples are shown below.

- On macOS, using `brew`

  ```sh
  brew tap homebrew/cask-fonts  # This is only required once
  brew install font-jetbrains-mono-nerd-font  # Or any other nerd-font
  ```

- On Windows, using `choco`

  ```sh
  choco install nerd-fonts-jetbrainsmono
  ```

- On Linux, many distribution make fonts available via their package manager.

  E.g., search for the JetBrains Nerd Font on Manjaro using paru

  ```sh
  paru jetbrains nerd
  ```

**After installing the font, make sure to update your terminals font configuration!**

#### 2. Unicode symbol font

A Unicode symbol font("emoji-font") needs to be available on the system.
It is likely already installed if you see emojis correctly rendered in your browser and in other applications.
It will also allow to display line characters that are used in wthrrs daily weather graphs.
Noto fonts that add support for Unicode glyphs are usually available via the package manager.

- macOS

  ```sh
  brew install font-noto-sans-symbols-2  # Required when using e.g., iterm2 / alacritty
  ```

- Debian based distros

  ```sh
  sudo apt install fonts-noto-core
  ```

It's enough to install the font, there is no need for configuration changes.

> **Note**
> Depending on the used system and terminal another font package might be necessary. If you encounter missing glyphs in the graph: Instead of searching for the correct font package, you can also try setting a different graph style in the [config](https://github.com/andygeorge/ww#config).

</details>

## Other requirements

- Ubuntu

  ```sh
  sudo apt install libssl-dev pkg-config
  ```

  When using the binaries from the release page, you may need to add libssl manually

  ```sh
  wget http://nz2.archive.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.1_1.1.1f-1ubuntu2.16_amd64.deb ; sudo dpkg -i libssl1.1_1.1.1f-1ubuntu2.16_amd64.deb
  ```

# Alternative installation methods

There are several alternatives to the installation via `cargo`.

- Prebuilt binaries for GNU/Linux, macOS and Windows can be downloaded from the [GitHub release page](https://github.com/andygeorge/ww/releases).

- If you are on NetBSD, a package is available from the official repositories.
  To install it, simply run
  ```sh
  pkgin install ww
  ```
- A Nix flake is also available
  ```sh
  nix profile install "github:andygeorge/ww"
  ```
  ```sh
  nix run "github:andygeorge/ww"
  ```
- On Arch Linux, `ww` can be installed from the [AUR](https://aur.archlinux.org/packages?O=0&SeB=nd&K=ww&outdated=&SB=p&SO=d&PP=50&submit=Go) using an [AUR helper](https://wiki.archlinux.org/title/AUR_helpers). For example:
  ```
  paru -S ww
  ```
- On macOS, you can use Homebrew
  ```
  brew install ww
  ```

## Build from source

To compile the app yourself, clone the repository and build the release version.

```sh
git clone https://github.com/andygeorge/ww.git
cd ww
cargo build --release
```

The resulting `ww` binary will be placed in the `./target/release` directory.
