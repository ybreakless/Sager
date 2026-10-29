Set WshShell = CreateObject("WScript.Shell")
strCurDir = WshShell.CurrentDirectory
WshShell.Run """" & strCurDir & "\target\release\sager.exe""", 0, False
