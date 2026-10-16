{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
  let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};

    buildDeps = with pkgs; [
      cargo
      rustc
      rustfmt
      clippy
      rust-analyzer
      cmake
      rustPlatform.bindgenHook
      llvmPackages.libclang
    ];

    runtimeDeps = with pkgs; [
      libxkbcommon
      wayland
      libX11
      libXcursor
      libXi
      libXrandr
    ];
  in
  {
    devShells.${system}.default = pkgs.mkShell {
      nativeBuildInputs = buildDeps;
      buildInputs = runtimeDeps;

      BINDGEN_EXTRA_CLANG_ARGS = "-I${pkgs.glibc.dev}/include";

      RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";

      LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath runtimeDeps;
    };
  };
}

