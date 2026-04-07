---
name: zenn-cli
description: Manage Zenn articles and books in a repository that uses Zenn CLI. Use when Codex needs to create, preview, or update Zenn article files under `articles/`, manage Zenn books under `books/`, run `npx zenn` commands, or prepare Git work for publishing through a Zenn-connected repository.
---

# Zenn CLI

Use this skill to work in repositories that manage Zenn content with `zenn-cli`.

## Workflow

1. Confirm that the repository is a Zenn CLI project.
2. Inspect `package.json`, `articles/`, `books/`, and existing Markdown files before making changes.
3. Use the article workflow below for `npx zenn new:article`.
4. Use the book workflow below for `npx zenn new:book` or chapter edits.
5. Use `npx zenn preview` when the user wants local rendering.

Read [references/zenn-cli-guide.md](./references/zenn-cli-guide.md) when you need command rules, file layout, or front matter details.

## Article Workflow

Treat this workflow as mandatory whenever creating a new article.

1. Ask the user for the article slug with `AskQuestion` before running any article creation command.
2. Require a non-empty slug that matches Zenn's article slug rule: `a-z0-9`, `-`, `_`, length `12` to `50`.
3. Create or switch to a Git branch whose name is exactly that slug.
4. Run `npx zenn new:article --slug <slug>`.
5. Never run `npx zenn new:article` without `--slug`.

Use this branch flow:

- If the branch does not exist, run `git switch -c <slug>`.
- If the branch already exists, run `git switch <slug>`.

After file creation:

- Edit the generated file in `articles/<slug>.md`.
- Fill front matter such as `title`, `emoji`, `type`, `topics`, and `published`.
- Preserve the filename slug. Changing it creates a different article on Zenn.

## Book Workflow

Use `npx zenn new:book` when the user wants to create a new book.

- Ensure the repository has been initialized for Zenn content.
- Check the generated `books/<book-slug>/config.yaml`.
- Manage chapter order through the `chapters:` array in `config.yaml`.
- Keep chapter filenames within Zenn's allowed pattern and write chapter front matter with at least `title`.

## Preview And Publish

- Use `npx zenn preview` for local preview.
- Respect optional preview flags such as `--port` and `--no-watch` when the user asks.
- Publishing is Git-driven: commit and push to the branch configured in the Zenn-connected GitHub repository.
- Warn that `[ci skip]` or `[skip ci]` in the commit message prevents Zenn deployment.

## Working Rules

- Prefer existing repository conventions for article metadata and topics.
- When editing an existing article, keep the same slug filename unless the user explicitly wants a new article.
- If `articles/` or `books/` is missing, initialize the repo with `npx zenn init` before creating content.
- If the user asks for reserved publication time, use `published: true` and `published_at` in JST format.
