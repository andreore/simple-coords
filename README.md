# Show Coordinates

A Horizon mod that shows your coordinates the way **Bedrock Edition** does with
*Show Coordinates* on:

```
X: 123, Y: 65, Z: -45
```

One line, top-left corner, and nothing else. `/coords` hides or shows the panel.

## Capabilities

`game-read` (the player's position) and `hud` (the panel).

## Build

```sh
cargo install --git https://github.com/horizon-rust/horizon cargo-horizon-mod
cargo horizon-mod build      # writes target/horizon-mod/coords_bedrock-0.1.0.hmod
cargo horizon-mod check target/horizon-mod/coords_bedrock-0.1.0.hmod
```

Put the `.hmod` in Horizon's `horizon-mods/` folder (Options → Mods → Open Mod
Folder) and turn it on in Options → Mods.

## Licence

MIT.
