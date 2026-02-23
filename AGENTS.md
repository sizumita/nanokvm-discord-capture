# Repository Guidelines

## Project Structure & Module Organization
This repository is a small Rust component that imports NanoKVM capture APIs via WIT.
- `src/main.rs`: current application entrypoint.
- `wit/world.wit`: world definition (`imago:nanokvm/capture@0.1.0` import).
- `wit/deps/`: resolved WIT dependency packages managed by lock data.
- `imago.toml`: component metadata, build command, capabilities, and target settings.
- `imago.lock`: pinned WIT sources/digests for reproducible builds.
- `certs/`: local client certificate/key material.
- `build/` and `target/`: generated artifacts (do not hand-edit).

When code grows, keep runtime logic in `src/` modules (for example `src/capture.rs`) and keep `main.rs` thin.

## Build, Test, and Development Commands
- `cargo build`: fast local compile check.
- `cargo build --target wasm32-wasip2 --release`: produces the deployable `.wasm`.
- `cargo test`: runs all unit/integration tests.
- `cargo fmt --all`: formats Rust code.
- `cargo clippy --all-targets --all-features -- -D warnings`: lint gate used before review.
- `imago build` (if CLI is installed): executes the `[build.command]` from `imago.toml` and refreshes `build/manifest.json`.

## Coding Style & Naming Conventions
Use Rust 2024 defaults and `rustfmt` output as source of truth.
- Indentation: 4 spaces, no tabs.
- Naming: `snake_case` (functions/variables), `PascalCase` (types/traits), `UPPER_SNAKE_CASE` (consts).
- Prefer explicit error handling over unchecked `unwrap()` in long-running paths.
- Environment variables should remain uppercase with `NANOKVM_` prefix (for example `NANOKVM_USERNAME`).

## Testing Guidelines
No strict coverage threshold is defined yet, but every behavior change should include tests or a documented smoke check.
- Unit tests: colocate with code using `#[cfg(test)]`.
- Integration tests: place in `tests/`.
- Test names should describe behavior, e.g. `captures_jpeg_with_valid_login`.
- For WIT/config changes, at minimum run `cargo build --target wasm32-wasip2 --release`.

## Commit & Pull Request Guidelines
History is minimal (`Init imago`), so use clear imperative subjects going forward.
- Commit subject style: short imperative line (<= 72 chars), e.g. `capture: handle auth failure`.
- Keep commits focused (avoid mixing refactors with dependency lock updates unless required).
- PRs should include: purpose, key changes, test commands/results, and any config/cert impact.
- Link related issues and include logs/screenshots when runtime behavior changes.

## Security & Configuration Tips
- Never commit real credentials or private keys.
- Treat `.env` and `certs/client.key` as local secrets.
- Verify `imago.toml` target fields (`remote`, `server_name`) before running against non-local environments.
