@echo off
rem Sager - Launch Silently in System Tray Background
if exist "%~dp0sager.exe" (
    start "" "%~dp0sager.exe"
) else if exist "%~dp0target\release\sager.exe" (
    start "" "%~dp0target\release\sager.exe"
) else (
    cargo run --release
)
