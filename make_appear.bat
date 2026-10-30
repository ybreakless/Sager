@echo off
rem Sager - Make Console Window & Companion UI Appear
powershell -NoProfile -Command "try { Invoke-RestMethod -Uri 'http://127.0.0.1:3939/api/system/make-appear' -Method Post -TimeoutSec 2 | Out-Null } catch {}; Start-Process 'http://127.0.0.1:3939'"
