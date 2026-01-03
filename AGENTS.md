# AGENTS.md

## Rules writing principles

- Keep rules concise and actionable to reduce token costs
- Remove redundant explanations and excessive examples
- Focus on essential guidelines that directly impact development

## Code language requirements

- All code, comments, and example data must be written in English
- Exception: User-facing UI messages and error messages may be localized

## Commit messages

- Follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)
- Format: `<type>(<scope>): <description>` or `<type>: <description>`
- Breaking changes: append `!` before colon (e.g., `feat(api)!: description`)
- Common types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`,
  `chore`, `ci`, `build`
- Scope is optional, use when it adds clarity (e.g., `ui`, `config`, `acp`)
- Description: imperative mood, lowercase start, no period
- Body: optional, separated by blank line, provides additional context
- Footer: optional, for metadata like `BREAKING CHANGE:` or issue references
- Do NOT include AI-generated signatures like:
  - `🤖 Generated with [Claude Code](https://claude.com/claude-code)`
  - `Co-Authored-By: Claude ... <noreply@anthropic.com>`
- Same rule applies to pull request descriptions

## Git workflow

- Separate `git add` and `git commit` into two commands
- Check `git status` before committing
- Generate two commit message options for user to choose:
  - Full message: `<type>(<scope>): <description>` with optional body
  - Title only: `<type>(<scope>): <description>` (single line)
- User will review and execute commit themselves
- Wait for user confirmation before proceeding to next task

## Build and quality checks

- Use `justfile` for all build operations - run `just` to list commands
- Run `just check` after code changes (check-fmt, clippy, test)
- Run `just all` for full clean rebuild
- Fix all linting errors before completing tasks
- Never create custom build commands when `justfile` tasks exist

## Code organization

### Import statements
- Always add imports at the file top instead of using full crate paths
- Group imports: std → external crates → crate modules
- Use `use crate::module::function_name` instead of full paths in code
- Exception: When disambiguating name conflicts, full paths are acceptable

### Function ordering in modules
- Public API functions first
- Internal helpers second (pub(crate) or private)
- Within each group: initialization → processing → query → transformation →
  utility

## Error handling

- Use `thiserror` for custom errors, `anyhow` for application-level errors
- Use `.context()` or `.with_context()` when propagating errors
- Define error types in `error.rs` or module-specific error types

## Logging and testing

- Use `tracing` with appropriate levels: `error`, `warn`, `info`, `debug`,
  `trace`
- Include context in log messages (connection info, operation names)
- Write unit tests in source files (`#[cfg(test)]`)
- Write integration tests in `tests/` directory
- Use descriptive test names

## Documentation

- Use `///` for public API doc comments
- Keep README.md and CONTRIBUTING.md up to date

## Documentation workflow

- **Before development**: Review `docs/` for requirements, designs, and
  specifications
- **During development**: Reference docs to align with requirements and
  technical decisions
- **After completion**: Update docs with new features, implementation details,
  changes, and API modifications
- Use descriptive filenames (e.g., `acp-protocol-design.md`)

## Document formatting

- Target line length: 80 characters
- Rewrap: paragraph text, list items, code comments
- Do NOT rewrap: tables, code blocks, headers, URLs, commands
- Apply to all Markdown files in `docs/` and repository root
- Break long sentences at natural boundaries (commas, conjunctions)
- Preserve indentation for continuation lines in lists
