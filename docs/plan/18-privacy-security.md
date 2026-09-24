# 18. Privacy and security requirements

- Local-first by default.
- OAuth tokens in the operating system credential store.
- No secrets in Markdown, SQLite exports, or `.obsidian/`.
- Per-provider AI consent.
- Clear preview of context sent to an AI model.
- Ability to disable all AI without losing core functionality.
- Loopback API authenticated and origin-restricted.
- Live Markdown and export files use durable atomic replacement where supported, with recovery when file and metadata commits diverge.
- Resolve and validate workspace paths; do not follow note paths or symlinks outside the selected workspace.
- Database backups encrypted when requested.
- Connector permissions kept to the smallest feasible scope.
- User-visible audit history for imports and generated classifications.
- Ability to delete cached extracted text independently of source metadata.

This aligns with the repository’s existing direction that secrets should not be committed and local Obsidian configuration should be treated cautiously. AGENTS.md:40-42

---

[← Previous](17-testing.md) · [Plan index](README.md) · [Next →](19-open-decisions.md)
