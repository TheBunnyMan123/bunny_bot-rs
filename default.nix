{
   pkgs ? import <nixpkgs>,
   lib ? pkgs.lib,
   rustPlatform,
   ...
}: rustPlatform.buildRustPackage {
   pname = "bunny_bot";
   version = "0.1.0";
   src = lib.cleanSource ./.;

   cargoLock.lockFile = ./Cargo.lock;
   cargoHash = "";

   buildInputs = with pkgs; [
      openssl.dev
   ];
   nativeBuildInputs = with pkgs; [
      pkg-config
   ];

   meta = {
      description = "TheBunnyBot, written in rust!";
      homepage = "https://github.com/TheBunnyMan123/bunny_bot-rs";
      license = lib.licenses.asl20;
      maintainers = [];
   };
}

