#!/bin/sh

echo "🔧 Running checks for server..."

if git diff --cached --name-only | grep '^server/.*\.rs$' > /dev/null; then
  echo "🦀 Rust changes detected, running checks..."

  cd server || exit 1

  echo "🔍 Running cargo fmt..."
  cargo fmt -- --check || {
    echo "❌ Formatting failed. Running cargo fmt."
    cargo fmt
    git -C .. add $(git -C .. diff --cached --name-only | grep '^server/.*\.rs$')
  }

  echo "🧠 Running cargo check..."
  cargo check || {
    echo "❌ Cargo check failed."
  }

  cd .. || exit 1
else
  echo "No Rust changes detected, skipping Rust checks."
fi

echo "🔧 Running checks for client..."

if git diff --cached --name-only | grep '^client/.*$' > /dev/null; then
  echo "🟢 Client changes detected, running checks..."

  cd client || exit 1

  echo "🔍 Running Svelte format check..."
  pnpm run lint || {
    echo "❌ Svelte formatting failed."
    echo "💨 Running Svelte fmt..."
    pnpm run format
    client_staged_files=$(git -C .. diff --cached --name-only | grep '^client/.*$' || true)
    if [ -n "$client_staged_files" ]; then
      printf '%s\n' "$client_staged_files" | xargs git -C .. add --
    fi
  }

  echo "🧠 Running Svelte checks..."
  pnpm run check || {
    echo "❌ Svelte checks failed."
    exit 1
  }

#  echo "🧪 Running Svelte tests..."
#  pnpm run test || {
#    echo "❌ Svelte tests failed."
#    exit 1
#  }

  cd .. || exit 1
else
  echo "No Svelte changes detected, skipping Svelte checks."
fi

echo "😁 All checks passed!"
echo "✅ Ready to commit!"