/* automatically generated by rust-bindgen 0.69.4 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CK_VERSION {
    pub major: u8,
    pub minor: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CK_SLOT_INFO {
    pub slotDescription: [u8; 64usize],
    pub manufacturerID: [u8; 32usize],
    pub flags: u32,
    pub hardwareVersion: CK_VERSION,
    pub firmwareVersion: CK_VERSION,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CK_TOKEN_INFO {
    pub label: [u8; 32usize],
    pub manufacturerID: [u8; 32usize],
    pub model: [u8; 16usize],
    pub serialNumber: [u8; 16usize],
    pub flags: u32,
    pub ulMaxSessionCount: u32,
    pub ulSessionCount: u32,
    pub ulMaxRwSessionCount: u32,
    pub ulRwSessionCount: u32,
    pub ulMaxPinLen: u32,
    pub ulMinPinLen: u32,
    pub ulTotalPublicMemory: u32,
    pub ulFreePublicMemory: u32,
    pub ulTotalPrivateMemory: u32,
    pub ulFreePrivateMemory: u32,
    pub hardwareVersion: CK_VERSION,
    pub firmwareVersion: CK_VERSION,
    pub utcTime: [u8; 16usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TByteArray {
    pub length: usize,
    pub data: *mut u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TSlotTokenInfo {
    pub slot_info: CK_SLOT_INFO,
    pub token_info: CK_TOKEN_INFO,
    pub valid: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TSlotTokenInfoArray {
    pub count: usize,
    pub slots_info: *mut TSlotTokenInfo,
}

extern "C" {
    pub fn perform_signing(
        input: TByteArray,
        userPIN: *mut ::std::os::raw::c_char,
        keyPairId: *mut ::std::os::raw::c_char,
    ) -> TByteArray;
}
extern "C" {
    pub fn get_slots_info() -> TSlotTokenInfoArray;
}