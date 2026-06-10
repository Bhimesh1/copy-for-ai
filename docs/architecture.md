# Architecture

Copy for AI is designed as a local-first clipboard utility.

## Core flow

```text
User action
  ↓
File path or clipboard input
  ↓
Converter
  ↓
Formatter
  ↓
Token estimator
  ↓
Clipboard
  ↓
Notification
```

## Main components
### Desktop app

The desktop app will provide:

- Tray menu
- Settings UI
- Drag-and-drop file input
- Notifications
- Windows integration hooks

### Converter

The converter turns supported files into structured text or Markdown.

Initial responsibilities:

- Read file input
- Detect file type
- Extract text
- Return normalized content

### Formatter

The formatter controls what gets copied.

Output formats may include:

- Markdown
- Plain text
- Prompt-wrapped Markdown

### Token estimator

The token estimator gives approximate token counts.

Initial implementation may use a simple approximation:

``estimated tokens = characters / 4``

More accurate model-specific tokenizers can be added later.

### Clipboard layer

The clipboard layer copies the final output to the system clipboard.

## Platform integration

Windows integration will include:

- Context menu
- Global hotkey
- Toast notifications

macOS support may be added later with equivalent platform-specific integrations.

