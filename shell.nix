{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  buildInputs = with pkgs; [
    xorg.libX11
    xorg.libXi
    libGL
    # pkgs.mesa
    # pkgs.alsa-lib
  ];
  
  # shellHook = ''
  #   export 
  LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${with pkgs; pkgs.lib.makeLibraryPath [ libGL xorg.libX11 xorg.libXi ]}";
}