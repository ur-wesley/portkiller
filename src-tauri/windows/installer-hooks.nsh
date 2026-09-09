!ifndef NSIS_MAX_STRLEN
  !define NSIS_MAX_STRLEN 1024
!endif

Function PortKillerStrStr
  Exch $1
  Exch
  Exch $0
  Push $2
  Push $3
  Push $4
  Push $5

  StrLen $4 $1
  StrLen $2 $0
  StrCpy $3 0
loop:
  IntCmp $3 $2 done_notfound
  StrCpy $5 $0 $4 $3
  StrCmp $5 $1 done_found
  IntOp $3 $3 + 1
  Goto loop
done_found:
  StrCpy $0 "1"
  Goto done
done_notfound:
  StrCpy $0 ""
done:
  Pop $5
  Pop $4
  Pop $3
  Pop $2
  Pop $1
  Exch $0
FunctionEnd

Function PortKillerQueryRegValueSize
  Exch $1
  Push $2
  Push $3
  Push $4

  StrCpy $0 0
  System::Call 'advapi32::RegOpenKey(i 0x80000001, t"Environment", *i.r2) i.r3'
  IntCmp $3 0 open_ok query_done query_done
open_ok:
  System::Call 'advapi32::RegQueryValueEx(i r2, t$1, i0, *i0, i0, *i r4) i.r3'
  IntCmp $3 0 size_ok query_close query_close
size_ok:
  StrCpy $0 $4
query_close:
  System::Call 'advapi32::RegCloseKey(i r2)'

query_done:
  Pop $4
  Pop $3
  Pop $2
  Pop $1
  Exch $0
FunctionEnd

Function PortKillerAppendToUserPath
  Push $0
  Push $1
  Push $2
  Push $3
  Push $4
  Push $5

  StrCpy $0 "$INSTDIR"

  ReadRegStr $1 HKCU "Environment" "Path"
  Push "Path"
  Call PortKillerQueryRegValueSize
  Pop $5

  StrCmp $1 "" 0 path_after_path_read
    IntCmp $5 2 path_try_upper_read path_try_upper_read path_overflow_skip

path_try_upper_read:
  ReadRegStr $1 HKCU "Environment" "PATH"
  Push "PATH"
  Call PortKillerQueryRegValueSize
  Pop $5
  StrCmp $1 "" 0 path_after_path_read
    IntCmp $5 2 path_build_new path_build_new path_overflow_skip

path_after_path_read:
  StrCmp $1 "" 0 path_check_present
  Goto path_build_new

path_overflow_skip:
  DetailPrint "PortKiller: skipping PATH update; existing user Path exceeds NSIS read limit"
  Goto path_done

path_check_present:
  StrCpy $2 "$1;"
  StrCpy $3 "$0;"
  Push $2
  Push $3
  Call PortKillerStrStr
  Pop $4
  StrCmp $4 "1" path_done 0 path_build_new

path_build_new:
  StrCmp $1 "" 0 path_append
    StrCpy $1 $0
    Goto path_write
path_append:
  StrCpy $2 $1 1 -1
  StrCmp $2 ";" 0 +2
    StrCpy $1 $1 -1
  StrCpy $1 "$1;$0"

path_write:
  StrLen $2 $1
  IntCmp $2 ${NSIS_MAX_STRLEN} path_do_write path_do_write path_too_long
path_too_long:
  DetailPrint "PortKiller: skipping PATH update; new Path length exceeds NSIS limit"
  Goto path_done
path_do_write:
  WriteRegExpandStr HKCU "Environment" "Path" "$1"
  DeleteRegValue HKCU "Environment" "PATH"
  System::Call 'user32::SendMessageTimeout(i 0xffff, i 0x001A, i 0, t "Environment", i 0x0002, i 5000, *i .r4)'

path_done:
  Pop $5
  Pop $4
  Pop $3
  Pop $2
  Pop $1
  Pop $0
FunctionEnd

!macro NSIS_HOOK_POSTINSTALL
  IfFileExists "$INSTDIR\PortKiller.exe" 0 +2
    CopyFiles "$INSTDIR\PortKiller.exe" "$INSTDIR\portkiller.exe"
  IfFileExists "$INSTDIR\portkiller-win.exe" 0 +2
    CopyFiles "$INSTDIR\portkiller-win.exe" "$INSTDIR\portkiller.exe"
  Call PortKillerAppendToUserPath
!macroend
