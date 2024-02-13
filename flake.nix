{
  description = "SunServer";

  inputs={
    nixpkgs.url = "nixpkgs/nixpkgs-unstable";
  };
  
  outputs = {self, nixpkgs}: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    defaultPackage.${system} = with pkgs; stdenv.mkDerivation {
      name = "SunServer";
      src = self;

      buildInputs = [
        openssl
        pkg-config
        rustc
        cargo
        cargo-watch
        just
        bacon
      ];

      shellHook = ''
        export OPENSSL_DIR=${openssl.dev}
        export OPENSSL_LIB_DIR=${openssl.out}/lib
        export OPENSSL_INCLUDE_DIR=${openssl.dev}/include
      '';
    };
  };
}
