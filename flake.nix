{
  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  outputs =
    { self
    , nixpkgs
    , flake-utils
    , rust-overlay
    , ...
    }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs { inherit system overlays; };
      rustVersion = pkgs.rust-bin.stable.latest.default;
    in
    {
      devShells.default = pkgs.mkShell {
        buildInputs = [
          (rustVersion.override { extensions = [ "rust-src" ]; })
        ];
        packages = with pkgs; [
          spirv-tools
        ];
      };
      defaultPackage = pkgs.rustPlatform.buildRustPackage
        {
          pname = "rivi-std";
          version = "0.1.0";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes = {
              "rivi-loader-0.2.0" = "5Xr+itpPZ4ZF3GPNlz8NGdiNFyu3JZSdvZQi5jSEZog=";
            };
          };
        };
    });
}
