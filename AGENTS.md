# Repository Guidelines

## Project Structure & Module Organization
- Workspace root: `Cargo.toml` defines a Cargo workspace; all crates live in `crates/*`.
- Crates:
  - `crates/rr-core` — CLI entry (currently minimal).
  - `crates/sdk` — worker process, Goridge pipes, examples and async runtime.
  - `crates/http` — HTTP plugin prototype.
  - `crates/plugins` — plugin traits and plumbing.
- Integration assets: `tests/` contains a PHP worker (`worker.php`) and Composer deps for protocol testing.
- Runtime config: `.rr-config.yaml` (RoadRunner address/worker command).

## Build, Test, and Development Commands
- Build all crates: `cargo build`
- Run unit tests: `cargo test`
- Run a specific crate: `cargo run -p sdk` or `cargo run -p rr-core`
- Format code: `cargo fmt --all` (uses `rustfmt.toml`, edition 2024)
- Lint (optional if installed): `cargo clippy --all-targets -- -D warnings`
- PHP test harness (optional): `cd tests && composer install` then use `worker.php` with the SDK worker.

## Coding Style & Naming Conventions
- Language: Rust 2024 edition; 4‑space indentation; Unix line endings.
- Naming: `snake_case` for files/modules/functions, `CamelCase` for types, `SCREAMING_SNAKE_CASE` for consts.
- Keep public APIs minimal; prefer `pub(crate)` when possible.
- Run `cargo fmt --all` before pushing; CI may reject unformatted code.

## Testing Guidelines
- Unit tests live alongside code using `#[cfg(test)]` and `#[test]` or `#[tokio::test]` for async.
- Run per-crate tests: `cargo test -p sdk` (or other crate).
- The top‑level `tests/` directory is for the PHP integration harness (not Rust integration tests).
- Aim for tests around protocol boundaries (frames, flags, I/O errors) and worker lifecycle.

## Commit & Pull Request Guidelines
- Git history shows no strict convention; use clear, imperative subjects (e.g., "Add HTTP plugin skeleton").
- Prefer scoping in the subject when helpful: `sdk: fix frame parsing`.
- Keep body brief: what/why, noteworthy tradeoffs, and migration notes.
- PRs should include: summary, linked issues, testing notes (`cargo test` output), and relevant screenshots/logs.

## Security & Configuration Tips
- The SDK depends on `goridge-rs` via a local path; ensure the path is valid or switch to a crates.io/git dependency before release.
- Do not commit secrets; review `.rr-config.yaml` before sharing logs.
