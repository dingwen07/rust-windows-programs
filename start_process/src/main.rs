#![windows_subsystem = "console"]

use windows::{
    Win32::Foundation::*, Win32::System::Threading::*,
    Win32::System::ProcessStatus::*,
    Win32::UI::Shell::ShellExecuteW,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut a_processes: [u32; 2048] = [0; 2048];
    let mut cb_needed: u32 = 0;
    let c_processes: u32;
    let mut flag = false;
    let mut found_pid: u32 = 0;
    
    unsafe {
        if !K32EnumProcesses(a_processes.as_mut_ptr(), 1000*4, &mut cb_needed).as_bool() {
            println!("EnumProcesses failed");
            return;
        }
    }

    if args.len() < 2 {
        println!("Usage: {} <executable path>", args[0]);
        return;
    }
    let mut path = args[1].clone();
    c_processes = cb_needed / 4;
    for i in 0..c_processes {
        if a_processes[i as usize] != 0 {
            let process_path = get_process_path(a_processes[i as usize]);
            if process_path.eq(&path) {
                flag = true;
                found_pid = a_processes[i as usize];
                break;
            }
        }
    }
    if !flag {
        println!("Process not found");
        unsafe {
            let mut v: Vec<u16> = path.encode_utf16().collect();
            v.push(0);
            let handle = ShellExecuteW(HWND::default(), PWSTR::default(), PWSTR(v.as_mut_ptr()), PWSTR::default(), PWSTR::default(), 0);
            let err = GetLastError();
            if err.0 != 0 {
                println!("ShellExecuteW failed: {:?}", err);
            } else {
                println!("Process started, handle: {:?}", handle);
            }
        }
        return;
    } else {
        println!("Process {}", found_pid);
    }


}

fn get_process_path (process_id: u32) -> String {
    let mut path: [u16; MAX_PATH as usize] = [0; MAX_PATH as usize];
    let mut cb_needed: u32 = 0;
    unsafe {
        let hprocess = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, process_id);
        if hprocess.0 == 0 {
            // let err = GetLastError();
            // println!("Process {}: Open Process failed with {:?}", process_id, err);
            return String::from("Error");
        }
        let mut hmod: HINSTANCE = HINSTANCE::default();

        if !K32EnumProcessModulesEx(hprocess, &mut hmod, 4, &mut cb_needed, LIST_MODULES_ALL).as_bool() {
            let err = GetLastError();
            println!("Process {}: Failed with {:?}", process_id, err);
            return String::from("Error");
        } else {
            let len = K32GetModuleFileNameExW(hprocess, hmod, PWSTR(path.as_mut_ptr()), MAX_PATH);
            let path = String::from_utf16_lossy(&path[..len as usize]);
            // println!("Process {}: {}", process_id, path);
            return path;
        }
    }
}

