{
  tableaux,
  nodejs,
  pnpm_10,
  bash,
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
    nodejs
    pnpm_10.configHook
  ];

  EXAMPLES_GRAHAM_PRIEST_PATH = ../examples-graham-priest.toml;

  pnpmDeps = pnpm_10.fetchDeps {
    inherit (finalAttrs) pname version src;
    hash = "sha256-OJ+GUDy1Od4D6e4LCe8EmdJQCFtmISzmXqS/RYP4Kmw=";
    fetcherVersion = 2;
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

    mkdir -p $out/bin
    echo "\
    #!${bash}/bin/bash
    ${nodejs}/bin/node $out/playground/build
    " > $out/bin/${finalAttrs.pname}

    chmod ugo+x $out/bin/${finalAttrs.pname}

    runHook postInstall
  '';
})
