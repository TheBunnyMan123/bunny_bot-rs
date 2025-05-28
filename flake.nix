{
   description = "The flake for TheBunnyBot";

   inputs = {
      nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
   };

   outputs = { self, nixpkgs }: let
      supportedSystems = [ "x86_64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      pkgsFor = nixpkgs.legacyPackages;
   in {
      packages = forAllSystems (system: {
         default = pkgsFor."${system}".callPackage ./default.nix {};
      });
      devShells = forAllSystems (system: {
         default = pkgsFor."${system}".mkShell {
            buildInputs = with pkgsFor."${system}"; [
               rustc
               cargo
               openssl.dev
            ];
            nativeBuildInputs = with pkgsFor."${system}"; [
               pkg-config
            ];
         };
      });
   };
}

