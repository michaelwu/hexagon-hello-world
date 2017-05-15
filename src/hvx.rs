#![allow(dead_code, non_camel_case_types)]

#[repr(simd)]
#[derive(Copy, Clone)]
pub struct u32x4(u32, u32, u32, u32);

#[repr(simd)]
#[derive(Copy, Clone)]
pub struct u8x128(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8);
#[repr(simd)]
#[derive(Copy, Clone)]
pub struct i8x128(i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8);
#[repr(simd)]
#[derive(Copy, Clone)]
pub struct u16x64(u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16);
#[repr(simd)]
#[derive(Copy, Clone)]
pub struct i16x64(i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16);
#[repr(simd)]
#[derive(Copy, Clone)]
pub struct u32x32(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32);
#[repr(simd)]
#[derive(Copy, Clone)]
pub struct i32x32(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32);

#[repr(simd)]
#[derive(Copy, Clone)]
pub struct u8x256(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8);
#[repr(simd)]
#[derive(Copy, Clone)]
pub struct i8x256(i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8);
#[repr(simd)]
#[derive(Copy, Clone)]
pub struct u16x128(u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16);
#[repr(simd)]
#[derive(Copy, Clone)]
pub struct i16x128(i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16);
#[repr(simd)]
#[derive(Copy, Clone)]
pub struct u32x64(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32);
#[repr(simd)]
#[derive(Copy, Clone)]
pub struct i32x64(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32);

// Borrowed from the simd crate https://github.com/rust-lang-nursery/simd/
#[repr(packed)]
#[derive(Copy, Clone)]
struct Unalign<T>(T);

extern "platform-intrinsic" {
    pub fn simd_cast<T, U>(x: T) -> U;
}

macro_rules! basic_impls {
    ($(
        $name: ident:
        $elem: ident, $length: expr;
    )*) => {
        $(impl $name {
            #[inline]
            pub fn load(array: &[$elem], idx: usize) -> Self {
                let data = &array[idx..idx + $length];
                let loaded = unsafe {
                    *(data.as_ptr() as *const Unalign<Self>)
                };
                loaded.0
            }

            #[inline]
            pub fn store(self, array: &mut [$elem], idx: usize) {
                let place = &mut array[idx..idx + $length];
                unsafe {
                    *(place.as_mut_ptr() as *mut Unalign<Self>) = Unalign(self)
                }
            }
        })*
    }
}

basic_impls! {
    u32x4: u32, 4;

    u8x128: u8, 128;
    i8x128: i8, 128;
    u16x64: u16, 64;
    i16x64: i16, 64;
    u32x32: u32, 32;
    i32x32: i32, 32;

    u8x256: u8, 256;
    i8x256: i8, 256;
    u16x128: u16, 128;
    i16x128: i16, 128;
    u32x64: u32, 64;
    i32x64: i32, 64;
}

macro_rules! simd {
    ($($bool: ty: $($ty: ty = $elem: ty),*;)*) => {
        $($(unsafe impl Simd for $ty {
            type Bool = $bool;
            type Elem = $elem;
        }
            impl Clone for $ty { #[inline] fn clone(&self) -> Self { *self } }
            )*)*}
}

// End borrowing

extern "platform-intrinsic" {
    pub fn Q6_R_vextract128(x: u32x32, y: u32) -> u32;
    pub fn Q6_V_lo128(x: u32x64) -> u32x32;
    pub fn Q6_V_hi128(x: u32x64) -> u32x32;
    pub fn Q6_V_vsplat_R128(x: u32) -> u32x32;
    pub fn Q6_Q_and_QQ128(x: u32x4, y: u32x4) -> u32x4;
    pub fn Q6_Q_not_Q128(x: u32x4) -> u32x4;
    pub fn Q6_Q_or_QQ128(x: u32x4, y: u32x4) -> u32x4;
    pub fn Q6_Q_xor_QQ128(x: u32x4, y: u32x4) -> u32x4;
    pub fn Q6_Vub_vabsdiff_VubVub128(x: u8x128, y: u8x128) -> u8x128;
    pub fn Q6_Vuh_vabsdiff_VuhVuh128(x: u16x64, y: u16x64) -> u16x64;
    pub fn Q6_Vuh_vabsdiff_VhVh128(x: i16x64, y: i16x64) -> u16x64;
    pub fn Q6_Vuw_vabsdiff_VwVw128(x: i32x32, y: i32x32) -> u32x32;
    pub fn Q6_Vh_vabs_Vh128(x: i16x64) -> i16x64;
    pub fn Q6_Vw_vabs_Vw128(x: i32x32) -> i32x32;
    pub fn Q6_Vh_vabs_Vh_sat128(x: i16x64) -> i16x64;
    pub fn Q6_Vw_vabs_Vw_sat128(x: i32x32) -> i32x32;
    pub fn Q6_Vb_vadd_VbVb128(x: i8x128, y: i8x128) -> i8x128;
    pub fn Q6_Vh_vadd_VhVh128(x: i16x64, y: i16x64) -> i16x64;
    pub fn Q6_Vw_vadd_VwVw128(x: i32x32, y: i32x32) -> i32x32;
    pub fn Q6_Vh_vadd_VhVh_sat128(x: i16x64, y: i16x64) -> i16x64;
    pub fn Q6_Vw_vadd_VwVw_sat128(x: i32x32, y: i32x32) -> i32x32;
    pub fn Q6_Vub_vadd_VubVub_sat128(x: u8x128, y: u8x128) -> u8x128;
    pub fn Q6_Vuh_vadd_VuhVuh_sat128(x: u16x64, y: u16x64) -> u16x64;
    pub fn Q6_Wb_vadd_WbWb128(x: i8x256, y: i8x256) -> i8x256;
    pub fn Q6_Wh_vadd_WhWh128(x: i16x128, y: i16x128) -> i16x128;
    pub fn Q6_Ww_vadd_WwWw128(x: i32x64, y: i32x64) -> i32x64;
    pub fn Q6_Wh_vadd_WhWh_sat128(x: i16x128, y: i16x128) -> i16x128;
    pub fn Q6_Ww_vadd_WwWw_sat128(x: i32x64, y: i32x64) -> i32x64;
    pub fn Q6_Wub_vadd_WubWub_sat128(x: u8x256, y: u8x256) -> u8x256;
    pub fn Q6_Wuh_vadd_WuhWuh_sat128(x: u16x128, y: u16x128) -> u16x128;
    pub fn Q6_V_valign_VVR128(x: u8x128, y: u8x128, z: u32) -> u8x128;
    pub fn Q6_V_valign_VVI128(x: u8x128, y: u8x128, z: u32) -> u8x128;
    pub fn Q6_V_vlalign_VVR128(x: u8x128, y: u8x128, z: u32) -> u8x128;
    pub fn Q6_V_vlalign_VVI128(x: u8x128, y: u8x128, z: u32) -> u8x128;
    pub fn Q6_V_vand_VV128(x: u16x64, y: u16x64) -> u16x64;
    pub fn Q6_V_vand_QR128(x: u32x4, y: u32) -> u8x128;
    pub fn Q6_V_vandor_VQR128(x: u8x128, y: u32x4, z: u32) -> u8x128;
    pub fn Q6_Q_vand_VR128(x: u8x128, y: u32) -> u32x4;
    pub fn Q6_Q_vandor_QVR128(x: u32x4, y: u8x128, z: u32) -> u32x4;
    pub fn Q6_Vh_vasl_VhR128(x: i16x64, y: u32) -> i16x64;
    pub fn Q6_Vw_vasl_VwR128(x: i32x32, y: u32) -> i32x32;
    pub fn Q6_Vh_vasl_VhVh128(x: i16x64, y: i16x64) -> i16x64;
    pub fn Q6_Vw_vasl_VwVw128(x: i32x32, y: i32x32) -> i32x32;
    pub fn Q6_Vw_vaslacc_VwVwR128(x: i32x32, y: i32x32, z: u32) -> i32x32;
    pub fn Q6_Vh_vasr_VhR128(x: i16x64, y: u32) -> i16x64;
    pub fn Q6_Vw_vasr_VwR128(x: i32x32, y: u32) -> i32x32;
    pub fn Q6_Vh_vasr_VhVh128(x: i16x64, y: i16x64) -> i16x64;
    pub fn Q6_Vw_vasr_VwVw128(x: i32x32, y: i32x32) -> i32x32;
    pub fn Q6_Vw_vasracc_VwVwR128(x: i32x32, y: i32x32, z: u32) -> i32x32;
    pub fn Q6_Vh_vasr_VwVwR128(x: i32x32, y: i32x32, z: u32) -> i16x64;
    pub fn Q6_Vb_vasr_VhVhR_sat128(x: i16x64, y: i16x64, z: u32) -> i8x128;
    pub fn Q6_Vub_vasr_VhVhR_sat128(x: i16x64, y: i16x64, z: u32) -> u8x128;
    pub fn Q6_Vh_vasr_VwVwR_sat128(x: i32x32, y: i32x32, z: u32) -> i16x64;
    pub fn Q6_Vuh_vasr_VwVwR_sat128(x: i32x32, y: i32x32, z: u32) -> u16x64;
    pub fn Q6_Vb_vasr_VhVhR_rnd_sat128(x: i16x64, y: i16x64, z: u32) -> i8x128;
    pub fn Q6_Vub_vasr_VhVhR_rnd_sat128(x: i16x64, y: i16x64, z: u32) -> u8x128;
    pub fn Q6_Vh_vasr_VwVwR_rnd_sat128(x: i32x32, y: i32x32, z: u32) -> i16x64;
    pub fn Q6_Vuh_vasr_VwVwR_rnd_sat128(x: i32x32, y: i32x32, z: u32) -> u16x64;
    pub fn Q6_V_equals_V128(x: u32x32) -> u32x32;
    pub fn Q6_W_equals_W128(x: u32x64) -> u32x64;
    pub fn Q6_Vh_vavg_VhVh128(x: i16x64, y: i16x64) -> i16x64;
    pub fn Q6_Vw_vavg_VwVw128(x: i32x32, y: i32x32) -> i32x32;
    pub fn Q6_Vub_vavg_VubVub128(x: u8x128, y: u8x128) -> u8x128;
    pub fn Q6_Vuh_vavg_VuhVuh128(x: u16x64, y: u16x64) -> u16x64;
    pub fn Q6_Vh_vavg_VhVh_rnd128(x: i16x64, y: i16x64) -> i16x64;
    pub fn Q6_Vw_vavg_VwVw_rnd128(x: i32x32, y: i32x32) -> i32x32;
    pub fn Q6_Vub_vavg_VubVub_rnd128(x: u8x128, y: u8x128) -> u8x128;
    pub fn Q6_Vuh_vavg_VuhVuh_rnd128(x: u16x64, y: u16x64) -> u16x64;
    pub fn Q6_Vuh_vcl0_Vuh128(x: u16x64) -> u16x64;
    pub fn Q6_Vuw_vcl0_Vuw128(x: u32x32) -> u32x32;
    pub fn Q6_W_vcombine_VV128(x: u8x128, y: u8x128) -> u8x256;
    pub fn Q6_V_vzero128() -> u32x32;
    pub fn Q6_Vb_vdeal_Vb128(x: i8x128) -> i8x128;
    pub fn Q6_Vh_vdeal_Vh128(x: i16x64) -> i16x64;
    pub fn Q6_Vb_vdeale_VbVb128(x: i8x128, y: i8x128) -> i8x128;
    pub fn Q6_W_vdeal_VVR128(x: u8x128, y: u8x128, z: u32) -> u8x256;
    pub fn Q6_V_vdelta_VV128(x: u8x128, y: u8x128) -> u8x128;
    pub fn Q6_Vh_vdmpy_VubRb128(x: u8x128, y: u32) -> i16x64;
    pub fn Q6_Vh_vdmpyacc_VhVubRb128(x: i16x64, y: u8x128, z: u32) -> i16x64;
    pub fn Q6_Wh_vdmpy_WubRb128(x: u8x256, y: u32) -> i16x128;
    pub fn Q6_Wh_vdmpyacc_WhWubRb128(x: i16x128, y: u8x256, z: u32) -> i16x128;
    pub fn Q6_Vw_vdmpy_VhRb128(x: i16x64, y: u32) -> i32x32;
    pub fn Q6_Vw_vdmpyacc_VwVhRb128(x: i32x32, y: i16x64, z: u32) -> i32x32;
    pub fn Q6_Ww_vdmpy_WhRb128(x: i16x128, y: u32) -> i32x64;
    pub fn Q6_Ww_vdmpyacc_WwWhRb128(x: i32x64, y: i16x128, z: u32) -> i32x64;
    pub fn Q6_Vw_vdmpy_WwRh_sat128(x: i32x64, y: u32) -> i32x32;
    pub fn Q6_Vw_vdmpy_VhRh_sat128(x: i16x64, y: u32) -> i32x32;
    pub fn Q6_Vw_vdmpy_WhRuh_sat128(x: i16x128, y: u32) -> i32x32;
    pub fn Q6_Vw_vdmpy_VhRuh_sat128(x: i16x64, y: u32) -> i32x32;
    pub fn Q6_Vw_vdmpy_VhVh_sat128(x: i16x64, y: i16x64) -> i32x32;
    pub fn Q6_Vw_vdmpyacc_VwWwRh_sat128(x: i32x32, y: i32x64, z: u32) -> i32x32;
    pub fn Q6_Wuw_vdsad_WuhRuh128(x: u16x128, y: u32) -> u32x64;
    pub fn Q6_Wuw_vdsadacc_WuwWuhRuh128(x: u32x64, y: u16x128, z: u32) -> u32x64;
    pub fn Q6_Vw_vdmpyacc_VwVhRh_sat128(x: i32x32, y: i16x64, z: u32) -> i32x32;
    pub fn Q6_Vw_vdmpyacc_VwWhRuh_sat128(x: i32x32, y: i16x128, z: u32) -> i32x32;
    pub fn Q6_Vw_vdmpyacc_VwVhRuh_sat128(x: i32x32, y: i16x64, z: u32) -> i32x32;
    pub fn Q6_Vw_vdmpyacc_VwVhVh_sat128(x: i32x32, y: i16x64, z: i16x64) -> i32x32;
    pub fn Q6_Q_vcmp_eq_VbVb128(x: i8x128, y: i8x128) -> u32x4;
    pub fn Q6_Q_vcmp_eq_VhVh128(x: i16x64, y: i16x64) -> u32x4;
    pub fn Q6_Q_vcmp_eq_VwVw128(x: i32x32, y: i32x32) -> u32x4;
    pub fn Q6_Q_vcmp_eqand_QVbVb128(x: u32x4, y: i8x128, z: i8x128) -> u32x4;
    pub fn Q6_Q_vcmp_eqand_QVhVh128(x: u32x4, y: i16x64, z: i16x64) -> u32x4;
    pub fn Q6_Q_vcmp_eqand_QVwVw128(x: u32x4, y: i32x32, z: i32x32) -> u32x4;
    pub fn Q6_Q_vcmp_eqor_QVbVb128(x: u32x4, y: i8x128, z: i8x128) -> u32x4;
    pub fn Q6_Q_vcmp_eqor_QVhVh128(x: u32x4, y: i16x64, z: i16x64) -> u32x4;
    pub fn Q6_Q_vcmp_eqor_QVwVw128(x: u32x4, y: i32x32, z: i32x32) -> u32x4;
    pub fn Q6_Q_vcmp_eqxacc_QVbVb128(x: u32x4, y: i8x128, z: i8x128) -> u32x4;
    pub fn Q6_Q_vcmp_eqxacc_QVhVh128(x: u32x4, y: i16x64, z: i16x64) -> u32x4;
    pub fn Q6_Q_vcmp_eqxacc_QVwVw128(x: u32x4, y: i32x32, z: i32x32) -> u32x4;
    pub fn Q6_Q_vcmp_gt_VbVb128(x: i8x128, y: i8x128) -> u32x4;
    pub fn Q6_Q_vcmp_gt_VhVh128(x: i16x64, y: i16x64) -> u32x4;
    pub fn Q6_Q_vcmp_gt_VwVw128(x: i32x32, y: i32x32) -> u32x4;
    pub fn Q6_Q_vcmp_gt_VubVub128(x: u8x128, y: u8x128) -> u32x4;
    pub fn Q6_Q_vcmp_gt_VuhVuh128(x: u16x64, y: u16x64) -> u32x4;
    pub fn Q6_Q_vcmp_gtand_QVbVb128(x: u32x4, y: i8x128, z: i8x128) -> u32x4;
    pub fn Q6_Q_vcmp_gtand_QVhVh128(x: u32x4, y: i16x64, z: i16x64) -> u32x4;
    pub fn Q6_Q_vcmp_gtand_QVwVw128(x: u32x4, y: i32x32, z: i32x32) -> u32x4;
    pub fn Q6_Q_vcmp_gtand_QVubVub128(x: u32x4, y: u8x128, z: u8x128) -> u32x4;
    pub fn Q6_Q_vcmp_gtand_QVuhVuh128(x: u32x4, y: u16x64, z: u16x64) -> u32x4;
    pub fn Q6_Q_vcmp_gtor_QVbVb128(x: u32x4, y: i8x128, z: i8x128) -> u32x4;
    pub fn Q6_Q_vcmp_gtor_QVhVh128(x: u32x4, y: i16x64, z: i16x64) -> u32x4;
    pub fn Q6_Q_vcmp_gtor_QVwVw128(x: u32x4, y: i32x32, z: i32x32) -> u32x4;
    pub fn Q6_Q_vcmp_gtor_QVubVub128(x: u32x4, y: u8x128, z: u8x128) -> u32x4;
    pub fn Q6_Q_vcmp_gtor_QVuhVuh128(x: u32x4, y: u16x64, z: u16x64) -> u32x4;
    pub fn Q6_Q_vcmp_gtxacc_QVbVb128(x: u32x4, y: i8x128, z: i8x128) -> u32x4;
    pub fn Q6_Q_vcmp_gtxacc_QVhVh128(x: u32x4, y: i16x64, z: i16x64) -> u32x4;
    pub fn Q6_Q_vcmp_gtxacc_QVwVw128(x: u32x4, y: i32x32, z: i32x32) -> u32x4;
    pub fn Q6_Q_vcmp_gtxacc_QVubVub128(x: u32x4, y: u8x128, z: u8x128) -> u32x4;
    pub fn Q6_Q_vcmp_gtxacc_QVuhVuh128(x: u32x4, y: u16x64, z: u16x64) -> u32x4;
    pub fn Q6_Vw_vinsert_VwR128(x: i32) -> i32x32;
    pub fn Q6_Vuh_vlsr_VuhR128(x: u16x64, y: u32) -> u16x64;
    pub fn Q6_Vuw_vlsr_VuwR128(x: u32x32, y: u32) -> u32x32;
    pub fn Q6_Vh_vlsr_VhVh128(x: i16x64, y: i16x64) -> i16x64;
    pub fn Q6_Vw_vlsr_VwVw128(x: i32x32, y: i32x32) -> i32x32;
    pub fn Q6_Vb_vlut32_VbVbR128(x: i8x128, y: i8x128, z: u32) -> i8x128;
    pub fn Q6_Wh_vlut16_VbVhR128(x: i8x128, y: i16x64, z: u32) -> i16x128;
    pub fn Q6_Vb_vlut32or_VbVbVbR128(x: i8x128, y: i8x128, z: i8x128, w: u32) -> i8x128;
    pub fn Q6_Wh_vlut16or_WhVbVhR128(x: i16x128, y: i8x128, z: i16x64, w: u32) -> i16x128;
    pub fn Q6_Vh_vmax_VhVh128(x: i16x64, y: i16x64) -> i16x64;
    pub fn Q6_Vw_vmax_VwVw128(x: i32x32, y: i32x32) -> i32x32;
    pub fn Q6_Vub_vmax_VubVub128(x: u8x128, y: u8x128) -> u8x128;
    pub fn Q6_Vuh_vmax_VuhVuh128(x: u16x64, y: u16x64) -> u16x64;
    pub fn Q6_Vh_vmin_VhVh128(x: i16x64, y: i16x64) -> i16x64;
    pub fn Q6_Vw_vmin_VwVw128(x: i32x32, y: i32x32) -> i32x32;
    pub fn Q6_Vub_vmin_VubVub128(x: u8x128, y: u8x128) -> u8x128;
    pub fn Q6_Vuh_vmin_VuhVuh128(x: u16x64, y: u16x64) -> u16x64;
    pub fn Q6_Wh_vmpa_WubRb128(x: u8x256, y: u32) -> i16x128;
    pub fn Q6_Wh_vmpaacc_WhWubRb128(x: i16x128, y: u8x256, z: u32) -> i16x128;
    pub fn Q6_Wh_vmpa_WubWb128(x: u8x256, y: i8x256) -> i16x128;
    pub fn Q6_Wh_vmpa_WubWub128(x: u8x256, y: u8x256) -> i16x128;
    pub fn Q6_Ww_vmpa_WhRb128(x: i16x128, y: u32) -> i32x64;
    pub fn Q6_Ww_vmpaacc_WwWhRb128(x: i32x64, y: i16x128, z: u32) -> i32x64;
    pub fn Q6_Wh_vmpy_VbVub128(x: i8x128, y: u8x128) -> i16x128;
    pub fn Q6_Ww_vmpy_VhVuh128(x: i16x64, y: u16x64) -> i32x64;
    pub fn Q6_Wh_vmpyacc_WhVbVub128(x: i16x128, y: i8x128, z: u8x128) -> i16x128;
    pub fn Q6_Ww_vmpyacc_WwVhVuh128(x: i32x64, y: i16x64, z: u16x64) -> i32x64;
    pub fn Q6_Wh_vmpy_VubVb128(x: u8x128, y: i8x128) -> i16x128;
    pub fn Q6_Wh_vmpyacc_WhVubVb128(x: i16x128, y: u8x128, z: i8x128) -> i16x128;
    pub fn Q6_Wh_vmpy_VbVb128(x: i8x128, y: i8x128) -> i16x128;
    pub fn Q6_Wuh_vmpy_VubVub128(x: u8x128, y: u8x128) -> u16x128;
    pub fn Q6_Ww_vmpy_VhVh128(x: i16x64, y: i16x64) -> i32x64;
    pub fn Q6_Wuw_vmpy_VuhVuh128(x: u16x64, y: u16x64) -> u32x64;
    pub fn Q6_Wh_vmpyacc_WhVbVb128(x: i16x128, y: i8x128, z: i8x128) -> i16x128;
    pub fn Q6_Wuh_vmpyacc_WuhVubVub128(x: u16x128, y: u8x128, z: u8x128) -> u16x128;
    pub fn Q6_Ww_vmpyacc_WwVhVh128(x: i32x64, y: i16x64, z: i16x64) -> i32x64;
    pub fn Q6_Wuw_vmpyacc_WuwVuhVuh128(x: u32x64, y: u16x64, z: u16x64) -> u32x64;
    pub fn Q6_Vw_vmpye_VwVuh128(x: i32x32, y: u16x64) -> i32x32;
    pub fn Q6_Ww_vmpy_VhRh128(x: i16x64, y: u32) -> i32x64;
    pub fn Q6_Wuw_vmpy_VuhRuh128(x: u16x64, y: u32) -> u32x64;
    pub fn Q6_Ww_vmpyacc_WwVhRh_sat128(x: i32x64, y: i16x64, z: u32) -> i32x64;
    pub fn Q6_Vw_vmpy_VhRh_s1_rnd_sat128(x: i16x64, y: u32) -> i32x32;
    pub fn Q6_Vw_vmpy_VhRh_s1_sat128(x: i16x64, y: u32) -> i32x32;
    pub fn Q6_Vh_vmpy_VhVh_s1_rnd_sat128(x: i16x64, y: i16x64) -> i16x64;
    pub fn Q6_Vw_vmpyieo_VhVh128(x: i16x64, y: i16x64) -> i32x32;
    pub fn Q6_Vw_vmpyieacc_VwVwVh128(x: i32x32, y: i32x32, z: i16x64) -> i32x32;
    pub fn Q6_Vw_vmpyieacc_VwVwVuh128(x: i32x32, y: i32x32, z: u16x64) -> i32x32;
    pub fn Q6_Vw_vmpyie_VwVuh128(x: i32x32, y: u16x64) -> i32x32;
    pub fn Q6_Vh_vmpyi_VhVh128(x: i16x64, y: i16x64) -> i16x64;
    pub fn Q6_Vh_vmpyiacc_VhVhVh128(x: i16x64, y: i16x64, z: i16x64) -> i16x64;
    pub fn Q6_Vh_vmpyi_VhRb128(x: i16x64, y: u32) -> i16x64;
    pub fn Q6_Vw_vmpyi_VwRb128(x: i32x32, y: u32) -> i32x32;
    pub fn Q6_Vh_vmpyiacc_VhVhRb128(x: i16x64, y: i16x64, z: u32) -> i16x64;
    pub fn Q6_Vw_vmpyiacc_VwVwRb128(x: i32x32, y: i32x32, z: u32) -> i32x32;
    pub fn Q6_Vw_vmpyi_VwRh128(x: i32x32, y: u32) -> i32x32;
    pub fn Q6_Vw_vmpyiacc_VwVwRh128(x: i32x32, y: i32x32, z: u32) -> i32x32;
    pub fn Q6_Vw_vmpyi_VwRub128(x: i32x32, y: u32) -> i32x32;
    pub fn Q6_Vw_vmpyiacc_VwVwRub128(x: i32x32, y: i32x32, z: u32) -> i32x32;
    pub fn Q6_Vw_vmpyo_VwVh_s1_sat128(x: i32x32, y: i16x64) -> i32x32;
    pub fn Q6_Vw_vmpyo_VwVh_s1_rnd_sat128(x: i32x32, y: i16x64) -> i32x32;
    pub fn Q6_Vw_vmpyo_VwVh_s1_rnd_sat_shift128(x: i32x32, y: i16x64) -> i32x32;
    pub fn Q6_Vw_vmpyo_VwVh_s1_sat_shift128(x: i32x32, y: i16x64) -> i32x32;
    pub fn Q6_Vw_vmpyio_VwVh128(x: i32x32, y: i16x64) -> i32x32;
    pub fn Q6_Wuh_vmpy_VubRub128(x: u8x128, y: u32) -> u16x128;
    pub fn Q6_Wuh_vmpyacc_WuhVubRub128(x: u16x128, y: u8x128, z: u32) -> u16x128;
    pub fn Q6_Wuw_vmpyacc_WuwVuhRuh128(x: u32x64, y: u16x64, z: u32) -> u32x64;
    pub fn Q6_Vuw_vmux_QVV128(x: u32x4, y: u32x32, z: u32x32) -> u32x32;
    pub fn Q6_Vh_vnavg_VhVh128(x: i16x64, y: i16x64) -> i16x64;
    pub fn Q6_Vuh_vnavg_VuhVuh128(x: u16x64, y: u16x64) -> u16x64;
    pub fn Q6_Vw_vnavg_VwVw128(x: i32x32, y: i32x32) -> i32x32;
    pub fn Q6_Vuw_vnavg_VuwVuw128(x: u32x32, y: u32x32) -> u32x32;
    pub fn Q6_Vub_vnavg_VubVub128(x: u8x128, y: u8x128) -> u8x128;
    pub fn Q6_Vh_vnormamt_Vh128(x: i16x64) -> i16x64;
    pub fn Q6_Vw_vnormamt_Vw128(x: i32x32) -> i32x32;
    pub fn Q6_V_vnot_VV128(x: u16x64) -> u16x64;
    pub fn Q6_V_vor_VV128(x: u16x64, y: u16x64) -> u16x64;
    pub fn Q6_Vb_vpacke_VhVh128(x: i16x64, y: i16x64) -> i8x128;
    pub fn Q6_Vh_vpacke_VwVw128(x: i32x32, y: i32x32) -> i16x64;
    pub fn Q6_Vb_vpacko_VhVh128(x: i16x64, y: i16x64) -> i8x128;
    pub fn Q6_Vh_vpacko_VwVw128(x: i32x32, y: i32x32) -> i16x64;
    pub fn Q6_Vb_vpack_VhVh_sat128(x: i16x64, y: i16x64) -> i8x128;
    pub fn Q6_Vub_vpack_VhVh_sat128(x: i16x64, y: i16x64) -> u8x128;
    pub fn Q6_Vh_vpack_VwVw_sat128(x: i32x32, y: i32x32) -> i16x64;
    pub fn Q6_Vuh_vpack_VwVw_sat128(x: i32x32, y: i32x32) -> u16x64;
    pub fn Q6_Vh_vpopcount_Vh128(x: i16x64) -> i16x64;
    pub fn Q6_V_vrdelta_VV128(x: u8x128, y: u8x128) -> u8x128;
    pub fn Q6_Vw_vrmpy_VubRb128(x: u8x128, y: u32) -> i32x32;
    pub fn Q6_Vw_vrmpyacc_VwVubRb128(x: i32x32, y: u8x128, z: u32) -> i32x32;
    pub fn Q6_Ww_vrmpy_WubRbI128(x: u8x256, y: u32) -> i32x64;
    pub fn Q6_Ww_vrmpyacc_WwWubRbI128(x: i32x64, y: u8x256, z: u32) -> i32x64;
    pub fn Q6_Vw_vrmpy_VubVb128(x: u8x128, y: i8x128) -> i32x32;
    pub fn Q6_Vw_vrmpyacc_VwVubVb128(x: i32x32, y: u8x128, z: i8x128) -> i32x32;
    pub fn Q6_Vw_vrmpy_VbVb128(x: i8x128, y: i8x128) -> i32x32;
    pub fn Q6_Vuw_vrmpy_VubVub128(x: u8x128, y: u8x128) -> u32x32;
    pub fn Q6_Vw_vrmpyacc_VwVbVb128(x: i32x32, y: i8x128, z: i8x128) -> i32x32;
    pub fn Q6_Vuw_vrmpyacc_VuwVubVub128(x: u32x32, y: u8x128, z: u8x128) -> u32x32;
    pub fn Q6_Vuw_vrmpy_VubRub128(x: u8x128, y: u32) -> u32x32;
    pub fn Q6_Vuw_vrmpyacc_VuwVubRub128(x: u32x32, y: u8x128, z: u32) -> u32x32;
    pub fn Q6_Wuw_vrmpy_WubRubI128(x: u8x256, y: u32) -> u32x64;
    pub fn Q6_Wuw_vrmpyacc_WuwWubRubI128(x: u32x64, y: u8x256, z: u32) -> u32x64;
    pub fn Q6_V_vror_VR128(x: u8x128, y: u32) -> u8x128;
    pub fn Q6_Vb_vround_VhVh_sat128(x: i16x64, y: i16x64) -> i8x128;
    pub fn Q6_Vub_vround_VhVh_sat128(x: i16x64, y: i16x64) -> u8x128;
    pub fn Q6_Vh_vround_VwVw_sat128(x: i32x32, y: i32x32) -> i16x64;
    pub fn Q6_Vuh_vround_VwVw_sat128(x: i32x32, y: i32x32) -> u16x64;
    pub fn Q6_Wuw_vrsad_WubRubI128(x: u8x256, y: u32) -> u32x64;
    pub fn Q6_Wuw_vrsadacc_WuwWubRubI128(x: u32x64, y: u8x256, z: u32) -> u32x64;
    pub fn Q6_Vub_vsat_VhVh128(x: i16x64, y: i16x64) -> u8x128;
    pub fn Q6_Vh_vsat_VwVw128(x: i32x32, y: i32x32) -> i16x64;
    pub fn Q6_Wh_vsxt_Vb128(x: i8x128) -> i16x128;
    pub fn Q6_Ww_vsxt_Vh128(x: i16x64) -> i32x64;
    pub fn Q6_Wuh_vzxt_Vub128(x: u8x128) -> u16x128;
    pub fn Q6_Wuw_vzxt_Vuh128(x: u16x64) -> u32x64;
    pub fn Q6_Vb_condacc_QVbVb128(x: u32x4, y: i8x128, z: i8x128) -> i8x128;
    pub fn Q6_Vh_condacc_QVhVh128(x: u32x4, y: i16x64, z: i16x64) -> i16x64;
    pub fn Q6_Vw_condacc_QVwVw128(x: u32x4, y: i32x32, z: i32x32) -> i32x32;
    pub fn Q6_Vb_condacc_QnVbVb128(x: u32x4, y: i8x128, z: i8x128) -> i8x128;
    pub fn Q6_Vh_condacc_QnVhVh128(x: u32x4, y: i16x64, z: i16x64) -> i16x64;
    pub fn Q6_Vw_condacc_QnVwVw128(x: u32x4, y: i32x32, z: i32x32) -> i32x32;
    pub fn Q6_Vb_condnac_QVbVb128(x: u32x4, y: i8x128, z: i8x128) -> i8x128;
    pub fn Q6_Vh_condnac_QVhVh128(x: u32x4, y: i16x64, z: i16x64) -> i16x64;
    pub fn Q6_Vw_condnac_QVwVw128(x: u32x4, y: i32x32, z: i32x32) -> i32x32;
    pub fn Q6_Vb_condnac_QnVbVb128(x: u32x4, y: i8x128, z: i8x128) -> i8x128;
    pub fn Q6_Vh_condnac_QnVhVh128(x: u32x4, y: i16x64, z: i16x64) -> i16x64;
    pub fn Q6_Vw_condnac_QnVwVw128(x: u32x4, y: i32x32, z: i32x32) -> i32x32;
    pub fn Q6_Vh_vshuffe_VhVh128(x: i16x64, y: i16x64) -> i16x64;
    pub fn Q6_Vh_vshuffo_VhVh128(x: i16x64, y: i16x64) -> i16x64;
    pub fn Q6_Vb_vshuff_Vb128(x: i8x128) -> i8x128;
    pub fn Q6_Vh_vshuff_Vh128(x: i16x64) -> i16x64;
    pub fn Q6_Vb_vshuffe_VbVb128(x: i8x128, y: i8x128) -> i8x128;
    pub fn Q6_Vb_vshuffo_VbVb128(x: i8x128, y: i8x128) -> i8x128;
    pub fn Q6_Vb_vshuffoe_VbVb128(x: i8x128, y: i8x128) -> i8x128;
    pub fn Q6_Vh_vshuffoe_VhVh128(x: i16x64, y: i16x64) -> i16x64;
    pub fn Q6_W_vshuff_VVR128(x: u8x128, y: u8x128, z: u32) -> u8x256;
    pub fn Q6_Vb_vsub_VbVb128(x: i8x128, y: i8x128) -> i8x128;
    pub fn Q6_Vh_vsub_VhVh128(x: i16x64, y: i16x64) -> i16x64;
    pub fn Q6_Vw_vsub_VwVw128(x: i32x32, y: i32x32) -> i32x32;
    pub fn Q6_Vh_vsub_VhVh_sat128(x: i16x64, y: i16x64) -> i16x64;
    pub fn Q6_Vw_vsub_VwVw_sat128(x: i32x32, y: i32x32) -> i32x32;
    pub fn Q6_Vub_vsub_VubVub_sat128(x: u8x128, y: u8x128) -> u8x128;
    pub fn Q6_Vuh_vsub_VuhVuh_sat128(x: u16x64, y: u16x64) -> u16x64;
    pub fn Q6_Wb_vsub_WbWb128(x: i8x256, y: i8x256) -> i8x256;
    pub fn Q6_Wh_vsub_WhWh128(x: i16x128, y: i16x128) -> i16x128;
    pub fn Q6_Ww_vsub_WwWw128(x: i32x64, y: i32x64) -> i32x64;
    pub fn Q6_Wh_vsub_WhWh_sat128(x: i16x128, y: i16x128) -> i16x128;
    pub fn Q6_Ww_vsub_WwWw_sat128(x: i32x64, y: i32x64) -> i32x64;
    pub fn Q6_Wub_vsub_WubWub_sat128(x: u8x256, y: u8x256) -> u8x256;
    pub fn Q6_Wuh_vsub_WuhWuh_sat128(x: u16x128, y: u16x128) -> u16x128;
    pub fn Q6_W_vswap_QVV128(x: u32x4, y: u8x128, z: u8x128) -> u8x256;
    pub fn Q6_Wh_vtmpy_WbRb128(x: i8x256, y: u32) -> i16x128;
    pub fn Q6_Wh_vtmpyacc_WhWbRb128(x: i16x128, y: i8x256, z: u32) -> i16x128;
    pub fn Q6_Wh_vtmpy_WubRb128(x: u8x256, y: u32) -> i16x128;
    pub fn Q6_Wh_vtmpyacc_WhWubRb128(x: i16x128, y: u8x256, z: u32) -> i16x128;
    pub fn Q6_Ww_vtmpy_WhRb128(x: i16x128, y: u32) -> i32x64;
    pub fn Q6_Wh_vunpack_Vb128(x: i8x128) -> i16x128;
    pub fn Q6_Wuh_vunpack_Vub128(x: u8x128) -> u16x128;
    pub fn Q6_Ww_vunpack_Vh128(x: i16x64) -> i32x64;
    pub fn Q6_Wuw_vunpack_Vuh128(x: u16x64) -> u32x64;
    pub fn Q6_Wh_vunpackoor_WhVb128(x: i16x128, y: i8x128) -> i16x128;
    pub fn Q6_Ww_vunpackoor_WwVh128(x: i32x64, y: i16x64) -> i32x64;
    pub fn Q6_Ww_vtmpyacc_WwWhRb128(x: i32x64, y: i16x128, z: u32) -> i32x64;
    pub fn Q6_V_vxor_VV128(x: u16x64, y: u16x64) -> u16x64;
}
