{
  rustPlatform,
  wasm-pack,
  wasm-bindgen-cli_0_2_100,
  lld,
}:
rustPlatform.buildRustPackage rec {
  pname = "tableaux";
  version = "0.1.0";
  src = builtins.path {
    path = ../.;
    name = pname;
  };

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  nativeBuildInputs = [
    wasm-pack
    wasm-bindgen-cli_0_2_100
    lld
  ];

  buildPhase = ''
    export HOME=$out/home
    echo "Starting custom build phase"
    wasm-pack build --release --out-dir $out/pkg --mode no-install
  '';
  installPhase = "echo 'Skipping installPhase'";
}
