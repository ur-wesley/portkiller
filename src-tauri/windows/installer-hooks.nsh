!macro NSIS_HOOK_POSTINSTALL
  IfFileExists "$INSTDIR\PortKiller.exe" 0 +2
    CopyFiles "$INSTDIR\PortKiller.exe" "$INSTDIR\portkiller.exe"
  IfFileExists "$INSTDIR\portkiller-win.exe" 0 +2
    CopyFiles "$INSTDIR\portkiller-win.exe" "$INSTDIR\portkiller.exe"
  ReadRegStr $0 HKCU "Environment" "Path"
  StrCmp $0 "" 0 +3
    ReadRegStr $0 HKCU "Environment" "PATH"
  StrCmp $0 "" 0 +2
    StrCpy $0 ""
  StrCpy $1 "$INSTDIR"
  StrCpy $2 "$0;$1"
  WriteRegExpandStr HKCU "Environment" "Path" "$2"
  DeleteRegValue HKCU "Environment" "PATH"
  System::Call 'user32::SendMessageTimeout(i 0xffff, i 0x001A, i 0, t "Environment", i 0x0002, i 5000, *i .r0)'
!macroend
