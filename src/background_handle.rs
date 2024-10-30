use std::ffi::CString;
use std::ptr::null_mut;
use vulkano::instance::Instance;
use winapi::shared::minwindef::HMODULE;
use winapi::shared::windef::{HWND, RECT};
use winapi::um::handleapi::CloseHandle;
use winapi::um::winnt::PROCESS_QUERY_INFORMATION;
use winapi::um::processthreadsapi::{OpenProcess, GetCurrentProcessId};
use winapi::um::libloaderapi::{GetModuleHandleExW, GetModuleHandleW, GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS};
use winapi::um::winuser::{EnumWindows, FindWindowA, FindWindowExA, GetDesktopWindow, GetWindowLongA, GetWindowLongPtrA, GetWindowRect, GetWindowThreadProcessId, SendMessageA};

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
pub fn get_background_handle() -> (HWND, HMODULE, (u32, u32)) {
    static mut WORKERW: HWND = null_mut();
    unsafe {
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
        let progman = {
            let window_name = CString::new("Progman").unwrap();
            FindWindowA(window_name.as_ptr(), null_mut())
        };
        SendMessageA(progman, 0x052C, 0, 0);
        SendMessageA(progman, 0x052C, 0, 1);
        EnumWindows(Some(enum_windows_proc), 0);
        let hinstance = get_module_handle_from_hwnd(WORKERW);


        let window_size = get_window_size(WORKERW).unwrap();



        (WORKERW, hinstance, (window_size.0 as u32, window_size.1 as u32))
    }
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