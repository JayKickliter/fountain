#[cfg(target_arch = "x86")]
use std::arch::x86 as arch;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64 as arch;

// Computing the XOR of two byte slices, `lhs` & `rhs`.
// `lhs` is mutated in-place with the result
pub fn xor_bytes(lhs: &mut [u8], rhs: &[u8]) {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("avx2") {
            return unsafe { xor_bytes_avx2(lhs, rhs) };
        } else if is_x86_feature_detected!("sse2") {
            return unsafe { xor_bytes_sse2(lhs, rhs) };
        }
    }
    xor_bytes_fallback(lhs, rhs);
}

pub fn xor_bytes_fallback(lhs: &mut [u8], rhs: &[u8]) {
    for (l, r) in lhs.iter_mut().zip(rhs) {
        *l ^= *r;
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
pub unsafe fn xor_bytes_avx2(mut lhs: &mut [u8], mut rhs: &[u8]) {
    const STRIDE: usize = ::std::mem::size_of::<arch::__m256i>();
    while lhs.len() >= STRIDE && rhs.len() >= STRIDE {
        #[allow(clippy::cast_ptr_alignment)]
        let l = arch::_mm256_loadu_si256(lhs.as_ptr() as *const _);
        #[allow(clippy::cast_ptr_alignment)]
        let r = arch::_mm256_loadu_si256(rhs.as_ptr() as *const _);
        let res = arch::_mm256_xor_si256(l, r);
        #[allow(clippy::cast_ptr_alignment)]
        arch::_mm256_storeu_si256(lhs.as_mut_ptr() as *mut _, res);
        lhs = &mut lhs[STRIDE..];
        rhs = &rhs[STRIDE..];
    }
    // Use fallback for any remaining elements
    xor_bytes_fallback(lhs, rhs);
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "sse2")]
pub unsafe fn xor_bytes_sse2(mut lhs: &mut [u8], mut rhs: &[u8]) {
    const STRIDE: usize = ::std::mem::size_of::<arch::__m128i>();
    while lhs.len() >= STRIDE && rhs.len() >= STRIDE {
        #[allow(clippy::cast_ptr_alignment)]
        let l = arch::_mm_loadu_si128(lhs.as_ptr() as *const _);
        #[allow(clippy::cast_ptr_alignment)]
        let r = arch::_mm_loadu_si128(rhs.as_ptr() as *const _);
        let res = arch::_mm_xor_si128(l, r);
        #[allow(clippy::cast_ptr_alignment)]
        arch::_mm_storeu_si128(lhs.as_mut_ptr() as *mut _, res);
        lhs = &mut lhs[STRIDE..];
        rhs = &rhs[STRIDE..];
    }
    // Use fallback for any remaining elements
    xor_bytes_fallback(lhs, rhs);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[test]
    fn test_xor_bytes_avx2() {
        #[target_feature(enable = "avx2")]
        unsafe {
            if is_x86_feature_detected!("avx2") {
                let mut lhs_v: Vec<u8> = (0_u16..256).map(|e| e as u8).collect();
                let mut lhs_s = lhs_v.clone();
                let rhs = lhs_v.clone();
                xor_bytes_avx2(&mut lhs_v, &rhs);
                xor_bytes_fallback(&mut lhs_s, &rhs);
                assert_eq!(lhs_v, lhs_s);
            }
        }
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[test]
    fn test_xor_bytes_sse2() {
        #[target_feature(enable = "sse2")]
        unsafe {
            if is_x86_feature_detected!("sse2") {
                let mut lhs_v: Vec<u8> = (0_u16..256).map(|e| e as u8).collect();
                let mut lhs_s = lhs_v.clone();
                let rhs = lhs_v.clone();
                xor_bytes_sse2(&mut lhs_v, &rhs);
                xor_bytes_fallback(&mut lhs_s, &rhs);
                assert_eq!(lhs_v, lhs_s);
            }
        }
    }
}
