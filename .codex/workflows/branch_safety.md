# Branch Safety

Never pull, reset, checkout, merge, rebase, or synchronize against a remote branch unless explicitly instructed.

If asked to create a branch and push current work:
- create a new branch from current working tree
- preserve local changes
- do not overwrite local work
- do not sync to an older branch
- do not discard uncommitted work

Before destructive git commands, stop and require explicit confirmation.
