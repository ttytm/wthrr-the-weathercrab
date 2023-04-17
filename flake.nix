{
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    };

    flake-utils = {
      url = "github:numtide/flake-utils";
    };
  };
  outputs = inputs @ {
    self,
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      wthrr-the-weathercrab = {
        lib,
        openssl,
        pkg-config,
        rustPlatform,
      }:
        rustPlatform.buildRustPackage {
          name = "wthrr-the-weathercrab";
          src = lib.cleanSource ./.;
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = [
            pkg-config
            rustPlatform.bindgenHook
          ];
          buildInputs = [openssl];

          checkFlags = [
            # connecting to internet does not work in the sandbox
            "--skip=modules::location::tests::geolocation_response"
            "--skip=modules::localization::tests::translate_string"
          ];
          
          meta = with lib; {
            license = licenses.mit;
            homepage = "https://github.com/tobealive/wthrr-the-weathercrab";
            platforms = platforms.all;
          };
        };
    in {
      packages.default = pkgs.callPackage wthrr-the-weathercrab {};
      apps.default = {
        type = "app";
        program = "${self.outputs.packages.${system}.default}/bin/wthrr";
      };
    });
}
