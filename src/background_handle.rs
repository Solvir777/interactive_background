use std::ffi::CString;
use std::ptr::{null, null_mut};
use vulkano::instance::Instance;
use winapi::shared::minwindef::{HMODULE, LPDWORD};
use winapi::shared::windef::{HWND, RECT};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::handleapi::CloseHandle;
use winapi::um::winnt::{HANDLE, LPCSTR, PROCESS_QUERY_INFORMATION, PROCESS_TERMINATE};
use winapi::um::processthreadsapi::{OpenProcess, GetCurrentProcessId, TerminateProcess};
use winapi::um::libloaderapi::{GetModuleFileNameA, GetModuleHandleExA, GetModuleHandleExW, GetModuleHandleW, GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS, GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT};
use winapi::um::winuser::{DestroyWindow, EnumWindows, FindWindowA, FindWindowExA, GetDesktopWindow, GetWindowLongA, GetWindowLongPtrA, GetWindowRect, GetWindowThreadProcessId, PostMessageA, SendMessageA, ShowWindow, SW_HIDE, SW_RESTORE, SW_SHOW};

unsafe fn get_module_handle_from_hwnd(hwnd: HWND) -> HMODULE {
    let mut process_id = 0;
    GetWindowThreadProcessId(hwnd, &mut process_id);

    let process_handle = OpenProcess(PROCESS_QUERY_INFORMATION, 0, process_id);
    if process_handle.is_null() {
        return null_mut();
    }

    let mut module_handle: HMODULE = null_mut();
    GetModuleHandleExW(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS, process_handle as winapi::shared::ntdef::LPCWSTR, &mut module_handle);

    CloseHandle(process_handle);

    module_handle
}
fn find_progman() -> HWND {
    let window_name = CString::new("Progman").unwrap();
    unsafe {
        FindWindowA(window_name.as_ptr(), null_mut())
    }
}

unsafe fn kill_workerw(workerw: HWND) -> bool {
    let process_id: LPDWORD = &mut 0;

    GetWindowThreadProcessId(workerw, process_id);
    if (process_id.is_null()) {
        println!("no process found");
        return true;
    }

    let process = OpenProcess(PROCESS_TERMINATE, 0, *process_id);
    if (process.is_null()) {
        println!("OpenProcess failed, {}", GetLastError());
        return false;
    }

    if (TerminateProcess(process, 0) == 0) {
        println!("TerminateProcess failed, {}", GetLastError());
        return false;
    }

    get_background_handle();

    true
}
pub fn get_background_handle() -> (HWND, HMODULE, (u32, u32)) {
    unsafe {

        let progman = find_progman();
        let workerw = get_workerw(progman);
        println!("WorkerW hwnd: {:?}", workerw);

        let hinstance = get_module_handle_from_hwnd(workerw);

        let window_size = get_window_size(workerw).unwrap();


        (workerw, hinstance, (window_size.0 as u32, window_size.1 as u32))
    }
}

unsafe fn get_workerw(progman: HWND) -> HWND {
    static mut WORKERW: HWND = null_mut();
    unsafe extern "system" fn enum_windows_proc(tophandle: HWND, _: isize) -> i32 {
        let shell_dll_def_view = FindWindowExA(
            tophandle,
            null_mut(),
            "SHELLDLL_DefView\0".as_ptr() as *const i8,
            null_mut(),
        );

        if !shell_dll_def_view.is_null() {
            let workerw = FindWindowExA(
                null_mut(),
                tophandle,
                "WorkerW\0".as_ptr() as *const i8,
                null_mut(),
            );
            let _ = std::mem::replace(&mut WORKERW, workerw);
        }

        1
    }

    SendMessageA(progman, 0x052C, 0, 0);
    SendMessageA(progman, 0x052C, 0, 1);
    EnumWindows(Some(enum_windows_proc), 0);

    WORKERW
}

unsafe fn get_window_size(hwnd: HWND) -> Option<(i32, i32)> {
    let mut rect: RECT = std::mem::zeroed();
    if GetWindowRect(hwnd, &mut rect) == 0 {
        return None;
    }

    let width = rect.right - rect.left;
    let height = rect.bottom - rect.top;

    Some((width, height))
}

pub fn release_window(hwnd: HWND) {
    if(unsafe{kill_workerw(hwnd)}) {
        println!("WorkerW killed");
        return;
    };
    println!("WorkerW not killed");

}