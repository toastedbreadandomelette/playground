use crate::utils::c64::C64;
use core::arch::x86_64::*;
use core::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

type F64x4 = __m256d;

#[derive(Debug, Clone, Copy)]
pub struct C64x2(F64x4);

impl Default for C64x2 {
    #[inline(always)]
    fn default() -> Self {
        C64x2::splat(C64::zero())
    }
}

impl C64x2 {
    #[inline(always)]
    pub fn from_slice(val: &[C64]) -> Self {
        unsafe { Self(_mm256_loadu_pd(val.as_ptr() as *const f64)) }
    }

    #[allow(unused)]
    #[inline(always)]
    pub fn unit(val: &[C64]) -> Self {
        unsafe { Self(_mm256_set_pd(0.0, 0.1, 0.0, 0.1)) }
    }

    #[inline(always)]
    pub fn splat(val: C64) -> Self {
        unsafe { Self(_mm256_set_pd(val.img, val.real, val.img, val.real)) }
    }

    #[inline(always)]
    pub fn as_array(&self) -> [C64; 2] {
        let mut vec = [C64::zero(); 2];
        unsafe { _mm256_storeu_pd(vec.as_mut_ptr() as *mut f64, self.0) };
        vec
    }

    #[allow(unused)]
    #[inline(always)]
    pub fn reduce_sum(&self) -> C64 {
        self.as_array().into_iter().fold(C64::zero(), |p, c| p + c)
    }

    #[allow(unused)]
    #[inline(always)]
    pub fn reduce_prod(&self) -> C64 {
        self.as_array().into_iter().fold(C64::unit(), |p, c| p * c)
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
    pub fn scalar_mul_vec(&self, val: C64x2) -> Self {
        unsafe { Self(_mm256_mul_pd(self.0, val.0)) }
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
        }
    }
}
