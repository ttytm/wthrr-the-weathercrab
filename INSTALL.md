## Requirements

This app uses font symbols and emojis. Therefore, font configuration is the primary requirement.

- Set your default terminal font to a nerd font.
  - For installation check the nerd-fonts github repository: [ryanoasis/nerd-fonts](https://www.nerdfonts.com/font-downloads).
  - Alternatively they are available on the nerdfonts website: [nerdfonts.com/font-downloads](https://github.com/ryanoasis/nerd-fonts).
  - For installation via `brew`, see below.
- A Unicode symbol font needs to be available on the system.
  - If none is installed by default, noto font packages are usually available via your distribution's package manager.

### Debian based distros

- Install package and font dependencies

  ```
  sudo apt install libssl-dev pkg-config fonts-noto-core
  ```

- When using the binaries from the release page, you may need to add libssl manually

  ```
  wget http://nz2.archive.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.1_1.1.1f-1ubuntu2.16_amd64.deb ; sudo dpkg -i libssl1.1_1.1.1f-1ubuntu2.16_amd64.deb
  ```

### MacOS

- Install cask-fonts, a unicode symbol font and a nerd font
  ```
  brew tap homebrew/cask-fonts
  brew install font-noto-sans-symbols-2    # Required when using e.g., iterm2 / alacritty
  brew install font-jetbrains-mono-nerd-font    # Or any other nerd-font
  ```

## Installation

There are several alternatives to `cargo install wthrr` from crates.io.

- Prebuilt binaries for GNU/Linux, macOS and Windows can be downloaded from the the [GitHub release page](https://github.com/tobealive/wthrr-the-weathercrab/releases).

- If you are on NetBSD, a package is available from the official repositories.
  To install it, simply run
  ```
  pkgin install wthrr
  ```
- A Nix flake is also available
  ```
  nix profile install "github:tobealive/wthrr-the-weathercrab"
  ```
  ```
  nix run "github:tobealive/wthrr-the-weathercrab"
  ```
- Use cargo to install the from souce
  ```
  cargo install --git  https://github.com/tobealive/wthrr-the-weathercrab.git `
  ```

### Build from source

Another way is to compile the app yourself.
Assuming the rust toolchain is installed on your system, just clone the repo and build the release version.

```
git clone https://github.com/tobealive/wthrr-the-weathercrab.git
cd wthrr-the-weathercrab
cargo build --release
```

When the build has finished, you'll find the `wthrr` binary inside the `./target/release` directory
