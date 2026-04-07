# Zenn CLI Guide Notes

Source: <https://zenn.dev/zenn/articles/zenn-cli-guide> (opened on 2026-04-07)

## Articles

- Store article files under `articles/`.
- Create a new article with `npx zenn new:article`.
- The official CLI can auto-generate a random slug, but this skill overrides that behavior:
  always ask for a slug first and run `npx zenn new:article --slug <slug>`.
- Article slug rule: use only `a-z0-9`, `-`, `_`, with length `12` to `50`.
- Generated article front matter typically contains:
  - `title`
  - `emoji`
  - `type`
  - `topics`
  - `published`
- Preview with `npx zenn preview`.
- Optional preview flags:
  - `--port <port>`
  - `--no-watch`

## Publishing

- Push commits to the GitHub repository connected to Zenn to deploy content.
- Deployment is skipped if the commit message contains `[ci skip]` or `[skip ci]`.
- Use `published_at` for scheduled publication.
- `published_at` uses JST.

## Books

- Store books under `books/<book-slug>/`.
- Typical book structure:
  - `config.yaml`
  - `cover.png` or `cover.jpeg`
  - chapter markdown files
- `config.yaml` commonly includes:
  - `title`
  - `summary`
  - `topics`
  - `published`
  - `price`
  - `chapters`
  - optional `toc_depth`
- Chapter files use front matter with at least `title`.
- Free preview chapters can set `free: true`.
