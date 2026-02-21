{
  tableaux,
  nodejs_24,
  pnpm_10,
  pnpmConfigHook,
  fetchPnpmDeps,
  stdenv,
}:
stdenv.mkDerivation (finalAttrs: {
  pname = "tableaux-playground";
  version = "0.1.0";

  src = builtins.path {
    path = ../playground;
    name = finalAttrs.pname;
  };

  nativeBuildInputs = [
    nodejs_24
    pnpm_10
    (pnpmConfigHook.override { pnpm = pnpm_10; })
  ];

  EXAMPLES_GRAHAM_PRIEST_PATH = ../examples-graham-priest.toml;

  pnpmDeps = fetchPnpmDeps {
    inherit (finalAttrs) pname version src;
    pnpm = pnpm_10;
    fetcherVersion = 3;
    hash = "sha256-h2PjIFWLZDgdNeIgdvAFG6X5vUzXSEzbIgwS1D0FPkc=";
  };

  installPhase = ''
    runHook preInstall

    mkdir -p $out
    rm rust-pkg
    ln -s ${tableaux}/pkg rust-pkg

    pnpm run build
    mkdir -p $out/playground/build
    cp -r build $out/playground/

    pnpm prune --prod
    # Clean up broken symlinks left behind by `pnpm prune`
    # https://github.com/pnpm/pnpm/issues/3645
    find node_modules -xtype l -delete

    cp -r node_modules package.json $out/playground/
    cp -r ${tableaux}/pkg $out/

    runHook postInstall
  '';
})
