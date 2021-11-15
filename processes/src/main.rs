use windows::{
    Win32::Foundation::*, Win32::System::Threading::*,
    Win32::System::ProcessStatus::*,
};

fn main() {
    let mut a_processes: [u32; 2048] = [0; 2048];
    let mut cb_needed: u32 = 0;
    let c_processes: u32;
    
    unsafe {
        if !K32EnumProcesses(a_processes.as_mut_ptr(), 1000*4, &mut cb_needed).as_bool() {
            println!("EnumProcesses failed");
            return;
        }
    }

    c_processes = cb_needed / 4;
    for i in 0..c_processes {
        if a_processes[i as usize] != 0 {
            // println!("Process {}: {}", i, a_processes[i as usize]);
            print_process_name_and_path(a_processes[i as usize]);
        }
    }


}

fn print_process_name_and_path (process_id: u32) {
    let mut path: [u16; MAX_PATH as usize] = [0; MAX_PATH as usize];
    let mut cb_needed: u32 = 0;
    unsafe {
        let hprocess = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, process_id);
        if hprocess.0 == 0 {
            let err = GetLastError();
            println!("Process {}: Open Process failed with {:?}", process_id, err);
            return;
        }
        let mut hmod: HINSTANCE = HINSTANCE::default();

        if !K32EnumProcessModulesEx(hprocess, &mut hmod, 4, &mut cb_needed, LIST_MODULES_ALL).as_bool() {
            let err = GetLastError();
            println!("Process {}: Failed with {:?}", process_id, err);
            return;
        } else {
            let len = K32GetModuleFileNameExW(hprocess, hmod, PWSTR(path.as_mut_ptr()), MAX_PATH);
            let path = String::from_utf16_lossy(&path[..len as usize]);
            println!("Process {}: {}", process_id, path);
        }
    }
}

