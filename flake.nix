{
  description = "Thinnerthinker's personal website";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    tomers = {
      url = "github:thinnerthinker/tomers";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, tomers, ... }:
    tomers.inputs.flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        buildFilePatterns = [ ".*/templates/.*" ".*/assets/.*" ];
        targetPlatforms = [
          {
            system = "x86_64-unknown-linux-gnu";
            arch = "x86_64-linux";
            depsBuild = with pkgs; [ patchelf ];
            postInstall = crateName: ''
                find $out -type f -exec sh -c '
                if file "$1" | grep -q "ELF .* executable"; then
                  patchelf --set-interpreter "/lib64/ld-linux-x86-64.so.2" "$1"
                fi
              ' sh {} \;
            '';
            inherit buildFilePatterns;
          }
          {
            system = "aarch64-unknown-linux-gnu";
            arch = "aarch64-linux";
            depsBuild = with pkgs; [
              qemu
            ];
            postInstall = crateName: ''
              find $out -type f -exec sh -c '
                if file "$1" | grep -q "ELF .* executable"; then
                  patchelf --set-interpreter "/lib/ld-linux-aarch64.so.1" "$1"
                fi
              ' sh {} \;
            '';
            inherit buildFilePatterns;
            env = {
              CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER = "${pkgs.stdenv.cc.targetPrefix}cc";
              CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER = "qemu-aarch64";
              cargoExtraArgs = "--target aarch64-unknown-linux-gnu";

              HOST_CC = "${pkgs.stdenv.cc.nativePrefix}cc";
              TARGET_CC = "${pkgs.stdenv.cc.targetPrefix}cc";

              nativeBuildInputs = with pkgs; [
                patchelf
                pkg-config
              ] ++ lib.optionals stdenv.buildPlatform.isDarwin [
                libiconv
              ];
            };
          }
        ];
        tomersLib = tomers.libFor system targetPlatforms;
      in
      rec {
        packagesForEachPlatform = tomersLib.packagesForEachPlatform;
        devShellsForEachPlatform = tomersLib.devShellsForEachPlatform;

        packages = packagesForEachPlatform ./.;
        devShells = devShellsForEachPlatform ./.;
      }
    );
} 
 