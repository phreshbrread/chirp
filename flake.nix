{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
  let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in
  {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
      ];

      nativeBuildInputs = with pkgs; [
        cargo
        rustc
        rustfmt
        clippy
        rust-analyzer
        cmake
        rustPlatform.bindgenHook
        llvmPackages.libclang
      ];

      BINDGEN_EXTRA_CLANG_ARGS = "-I${pkgs.glibc.dev}/include";

      RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
    };
  };
}

