# Update README & CHANGELOG Skill

Automatically updates README.md (if structural changes detected) and CHANGELOG.md based on git changes between the current branch and main. Optionally commits the changes.

## How to Use

Run this skill when you're ready to document changes before merging a feature branch:

```bash
/update-docs
```

The skill will:
1. Analyze changes between your branch and main
2. Update CHANGELOG.md with a new version entry
3. Update README.md if there are structural changes
4. Ask if you want to commit

---

## Implementation

### Step 1: Detect Changes

Analyze what changed between current branch and main.

```bash
# Get list of changed files
git_changes=$(git diff main --name-status)

# Check for structural changes (new directories in src/)
has_structure_change=$(echo "$git_changes" | grep -E "^[A-D]\s+src/[^/]+/$" || echo "")

# Check for files with new directories
struct_files=$(find src -type d -newer .git/refs/heads/main 2>/dev/null | grep -v "^\." || echo "")

echo "=== Git Changes Detected ==="
echo "$git_changes"
echo ""
```

### Step 2: Parse Current Version & Auto-Increment

Extract version from CHANGELOG and prepare new version.

```bash
# Get current version from CHANGELOG (first line after # )
current_version=$(head -1 CHANGELOG.md | sed 's/^# //')
current_date=$(echo "$current_version" | awk '{print $NF}')
version_num=$(echo "$current_version" | awk '{print $1}')

# Auto-increment patch version
IFS='.' read -r major minor patch <<< "$version_num"
patch=$((patch + 1))
new_version="${major}.${minor}.${patch}"

# Get today's date
new_date=$(date +%Y-%m-%d)

echo "=== Version Info ==="
echo "Current version: $version_num"
echo "New version: $new_version"
echo "New date: $new_date"
echo ""
```

### Step 3: Ask for Changelog Description

Prompt user to describe the changes.

**User Input Needed**: Short description of changes for CHANGELOG

---

### Step 4: Detect Architecture Changes & Prepare README Update

Check if structural changes warrant a README update.

```bash
# Function to generate architecture tree
generate_architecture() {
  cat << 'EOF'
config-manager/
├── src/
│   ├── main.rs                      # CLI entry point
│   ├── lib.rs                       # Library exports
│   ├── error.rs                     # Error handling
│   ├── cli.rs                       # Command parser
│   ├── config/                      # Config management
│   │   ├── mod.rs                   # Module file
│   │   ├── config_format.rs         # Format detection
EOF
  
  # Dynamically add discovered directories
  find src/config -type f -name "*.rs" -not -path "*/\.*" | sort | while read f; do
    indent="│   │   │   "
    filename=$(basename "$f")
    echo "$indent├── $filename"
  done
  
  cat << 'EOF'
│   ├── editor/                      # Editor operations
│   │   ├── mod.rs
│   │   ├── file_config.rs           # Configuration wrapper
│   │   └── file_repository.rs       # File operations
│   ├── handler/                     # Command handlers
│   │   ├── mod.rs
│   │   ├── list_handler.rs          # List command logic
│   │   └── edit_handler.rs          # Edit command logic
└── Cargo.toml                       # Manifest
EOF
}

# Check if README exists and has Project Architecture section
if grep -q "## Project Architecture" README.md; then
  echo "=== Detected README Architecture Section ==="
  echo "README.md will be updated with new architecture structure"
  echo ""
fi
```

### Step 5: Update CHANGELOG

Insert new entry at the top of CHANGELOG.md.

```bash
# Create temporary file with new changelog entry
{
  echo "# $new_version $new_date"
  echo ""
  echo "## Changes"
  echo ""
  echo "- $changelog_description"
  echo ""
  # Append existing CHANGELOG content
  tail -n +1 CHANGELOG.md
} > CHANGELOG.md.tmp

mv CHANGELOG.md.tmp CHANGELOG.md

echo "✓ CHANGELOG.md updated with version $new_version"
```

### Step 6: Update README (if needed)

Update Project Architecture section if changes detected.

```bash
# Only update README if structural changes detected
if [ -n "$has_structure_change" ] || [ -n "$struct_files" ]; then
  
  # Find the line numbers of the Architecture section
  start_line=$(grep -n "## Project Architecture" README.md | cut -d: -f1)
  
  if [ -n "$start_line" ]; then
    # Find where the next section starts (next ## header)
    next_section=$(tail -n +$((start_line + 1)) README.md | grep -n "^## " | head -1 | cut -d: -f1)
    end_line=$((start_line + next_section - 1))
    
    if [ -z "$next_section" ]; then
      end_line=$(wc -l < README.md)
    fi
    
    # Generate new architecture
    new_arch=$(generate_architecture)
    
    # Reconstruct README with updated architecture
    {
      head -n $start_line README.md
      echo ""
      echo "\`\`\`"
      echo "$new_arch"
      echo "\`\`\`"
      echo ""
      tail -n +$((end_line + 1)) README.md
    } > README.md.tmp
    
    mv README.md.tmp README.md
    echo "✓ README.md updated with new architecture"
  fi
else
  echo "ℹ No structural changes detected - README.md not modified"
fi
```

### Step 7: Show Changes Preview

Display what will be committed.

```bash
echo ""
echo "=== Preview of Changes ==="
echo ""
echo "--- CHANGELOG.md (first 15 lines) ---"
head -15 CHANGELOG.md
echo ""
echo "--- README.md Architecture Section ---"
grep -A 30 "## Project Architecture" README.md | head -35
echo ""
```

### Step 8: Ask for Commit Confirmation

Prompt user if they want to commit these changes.

**User Input Needed**: 
- Commit changes? (yes/no)
- Commit type: (feat/fix/chore/hotfix)
- Commit message: (or use changelog description)

---

### Step 9: Execute Git Commit

Run git add and commit if user confirmed.

```bash
if [ "$commit_confirmed" = "yes" ]; then
  git add README.md CHANGELOG.md
  
  # Build commit message
  if [ -z "$commit_message" ]; then
    commit_message="$changelog_description"
  fi
  
  commit_cmd="git commit -m \"$commit_type: $commit_message\""
  
  echo "Executing: $commit_cmd"
  eval "$commit_cmd"
  
  if [ $? -eq 0 ]; then
    echo "✓ Changes committed successfully"
    echo ""
    echo "Commit info:"
    git log -1 --oneline
  else
    echo "✗ Commit failed"
  fi
else
  echo "ℹ Skipped git commit. Changes are ready in README.md and CHANGELOG.md"
fi
```

---

## Notes

- The skill automatically increments the patch version (e.g., 0.0.2 → 0.0.3)
- README is only updated if structural changes are detected in `src/`
- CHANGELOG is always updated with the new version
- Commit is optional - you can review changes before committing
- All file updates are atomic and reversible with `git checkout`

## Environment

- **Working directory**: Project root
- **Requirements**: `git`, `bash`, `grep`, `find`, `awk`, `sed`
- **Tested with**: macOS, Linux
