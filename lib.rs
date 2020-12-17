#![allow(dead_code)]

use winapi::um::winnt::{ LPCWSTR };
use winapi::shared::minwindef::{ LPVOID };

#[repr(C)]
pub enum MhStatus {
    Unknown = -1,
    Ok,
    ErrorAlreadyInitialized,
    ErrorNotInitialized,
    ErrorAlreadyCreated,
    ErrorNotCreated,
    ErrorEnabled,
    ErrorDisabled,
    ErrorNotExecutable,
    ErrorUnsupportedFunction,
    ErrorMemoryAlloc,
    ErrorMemoryProtect,
    ErrorModuleNotFound,
    ErrorFunctionNotFound
}

#[link(name = "minhook.x64d", kind = "static")]
extern "C" {
    pub fn MH_Initialize() -> MhStatus;

    pub fn MH_Uninitialize() -> MhStatus;

    pub fn MH_CreateHook(p_target: LPVOID, p_detour: LPVOID, pp_original: &LPVOID) -> MhStatus;

    pub fn MH_CreateHookApi(psz_module: LPCWSTR, psz_proc_name: LPCWSTR, p_detour: LPVOID, pp_original: &LPVOID) -> MhStatus;

    pub fn MH_CreateHookApiEx(psz_module: LPCWSTR, psz_proc_name: LPCWSTR, p_detour: LPVOID, pp_original: &LPVOID, pp_target: &LPVOID) -> MhStatus;

    pub fn MH_RemoveHook(p_target: LPVOID) -> MhStatus;

    pub fn MH_EnableHook(p_target: LPVOID) -> MhStatus;

    pub fn MH_DisableHook(p_target: LPVOID) -> MhStatus;

    pub fn MH_QueueEnableHook(p_target: LPVOID) -> MhStatus;

    pub fn MH_QueueDisableHook(p_target: LPVOID) -> MhStatus;

    pub fn MH_ApplyQueued() -> MhStatus;

    pub fn MH_StatusToString(status: MhStatus) -> *const cty::c_char;
}

