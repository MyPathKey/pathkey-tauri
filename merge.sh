# --- run from your local clone of MyPathKey/pathkey-tauri ---
set -euo pipefail

# 0) Safety snapshot
git checkout -B backup/pre-tauri-merge

# 1) Switch to your working branch
git checkout dev

# 2) Add the upstream remote (idempotent) and fetch
git remote remove spell 2>/dev/null || true
git remote add spell https://github.com/SpellboundScents/tauri.git
git fetch spell --prune

# 3) Detect the upstream’s default branch (HEAD -> usually main)
UPSTREAM_HEAD=$(git remote show spell | sed -n 's/.*HEAD branch: //p')
: "${UPSTREAM_HEAD:?Upstream default branch not detected}"

# Ensure we have that branch locally
git fetch spell +"refs/heads/$UPSTREAM_HEAD:refs/remotes/spell/$UPSTREAM_HEAD"

echo "Merging spell/$UPSTREAM_HEAD into dev (preserve local edits on conflicts)…"

# 4) Merge ALL files/dirs; don’t commit yet so we can choose ours on conflicts
git merge "spell/$UPSTREAM_HEAD" --allow-unrelated-histories --no-commit --no-ff

# 5) Keep YOUR versions anywhere both sides changed
#    (Upstream-only files are still staged/added)
git checkout --ours .
git add -A

# 6) Finish the merge
git commit -m "Merge SpellboundScents/tauri (${UPSTREAM_HEAD}) into dev; keep local changes on conflicts"

 #7) Push if you want
 git push origin dev

echo "Done. Upstream files are merged; your modified files were preserved."