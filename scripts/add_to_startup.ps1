$sagerExe = "C:\Mein Projekts\Sager\target\release\sager.exe"
$startupDir = [Environment]::GetFolderPath("Startup")
$shortcutPath = Join-Path $startupDir "Sager.lnk"

$WshShell = New-Object -ComObject WScript.Shell
$shortcut = $WshShell.CreateShortcut($shortcutPath)
$shortcut.TargetPath = $sagerExe
$shortcut.WorkingDirectory = "C:\Mein Projekts\Sager"
$shortcut.Description = "Sager - Game Achievements Notifier & Cloud Save Backup Hub"
$shortcut.Save()

Write-Host "✅ Sager added to Windows Startup at: $shortcutPath"
