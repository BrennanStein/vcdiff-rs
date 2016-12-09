/* automatically generated by rust-bindgen */

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]
pub type ptrdiff_t = isize;
pub type size_t = usize;
pub type wchar_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __clang_max_align_nonce2: f64,
}
impl ::std::default::Default for max_align_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type int_least8_t = ::std::os::raw::c_char;
pub type int_least16_t = ::std::os::raw::c_short;
pub type int_least32_t = ::std::os::raw::c_int;
pub type int_least64_t = ::std::os::raw::c_long;
pub type uint_least8_t = ::std::os::raw::c_uchar;
pub type uint_least16_t = ::std::os::raw::c_ushort;
pub type uint_least32_t = ::std::os::raw::c_uint;
pub type uint_least64_t = ::std::os::raw::c_ulong;
pub type int_fast8_t = ::std::os::raw::c_char;
pub type int_fast16_t = ::std::os::raw::c_long;
pub type int_fast32_t = ::std::os::raw::c_long;
pub type int_fast64_t = ::std::os::raw::c_long;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_ulong;
pub type uint_fast32_t = ::std::os::raw::c_ulong;
pub type uint_fast64_t = ::std::os::raw::c_ulong;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type intmax_t = ::std::os::raw::c_long;
pub type uintmax_t = ::std::os::raw::c_ulong;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum VCDiffFormatExtensionFlagValues {
    VCD_STANDARD_FORMAT = 0,
    VCD_FORMAT_INTERLEAVED = 1,
    VCD_FORMAT_CHECKSUM = 2,
    VCD_FORMAT_JSON = 4,
}
pub type VCDiffFormatExtensionFlags = ::std::os::raw::c_int;
extern "C" {
    #[link_name = "vcdiff_encode"]
    pub fn encode(dictionary_data: *const uint8_t, dictionary_len: size_t,
                  target_data: *const uint8_t, target_len: size_t,
                  encoded_data: *mut *mut uint8_t, encoded_len: *mut size_t,
                  flags: VCDiffFormatExtensionFlags,
                  look_for_target_matches: u8);
    #[link_name = "vcdiff_decode"]
    pub fn decode(dictionary_data: *const uint8_t, dictionary_len: size_t,
                  encoded_data: *const uint8_t, encoded_len: size_t,
                  target_data: *mut *mut uint8_t, target_len: *mut size_t);
    #[link_name = "vcdiff_free_data"]
    pub fn free_data(data: *const uint8_t);
}
