# llm-observer

A lightweight background daemon that intercepts Claude Code hooks to detect and mask PII before requests reach Anthropic servers.

## What it does

- Runs a local HTTP server that Claude Code hooks POST events to
- Scans prompt content for PII patterns (emails, phone numbers, SSNs, API keys, etc.)
- Masks sensitive data in-place before it leaves your machine
- Stores a log of intercepted events in a local SQLite database
- Provides a web UI to inspect activity and review what was masked

## License

MIT
