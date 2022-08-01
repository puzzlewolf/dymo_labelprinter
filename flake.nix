{
  inputs = {
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages."${system}";
    in rec {
      # `nix build`
      packages.dymoprint = pkgs.callPackage ./. { };
      packages.default = packages.dymoprint;
    }) // {
      overlays.dymoprint = (final: prev: { dymoprint = self.packages."${final.system}".dymoprint; });
      overlays.default = self.overlays.dymoprint;

      nixosModules.dymoprint = ({ pkgs, ...}: {
        nixpkgs.overlays = [ self.overlays.default ];
        imports = [ ./module.nix ];
      });
      nixosModules.default = self.nixosModules.dymoprint;
    };
}
