# Changelog

## 1.2.0 — 2026-10-28

- **SYNC WITH UPSTREAM GRIDCRAFT v0.12.1**: 777 upstream commits since 1.1.1, including 100% Stacked Bar and 100% Stacked Area charts, PivotTable Show Details from the cell menu, Fill Days / Weekdays / Months / Years for dates, Paste Name (F3) and an improved Go To, and circular-reference tracking in the status bar and Error Checking.
- **ASK-TO-SAVE ON QUIT, KEPT AND STRENGTHENED**: the fork's close handshake survives the merge — closing the window or quitting (⌘Q / Quit) still asks about every unsaved workbook, one at a time, instead of discarding changes silently. It now also runs inside the app's own logic, so the prompt behaves the same on every close path.
- **TS EXCEL BRANDING KEPT**: the macOS app menu, About, and File tab all still say TS Excel, with no upstream links anywhere in the UI.

— tonynehihi

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
