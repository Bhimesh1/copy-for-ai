# Copy for AI

Copy for AI is a local-first desktop utility that converts files into AI-ready Markdown or text and copies the result directly to your clipboard.

The goal is simple:

> Right-click or hotkey on a file, convert it into clean AI-ready context, and paste it into ChatGPT, Claude, Cursor, or any other AI tool.

## Current status

This project is in early development.

The first target platform is Windows.

## Planned features

- Convert PDFs to Markdown/text
- Convert images/screenshots using OCR
- Convert text-based files into clean AI-ready context
- Copy converted output directly to clipboard
- Show estimated token count
- Support output modes such as Markdown, plain text, and prompt-wrapped Markdown
- Add Windows right-click menu integration
- Add global hotkey support
- Add settings for formatting, metadata, OCR, and clipboard behavior

## Product principles

- Clipboard-first
- Local-first
- Simple by default
- Configurable for power users
- Open-source friendly
- No auto-submit to AI tools in v1

## Development

Development instructions will be added in [`docs/development.md`](docs/development.md).

## CLI preview

The first development milestone is a small CLI.

Example:

```bash
cargo run -p copy-for-ai -- convert ./sample.txt
```

Output formats:

```
cargo run -p copy-for-ai -- convert ./sample.txt --format markdown
cargo run -p copy-for-ai -- convert ./sample.txt --format text
cargo run -p copy-for-ai -- convert ./sample.txt --format prompt
```

Output styles:

```
cargo run -p copy-for-ai -- convert ./sample.txt --style clean
cargo run -p copy-for-ai -- convert ./sample.txt --style compact
cargo run -p copy-for-ai -- convert ./sample.txt --style detailed
```

Token estimate only:
```
cargo run -p copy-for-ai -- convert ./sample.txt --tokens-only
```

## Roadmap

See [`docs/roadmap.md`](docs/roadmap.md).

## Contributing

Contributions are welcome. See [`CONTRIBUTING.md`](CONTRIBUTING.md).

## Security

See [`SECURITY.md`](SECURITY.md).

## License

This project is licensed under the MIT License.