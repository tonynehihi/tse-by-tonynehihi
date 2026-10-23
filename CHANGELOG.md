# Changelog

## 1.1.1 — 2026-10-23

- **SAVE PROMPT ON QUIT**: closing the window or quitting (⌘Q / Quit) now asks what to do with every unsaved workbook — Save, Don't Save or Cancel, one workbook at a time — instead of discarding the changes silently. (Upstream GridCraft only ever asked when closing a document from the File menu; the window-close path never asked.)
- **RENAMED TO TS EXCEL**: the macOS app bundle is now **TS Excel.app**, so Spotlight finds the app when you search "Excel". The bundle id, the executable name, and all settings and data folders are unchanged — nothing to reconfigure, no data moves.

— tonynehihi

## 1.1.0 — 2026-10-22

- **NEW NAME AND IDENTITY**: the app is now **TS Excel** (TS = Tony Suite) — macOS bundle **TsE.app**, bundle id `com.tony.excel`, and its own Excel-green "X" icon in the same design language as the rest of Tony's app suite.
- **SELF-UPDATE BUILT INTO THE APP**: Help → Check for Updates, plus a prompt at launch when a newer release exists (with the changelog on screen). On macOS the app downloads the new release, builds it in-app with live progress, and installs it — the app closes only for a few seconds to swap versions, then reopens by itself.
- **UPSTREAM LINKS REMOVED FROM THE APP**: no Discord, website or GitHub links to the original project anywhere in the UI. The Help menu is now exactly: About / Check for Updates / System Info / Keyboard Shortcuts.
- Settings and data folders keep their original GridCraft names, so existing settings carry over.

— tonynehihi
