{
  outputs = inputs: {
    lib.patchSourceInfo = pkgs: sourceInfo: drv: pkgs.runCommand "patch-source-info"
      {
        name = "${drv.name}-with-version";
        meta.mainProgram = drv.meta.mainProgram or drv.pname;
        buildInputs = [ (pkgs.callPackage ./ver-stub-tool.nix { }) ];
        # we need to rename `outPath` otherwise `toJSON` just returns it instead of the jsonified attrset
        env.PAYLOAD = builtins.toJSON ((removeAttrs sourceInfo [ "outPath" ]) // { storePath = sourceInfo.outPath; });
      }
      ''
        cp -r --dereference ${drv} $out
        chmod +w -R $out/bin
        for file in $out/bin/*; do
          tmp=$(mktemp)
          ver-stub --custom "$PAYLOAD" patch "$file" -o "$tmp"
          mv -v "$tmp" "$file"
        done
      '';
  };
}
