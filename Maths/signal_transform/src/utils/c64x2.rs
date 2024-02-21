use crate::utils::c64::C64;
use core::arch::x86_64::*;
use core::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

type F64x4 = __m256d;

#[derive(Debug, Clone, Copy)]
pub struct C64x2(F64x4);

impl C64x2 {
    #[inline(always)]
    pub fn from_slice(val: &[C64]) -> Self {
        unsafe { Self(_mm256_loadu_pd(val.as_ptr() as *const f64)) }
    }

    #[inline(always)]
    pub fn splat(val: C64) -> Self {
        unsafe { Self(_mm256_set_pd(val.img, val.real, val.img, val.real)) }
    }

    #[inline(always)]
    pub fn from_array([first, second]: [C64; 2]) -> Self {
        unsafe {
            Self(_mm256_set_pd(
                second.img,
                second.real,
                first.img,
                first.real,
            ))
        }
    }

    #[inline(always)]
    pub fn scalar_mul(&self, val: f64) -> Self {
        unsafe { Self(_mm256_mul_pd(self.0, _mm256_set1_pd(val))) }
    }

    #[inline(always)]
    pub fn scalar_mul_vec(&self, val: C64x2) -> Self {
        unsafe { Self(_mm256_mul_pd(self.0, val.0)) }
    }

    #[inline(always)]
    pub fn scalar_mul_vec_self(&mut self, val: C64x2) {
        unsafe { self.0 = _mm256_mul_pd(self.0, val.0) }
    }

    #[inline(always)]
    pub fn copy_to_slice(&self, val: &mut [C64]) {
        unsafe { _mm256_storeu_pd(val.as_mut_ptr() as *mut f64, self.0) }
    }
}

impl Add for C64x2 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        unsafe { Self(_mm256_add_pd(self.0, rhs.0)) }
    }
}

impl AddAssign for C64x2 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        unsafe { self.0 = _mm256_add_pd(self.0, rhs.0) }
    }
}

impl Sub for C64x2 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        unsafe { Self(_mm256_sub_pd(self.0, rhs.0)) }
    }
}

impl SubAssign for C64x2 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        unsafe { self.0 = _mm256_sub_pd(self.0, rhs.0) }
    }
}

impl Mul for C64x2 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        unsafe {
            let real = _mm256_mul_pd(self.0, rhs.0);
            let img = _mm256_mul_pd(self.0, _mm256_permute_pd(rhs.0, 0b0101));
            let realf = _mm256_hsub_pd(real, real);
            let imgf = _mm256_hadd_pd(img, img);
            Self(_mm256_blend_pd(realf, imgf, 0b1010))
            // let (mut ans0, mut ans1) = ([C64::zero(); 2], [C64::zero(); 2]);
            // self.copy_to_slice(&mut ans0);
            // rhs.copy_to_slice(&mut ans1);
            // return C64x2::from_array([ans0[0] * ans1[0], ans1[1] * ans0[1]]);
        }
    }
}

impl MulAssign for C64x2 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        unsafe {
            let real = _mm256_mul_pd(self.0, rhs.0);
            let img = _mm256_mul_pd(self.0, _mm256_permute_pd(rhs.0, 0b0101));
            let realf = _mm256_hsub_pd(real, real);
            let imgf = _mm256_hadd_pd(img, img);
            *self = Self(_mm256_blend_pd(realf, imgf, 0b1010))
            // let (mut ans0, mut ans1) = ([C64::zero(); 2], [C64::zero(); 2]);
            // self.copy_to_slice(&mut ans0);
            // rhs.copy_to_slice(&mut ans1);
            // *self = C64x2::from_array([ans0[0] * ans1[0], ans1[1] * ans0[1]]);
        }
    }
}
