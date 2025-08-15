#[cfg(target_arch = "x86_64")]
use std::arch::is_x86_feature_detected;
use simba::simd::{AutoF32x4, AutoU8x4, SimdComplexField, SimdPartialOrd, SimdValue};
use std::arch::x86_64::*;

#[inline(always)]
pub fn u8x4_to_f32x4(val: AutoU8x4) -> AutoF32x4 {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse4.1") {
            unsafe { return u8x4_to_f32x4_sse41(val) }
        }
    }
    AutoF32x4::from([val.0[0] as f32, val.0[1] as f32, val.0[2] as f32, val.0[3] as f32])
}

#[inline(always)]
pub fn f32x4_to_u32(val: AutoF32x4) -> u32 {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse2") {
            unsafe { return f32x4_to_u32_sse2(val) }
        }
    }

    let arr = val.simd_round().simd_clamp(AutoF32x4::splat(0f32), AutoF32x4::splat(255f32)).0;
    u32::from_be_bytes([arr[0] as u8, arr[1] as u8, arr[2] as u8, arr[3] as u8])
}

#[inline(always)]
pub fn u8x4_to_u32(val: AutoU8x4) -> u32 {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse2") {
            unsafe { return u8x4_to_u32_sse2(val) }
        }
    }

    u32::from_be_bytes([val.0[0], val.0[1], val.0[2], val.0[3]])
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse4.1")]
unsafe fn u8x4_to_f32x4_sse41(vals: AutoU8x4) -> AutoF32x4 {
    let packed: i32 = (vals.0[0] as i32)
        | ((vals.0[1] as i32) << 8)
        | ((vals.0[2] as i32) << 16)
        | ((vals.0[3] as i32) << 24);
    let x = _mm_cvtsi32_si128(packed);
    let i32s = _mm_cvtepu8_epi32(x);
    let ps = _mm_cvtepi32_ps(i32s);
    let mut out = [0f32; 4];
    unsafe { _mm_storeu_ps(out.as_mut_ptr(), ps) };
    AutoF32x4::from(out)
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
fn f32x4_to_u32_sse2(vals: AutoF32x4) -> u32 {
    let converted = vals.simd_round().simd_clamp(AutoF32x4::splat(0f32), AutoF32x4::splat(255f32)).0;
    let ps = _mm_setr_ps(converted[0], converted[1], converted[2], converted[3]);
    let i32s = _mm_cvtps_epi32(ps);
    let zero = _mm_setzero_si128();
    let packed16 = _mm_packs_epi32(i32s, zero);
    let packed8 = _mm_packus_epi16(packed16, zero);
    _mm_cvtsi128_si32(packed8) as u32
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
fn u8x4_to_u32_sse2(vals: AutoU8x4) -> u32 {
    let v = vals.0;
    let vec = _mm_setr_epi8(
        v[0] as i8, v[1] as i8, v[2] as i8, v[3] as i8,
        0,0,0,0,0,0,0,0,0,0,0,0,
    );

    _mm_cvtsi128_si32(vec) as u32
}