<#
.SYNOPSIS
  Reset the current Git repo to the latest version on remote, discarding all local changes.

.DESCRIPTION
  - Fetches all remotes
  - Resets your current branch to match origin/<current-branch> exactly
  - Removes all untracked files and directories (use `-CleanIgnored` for ignored files too)

.PARAMETER Remote
  The name of the remote to reset against. Defaults to "origin".
#>

param(
    [string]$Remote = "origin"
)

# Ensure we're inside a Git repository
if (-not (Test-Path .git)) {
    Write-Error "This directory does not appear to be a Git repository."
    exit 1
}

# Get the name of the current branch
$branch = git rev-parse --abbrev-ref HEAD 2>$null
if ($LASTEXITCODE -ne 0 -or [string]::IsNullOrWhiteSpace($branch)) {
    Write-Error "Could not determine current Git branch."
    exit 1
}

Write-Host "🔄 Fetching from all remotes..."
git fetch --all

Write-Host "💥 Resetting branch '$branch' to '$Remote/$branch' (hard)..."
git reset --hard "$Remote/$branch"

Write-Host "🧹 Cleaning untracked files and directories..."
git clean -fd

Write-Host "✅ Repository is now identical to $Remote/$branch."
