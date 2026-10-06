{
  lib,
  rustPlatform,
  fetchFromGitHub,
  pkg-config,
  wrapGAppsHook3,
  gtk3,
  gtk-layer-shell,
  dbus,
  glib,
  adwaita-icon-theme,
}:

rustPlatform.buildRustPackage (finalAttrs: {
  pname = "barricade-bar";
  version = "0.1.0";

  src = fetchFromGitHub {
    owner = "yarok-k";
    repo = "barricade-bar";
    rev = "master"; # лучше закрепить конкретный коммит
    hash = "sha256-vF+Gito5A2sfNawLXY85+9XiYbH0ljwXp7XVyic2x5g="; # nix сам покажет правильный хеш при первой сборке
  };

  cargoLock.lockFile = "${finalAttrs.src}/Cargo.lock";

  nativeBuildInputs = [
    pkg-config
    wrapGAppsHook3
  ];

  buildInputs = [
    gtk3
    gtk-layer-shell
    dbus
    glib
    adwaita-icon-theme
  ];

  meta = {
    description = "Rust GTK3 based statusbar for niri";
    homepage = "https://github.com/yarok-k/barricade-bar";
    license = lib.licenses.asl20;
    mainProgram = "barricade_bar";
    platforms = lib.platforms.linux;
  };
})
