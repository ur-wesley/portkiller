param(
    [Parameter(Mandatory = $true)]
    [string]$InstallDir
)

function Add-UserPathEntry {
    param(
        [AllowNull()]
        [string]$CurrentPath,
        [Parameter(Mandatory = $true)]
        [string]$Entry
    )

    $normalizedEntry = $Entry.TrimEnd('\')
    if ([string]::IsNullOrWhiteSpace($normalizedEntry)) {
        return $CurrentPath
    }

    if ([string]::IsNullOrWhiteSpace($CurrentPath)) {
        return $normalizedEntry
    }

    $segments = $CurrentPath.Split(';', [System.StringSplitOptions]::RemoveEmptyEntries)
    foreach ($segment in $segments) {
        if ($segment.TrimEnd('\').Equals($normalizedEntry, [System.StringComparison]::OrdinalIgnoreCase)) {
            return $CurrentPath
        }
    }

    $trimmed = $CurrentPath.TrimEnd(';')
    return "$trimmed;$normalizedEntry"
}

if ($MyInvocation.InvocationName -ne '.') {
    $currentPath = [Environment]::GetEnvironmentVariable('Path', 'User')
    $newPath = Add-UserPathEntry -CurrentPath $currentPath -Entry $InstallDir

    if ($newPath -ne $currentPath) {
        [Environment]::SetEnvironmentVariable('Path', $newPath, 'User')
    }
}
