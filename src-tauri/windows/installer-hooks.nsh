!macro NSIS_HOOK_POSTINSTALL
  IfFileExists "$INSTDIR\PortKiller.exe" 0 +2
    CopyFiles "$INSTDIR\PortKiller.exe" "$INSTDIR\portkiller.exe"
  IfFileExists "$INSTDIR\portkiller-win.exe" 0 +2
    CopyFiles "$INSTDIR\portkiller-win.exe" "$INSTDIR\portkiller.exe"
  ExecWait '"$SYSDIR\WindowsPowerShell\v1.0\powershell.exe" -NoProfile -ExecutionPolicy Bypass -File "$INSTDIR\resources\append-user-path.ps1" -InstallDir "$INSTDIR"'
!macroend
