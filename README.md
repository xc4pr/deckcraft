# Publishing queue

One file per item under `queue/`, written for storytold/deckcraft. Each file states its kind
(`issue+pr`, `pr` or `review`), the target repo, the base and head branch (or the PR number and the
commit that was checked), and the title and body to post. A PR body says `Fixes #ISSUE` when its
issue does not exist yet: open the issue first and put its number there.
