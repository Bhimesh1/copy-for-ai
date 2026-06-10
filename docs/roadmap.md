# Roadmap

Copy for AI will be built phase by phase.

## Phase 0 - Project foundation

- Repository setup
- Documentation
- License
- Contribution guide
- Code of conduct
- Security policy
- GitHub issue templates
- Pull request template
- Initial CI workflow

## Phase 1 - Core converter

Goal:

Convert supported files into Markdown or plain text.

Initial file types:

- TXT
- Markdown
- PDF with selectable text
- Images using OCR later

## Phase 2 - Clipboard command

Goal:

Convert a file and copy the output directly to the clipboard.

Example:

```text
copy-for-ai copy ./example.pdf
```

## Phase 3 - Desktop app

Goal:

Create a lightweight desktop app with:

* System tray
* Settings
* Drag-and-drop file input
* Toast notifications

## Phase 4 - Windows integration

Goal:

Add Windows-first workflow features:

* Right-click file → Copy for AI
* Global hotkey
* Explorer integration

## Phase 5 - OCR and advanced formatting

Goal:

Improve extraction quality.

- Image OCR
- Scanned PDF OCR
- Table handling
- Metadata controls
- Output profiles

## Phase 6 - Packaging and release

Goal:

Ship installable builds.

- Windows installer
- GitHub releases
- Release notes
- Versioning

## Future ideas

These are intentionally out of scope for the first versions:

- macOS support
- Browser extension
- React/UI element source context copying
- Cloud OCR providers
- Direct auto-submit to AI tools