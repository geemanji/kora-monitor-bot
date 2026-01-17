#!/bin/bash

# Make scripts executable
chmod +x scripts/pre-commit

# Install git hook
echo "Installing git pre-commit hook..."
ln -sf ../../scripts/pre-commit .git/hooks/pre-commit

echo "Local environment setup complete!"
