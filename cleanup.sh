#!/bin/bash
# Equans Operational Insights - Cleanup Script (Linux/Mac)
# This script removes all generated files before committing to Git

echo "🧹 Cleaning Equans Operational Insights project..."
echo ""

# Get script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
cd "$SCRIPT_DIR"

# Clean Backend (Rust)
echo "🦀 Cleaning Rust backend..."
if [ -d "backend" ]; then
    cd backend
    
    if [ -d "target" ]; then
        echo "  - Removing target/ folder..."
        cargo clean
    fi
    
    if [ -f "Cargo.lock" ]; then
        echo "  - Removing Cargo.lock..."
        rm -f Cargo.lock
    fi
    
    cd "$SCRIPT_DIR"
    echo "  ✅ Backend cleaned"
else
    echo "  ⚠️  Backend folder not found"
fi

echo ""

# Clean Frontend (Node.js)
echo "⚛️  Cleaning React frontend..."
if [ -d "frontend" ]; then
    cd frontend
    
    if [ -d "node_modules" ]; then
        echo "  - Removing node_modules/ folder..."
        rm -rf node_modules
    fi
    
    if [ -d "dist" ]; then
        echo "  - Removing dist/ folder..."
        rm -rf dist
    fi
    
    if [ -d "dist-ssr" ]; then
        echo "  - Removing dist-ssr/ folder..."
        rm -rf dist-ssr
    fi
    
    if [ -f "package-lock.json" ]; then
        echo "  - Removing package-lock.json..."
        rm -f package-lock.json
    fi
    
    cd "$SCRIPT_DIR"
    echo "  ✅ Frontend cleaned"
else
    echo "  ⚠️  Frontend folder not found"
fi

echo ""

# Clean logs and temporary files
echo "📝 Cleaning logs and temporary files..."
LOG_COUNT=$(find . -name "*.log" -type f 2>/dev/null | wc -l)
if [ "$LOG_COUNT" -gt 0 ]; then
    echo "  - Removing $LOG_COUNT log file(s)..."
    find . -name "*.log" -type f -delete 2>/dev/null
fi

echo "  ✅ Logs cleaned"

echo ""

# Show summary
echo "✨ Cleanup complete!"
echo ""
echo "Your project is now ready for Git."
echo "You can now run:"
echo "  git add ."
echo "  git commit -m 'Your commit message'"
echo "  git push"
echo ""

# Optional: Show what would be committed
if command -v git &> /dev/null; then
    echo "📊 Checking Git status..."
    git status --short
    echo ""
    
    # Show repository size
    echo "📦 Repository size check:"
    git count-objects -vH 2>/dev/null || echo "Repository not initialized"
else
    echo "⚠️  Git not found. Install Git to use version control."
fi

echo ""
