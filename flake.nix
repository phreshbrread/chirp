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
      nativeBuildInputs = with pkgs; [
        cargo
        rustc
        rustfmt
        clippy
        rust-analyzer
        cmake
        rustPlatform.bindgenHook
        llvmPackages.libclang

        # Raylib
        raylib
        cmake
        libxi
        libx11
        libxinerama
        libxrandr
        libxcursor
        clang
        glfw
        wayland
        libGL
        clang
        glfw
        wayland
        alsa-lib
      ];

      BINDGEN_EXTRA_CLANG_ARGS = "-I${pkgs.glibc.dev}/include";

      RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";

      shellHook = ''
        export LD_LIBRARY_PATH=${pkgs.libGL}/lib:${pkgs.libx11}/lib:${pkgs.libxrandr}/lib:${pkgs.libxinerama}/lib:${pkgs.libxcursor}/lib:${pkgs.libxi}/lib:${pkgs.alsa-lib}/lib:$LD_LIBRARY_PATH
      '';
    };
  };
}

