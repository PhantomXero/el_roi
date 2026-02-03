
# Commit changes with a message
commit MESSAGE:
    echo "[JUST] Adding all changes to git..."
    git add .
    echo "[JUST] Committing with message: '{{MESSAGE}}'"
    git commit -m "{{MESSAGE}}"


# Push changes to remote
push:
    echo "[JUST] Pushing to remote repository..."
    git push


# Build the Rust project
build:
    echo "[JUST] Building the project..."
    cargo build


# Run tests for the Rust project
test:
    echo "[JUST] Running tests..."
    cargo test

