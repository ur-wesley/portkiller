$ErrorActionPreference = 'Stop'

. "$PSScriptRoot\append-user-path.ps1" -InstallDir 'C:\ignored-for-tests'

function Assert-Equal {
    param(
        [string]$Expected,
        [string]$Actual,
        [string]$Name
    )

    if ($Expected -ne $Actual) {
        throw "FAIL $Name`n  expected: $Expected`n  actual:   $Actual"
    }
}

Assert-Equal -Name 'empty path' -Expected 'C:\App' -Actual (Add-UserPathEntry -CurrentPath '' -Entry 'C:\App')
Assert-Equal -Name 'empty path trailing slash' -Expected 'C:\App' -Actual (Add-UserPathEntry -CurrentPath '' -Entry 'C:\App\')

Assert-Equal -Name 'already present' -Expected 'C:\A;C:\B' -Actual (Add-UserPathEntry -CurrentPath 'C:\A;C:\B' -Entry 'C:\B')
Assert-Equal -Name 'already present case insensitive' -Expected 'C:\A;c:\b' -Actual (Add-UserPathEntry -CurrentPath 'C:\A;c:\b' -Entry 'C:\B')

Assert-Equal -Name 'trailing semicolon' -Expected 'C:\A;C:\B' -Actual (Add-UserPathEntry -CurrentPath 'C:\A;' -Entry 'C:\B')
Assert-Equal -Name 'append entry' -Expected 'C:\A;C:\B' -Actual (Add-UserPathEntry -CurrentPath 'C:\A' -Entry 'C:\B')

$longBase = ('X' * 1200) + ';C:\Existing'
$longResult = Add-UserPathEntry -CurrentPath $longBase -Entry 'C:\NewEntry'
if ($longResult.Length -le $longBase.Length) {
    throw "FAIL long path append shortened result (base $($longBase.Length), got $($longResult.Length))"
}
if (-not $longResult.EndsWith('C:\NewEntry')) {
    throw "FAIL long path append missing entry"
}

Write-Output 'PASS append-user-path tests'
