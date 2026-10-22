<p align="center">
  <img alt="TS Excel" src="assets/app-icon/tse-icon-1024.png" width="180">
</p>

<h1 align="center">TS Excel</h1>

<p align="center">
  <b>A personal fork of <a href="https://github.com/storytold/gridcraft">GridCraft</a> — rebranded and customized by Tony for his own use.</b><br>
  Not an official release. Personal use only.
</p>

## Origin

All of the original source code in this project is [GridCraft](https://github.com/storytold/gridcraft) by **storytold** — a clean-room, open-source spreadsheet application written in Rust, released under the **MIT / Apache-2.0** licenses (see `LICENSE-MIT` and `LICENSE-APACHE` in this repo, kept unchanged from the original).

This repository is **not** the official GridCraft and is not a public product release. It is a copy that Tony took, renamed to **TS Excel** (TS = Tony Suite), and adjusted for his own personal workflow. All credit for the original software belongs to storytold and the GridCraft contributors.

> "Excel" is a trademark of Microsoft. This personal project is not affiliated with or endorsed by Microsoft in any way; the name is used only for Tony's private use.

## What Tony changed

- **Name & identity**: the app is called **TS Excel**, with its own Excel-green "X" icon, About screen and macOS bundle (**TsE.app**, bundle id `com.tony.excel`).
- **Built-in self-update**: *Help → Check for Updates* — the app checks the Releases of this repo for the newest version, shows the changelog in a launch prompt, then downloads, builds and installs the update on macOS by itself.
- **No upstream links in the app**: the Discord / website / GitHub links to the original project were removed from the Help menu, the About screen and the menus; the app's Help menu is just About / Check for Updates / System Info / Keyboard Shortcuts.
- **Your data stays put**: settings and data folders keep their original GridCraft names, so existing settings and recent-file lists carry over untouched.

## Installing on macOS

The Mac build is a source build, compiled with Cargo on Tony's own machine. In short:

```bash
cargo build --release -p gridcraft
packaging/macos/package.sh --arch aarch64
```

The DMG lands in `dist/release/`. Full instructions (clean install, removing the old build, Gatekeeper troubleshooting) are in `packaging/macos/HUONG-DAN.md` (in Vietnamese).

Once installed, there is no need to repeat these steps for each new version: just open the app and click **Check for Updates**.

## License

Kept from the original project: **MIT OR Apache-2.0**. Tony's modifications in this repo are offered under the same terms, with the attribution above.
