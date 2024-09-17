{
  description = "CLI for Open Data Schema";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    with flake-utils.lib;
    with nixpkgs.lib;

    let
      systems = {
        aarch64-darwin = {
          target = "aarch64-apple-darwin";
          sha256 = "10b9dc321187bf6782bf7ddee45fc5364dc0e9f18ee82c32da75736af43f5d96";
        };
        x86_64-darwin = {
          target = "x86_64-apple-darwin";
          sha256 = "8c17e07759c6b950d23d8fde9279b176689fc645c250e734c9c32b1f35730351";
        };
        x86_64-linux = {
          target = "x86_64-unknown-linux-gnu";
          sha256 = "2ca58310e5bf04400a4749f9e30293d510e1ee27be373c1d99d5c274086fbc61";
        };
        i686-linux = {
          target = "i686-unknown-linux-gnu";
          sha256 = "a4190e661a2178d9d60929d4f464910e3ceedec8d400f357bf8272cb5812e7da";
        };
      };
    in eachSystem (mapAttrsToList (n: v: n) systems) (system: {
      packages.default = with import nixpkgs { inherit system; };

        stdenv.mkDerivation rec {
          name = "ods-${version}";
          version = "0.0.1";

          nativeBuildInputs = [ unzip ];

          src = pkgs.fetchurl {
            url = "https://github.com/open-data-schema/ods/releases/download/v${version}/ods-v${version}-${systems.${system}.target}.zip";
            inherit (systems.${system}) sha256;
          };

          sourceRoot = ".";

          installPhase = ''
            install -Dm755 ods $out/bin/ods
            install -Dm755 LICENSE $out/share/licenses/ods/LICENSE
          '';

          meta = {
            description = "CLI for Open Data Schema";
            homepage = "https://github.com/open-data-schema/ods";
            platforms = [ system ];
          };
        };
    });
}
