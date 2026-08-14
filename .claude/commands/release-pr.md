---
description: Open or refresh the dev → main release PR with a written title and description
argument-hint: "[extra context or emphasis for the description]"
allowed-tools: Bash(gh:*), Bash(git:*), Read, Write
---

Open the `dev` → `main` release PR, or refresh the one that already exists, giving it a real title and a written description of everything being promoted to production.

`main` is production; `dev` is where reviewed feature PRs land. This PR is the integration merge, not new work — say so.

$ARGUMENTS

## 1. Gather

Run these first (they are read-only, batch them):

- `git fetch origin main dev` then `git diff --stat origin/main...origin/dev` — the real file/line footprint.
- `gh pr list --state open --base main --head dev --json number,title,body` — is there already an open release PR to update rather than duplicate?
- `git log --oneline origin/main..origin/dev` — every commit being promoted.
- `gh pr list --state merged --base dev --limit 20 --json number,title,body,mergedAt` — the child PRs whose commits make up this range. **Read their bodies.** They hold the reasoning, the behaviour changes and the honest "not verified" notes that belong in the release description; the release PR summarises them rather than re-deriving anything from the diff.
- `gh issue list --state open --json number,title` — an issue a child PR said it "Closes"/"Completes" is still open when that PR merged into `dev` instead of the default branch. This release PR is what actually closes it, so its body needs the `Closes #N` line.

Read the diff itself only where the child PRs leave something unexplained — commits in the range that belong to no child PR (direct pushes to `dev`) still need describing.

## 2. Write

Title: conventional-commit style, lowercase after the type, naming the headline user-visible change — the same shape as the merged PR titles in `gh pr list --base main`. Not `Dev`, not "Release", not a date.

Body, in this repo's voice — written prose that explains *why*, tables where a table genuinely beats sentences, and no bullet-dump filler:

1. `Closes #N` on the first line when a child PR's issue is still open, then one sentence saying this promotes `dev` to production and listing the child PR numbers already reviewed and merged there.
2. **What ships** — the headline features, each with the reasoning that makes the design make sense, condensed from the child PR bodies. Detail lives in the child PRs; do not restate all of it.
3. **Behaviour changes worth knowing** — anything an existing user or an existing database will notice on upgrade: data-format changes with no migration path, defaults that flip, newly editable fields. This section is the point of the release PR; never omit it when such a change exists.
4. **Verification** — the test commands that pass and what they cover, then plainly what is *still unproven* (unsigned bundles, untested-on-device paths, known gaps and what closing them needs). Carry the child PRs' honesty forward; do not quietly upgrade "compiles" into "works".

Write the body to a file in the scratchpad directory rather than passing it as a shell argument — it is long and full of backticks.

## 3. Apply

- No open PR: `gh pr create --base main --head dev --title "..." --body-file <path>`
- Already open: `gh pr edit <number> --title "..." --body-file <path>`

Then `gh pr view <number>` to confirm, and report the URL plus a short summary of the framing you chose — especially the `Closes` line and the behaviour changes you called out. Do not merge the PR.
