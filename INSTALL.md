## Requirements

This app uses font icons and emojis. Therefore, having a font with Unicode support installed and using a nerd variant of your terminal font is required to display the output correctly.

- [nerd fonts](https://github.com/ryanoasis/nerd-fonts)
- [unicode font](https://github.com/googlefonts/noto-emoji/blob/main/fonts/NotoColorEmoji.ttf) (If none is installed by default, noto font packages are usually available via your distribution's package manager)

<!--<sub>If you are using brew, this gist contains easily digestible 🍝 copy-pasta for nerd-font installation.<br>
https://gist.github.com/davidteren/898f2dcccd42d9f8680ec69a3a5d350e</sub>-->

#### For ubuntu based distros:

- ```
  sudo apt install fonts-noto-core libssl-dev
  ```

- When using the binaries, you may need to add libssl manually
  ```
  wget http://nz2.archive.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.1_1.1.1f-1ubuntu2.16_amd64.deb ; sudo dpkg -i libssl1.1_1.1.1f-1ubuntu2.16_amd64.deb
  ```

## Installation

There are several alternatives to `cargo install wthrr`

- Prebuilt binaries for GNU/Linux, macOS and Windows can be downloaded from the the [GitHub release page](https://github.com/tobealive/wthrr-the-weathercrab/releases).

- If you are on NetBSD, a package is available from the official repositories.
  To install it, simply run
  ```
  pkgin install wthrr
  ```
- A Nix flake is also avaiable
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

When the build has finished, you'll find the `wthrr` binary inside the `./target/release` directory
