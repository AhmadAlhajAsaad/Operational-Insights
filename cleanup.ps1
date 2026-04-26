# Equans Operational Insights - Cleanup Script
# This script removes all generated files before committing to Git

Write-Host "Cleaning Equans Operational Insights project..." -ForegroundColor Cyan
Write-Host ""

# Get current location
$projectRoot = $PSScriptRoot
if (-not $projectRoot) {
    $projectRoot = Get-Location
}

Set-Location $projectRoot

# Clean Backend (Rust)
Write-Host "Cleaning Rust backend..." -ForegroundColor Yellow
if (Test-Path "backend") {
    Set-Location "backend"
    
    if (Test-Path "target") {
        Write-Host "  - Removing target/ folder..."
        cargo clean
    }
    
    if (Test-Path "Cargo.lock") {
        Write-Host "  - Removing Cargo.lock..."
        Remove-Item "Cargo.lock" -Force -ErrorAction SilentlyContinue
    }
    
    Set-Location $projectRoot
    Write-Host "  [OK] Backend cleaned" -ForegroundColor Green
} else {
    Write-Host "  [WARNING] Backend folder not found" -ForegroundColor Yellow
}

Write-Host ""

# Clean Frontend (Node.js)
Write-Host "Cleaning React frontend..." -ForegroundColor Yellow
if (Test-Path "frontend") {
    Set-Location "frontend"
    
    if (Test-Path "node_modules") {
        Write-Host "  - Removing node_modules/ folder..."
        Remove-Item -Recurse -Force "node_modules" -ErrorAction SilentlyContinue
    }
    
    if (Test-Path "dist") {
        Write-Host "  - Removing dist/ folder..."
        Remove-Item -Recurse -Force "dist" -ErrorAction SilentlyContinue
    }
    
    if (Test-Path "dist-ssr") {
        Write-Host "  - Removing dist-ssr/ folder..."
        Remove-Item -Recurse -Force "dist-ssr" -ErrorAction SilentlyContinue
    }
    
    if (Test-Path "package-lock.json") {
        Write-Host "  - Removing package-lock.json..."
        Remove-Item "package-lock.json" -Force -ErrorAction SilentlyContinue
    }
    
    Set-Location $projectRoot
    Write-Host "  [OK] Frontend cleaned" -ForegroundColor Green
} else {
    Write-Host "  [WARNING] Frontend folder not found" -ForegroundColor Yellow
}

Write-Host ""

# Clean logs and temporary files
Write-Host "Cleaning logs and temporary files..." -ForegroundColor Yellow
$logFiles = Get-ChildItem -Path . -Filter "*.log" -Recurse -ErrorAction SilentlyContinue
if ($logFiles.Count -gt 0) {
    Write-Host "  - Removing $($logFiles.Count) log file(s)..."
    $logFiles | Remove-Item -Force -ErrorAction SilentlyContinue
}

Write-Host "  [OK] Logs cleaned" -ForegroundColor Green

Write-Host ""

# Show summary
Write-Host "Cleanup complete!" -ForegroundColor Green
Write-Host ""
Write-Host "Your project is now ready for Git." -ForegroundColor Cyan
Write-Host "You can now run:" -ForegroundColor Cyan
Write-Host "  git add ." -ForegroundColor White
Write-Host "  git commit -m 'Your commit message'" -ForegroundColor White
Write-Host "  git push" -ForegroundColor White
Write-Host ""

# Optional: Show what would be committed
Write-Host "Checking Git status..." -ForegroundColor Cyan
if (Get-Command git -ErrorAction SilentlyContinue) {
    git status --short
    Write-Host ""
    
    # Show repository size
    Write-Host "Repository size check:" -ForegroundColor Cyan
    $objectCount = git count-objects -vH 2>$null
    if ($objectCount) {
        Write-Host $objectCount
    }
} else {
    Write-Host "Git not found. Install Git to use version control." -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Press any key to exit..."
$null = $Host.UI.RawUI.ReadKey('NoEcho,IncludeKeyDown')
