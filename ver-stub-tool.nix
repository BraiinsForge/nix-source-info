{ lib
, rustPlatform
, fetchCrate
, llvm
, makeWrapper
, writeShellScript
, runCommand
, stdenv
}:

let
  host = stdenv.hostPlatform.rust.rustcTarget;

  # minimal fake rustc sysroot, ver-stub only looks for these two tools under {sysroot}/lib/rustlib/{host}/bin/
  fakeSysroot = runCommand "ver-stub-fake-sysroot" { } ''
    mkdir -p $out/lib/rustlib/${host}/bin
    ln -s ${llvm}/bin/llvm-objcopy  $out/lib/rustlib/${host}/bin/llvm-objcopy
    ln -s ${llvm}/bin/llvm-readobj  $out/lib/rustlib/${host}/bin/llvm-readobj
  '';

  # rustc shim that only answers the two queries ver-stub makes
  rustcShim = writeShellScript "rustc-shim" ''
    case "$1" in
      --print)
        if [ "$2" = "sysroot" ]; then
          echo "${fakeSysroot}"
          exit 0
        fi
        ;;
      -vV)
        echo "rustc 0.0.0 (ver-stub shim)"
        echo "host: ${host}"
        exit 0
        ;;
    esac
    echo "rustc-shim: called with unexpected args: $*" >&2
    exit 1
  '';
in
rustPlatform.buildRustPackage rec {
  pname = "ver-stub-tool";
  version = "0.3.0";

  src = fetchCrate {
    inherit pname version;
    hash = "sha256-op0go1SSeDWrSzRRnYTpUQs/js4xcCIxF/hw6KFIYuo=";
  };

  cargoHash = "sha256-5nDvmD8TgU+PhuSkEfW0GYfY+qccsDkoFbwuTKhB2Ts=";

  nativeBuildInputs = [ makeWrapper ];

  postInstall = ''
    wrapProgram $out/bin/ver-stub --set RUSTC ${rustcShim}
  '';

  meta = with lib; {
    description = "CLI tool for injecting version data (git, build times) into binaries without triggering rebuilds";
    homepage = "https://github.com/cbeck88/ver-stub-rs";
    license = with licenses; [ mit asl20 ];
    mainProgram = "ver-stub";
    platforms = platforms.unix;
  };
}
