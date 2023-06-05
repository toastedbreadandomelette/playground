#include <immintrin.h>
#include <omp.h>

#include <chrono>
#include <iostream>

// How to run:
// g++ -march=native linalg/simd_matmul.cpp -O3 -mfma

void *amalloc(size_t size, size_t alignment) {
#ifdef _WIN32
    return _aligned_malloc(size, alignment);
#else
    return aligned_alloc(alignment, size);
#endif
}

void afree(void *ptr) {
#ifdef _WIN32
    _aligned_free(ptr);
#else
    free(ptr);
#endif
}

// Naive multiplication
double *mul(const double *a, const double *b, int m, int n, int p) {
    double *c = static_cast<double *>(malloc(m * p * sizeof(double)));
    for (int i = 0; i < m; ++i) {
        for (int j = 0; j < p; ++j) {
            c[i * m + j] = 0;
            for (int k = 0; k < n; ++k) {
                c[i * p + j] += a[i * n + k] * b[k * p + j];
            }
        }
    }
    return c;
}

// cache-friendly matrix multiplication
double *cf_mul(const double *a, const double *b, int m, int n, int p) {
    double *c = static_cast<double *>(malloc(m * p * sizeof(double)));
    for (int i = 0; i < m; ++i) {
        for (int k = 0; k < p; ++k) c[i * p + k] = 0;
    }
    for (int i = 0; i < m; ++i) {
        for (int k = 0; k < n; ++k) {
            for (int j = 0; j < p; ++j) {
                c[i * p + j] += a[i * n + k] * b[k * p + j];
            }
        }
    }
    return c;
}

// cache-friendly blocked matrix multiplication
double *cf_block_mul(const double *a, const double *b, int m, int n, int p) {
    double *c = static_cast<double *>(malloc(m * p * sizeof(double)));
    for (int i = 0; i < m; ++i) {
        for (int k = 0; k < p; ++k) c[i * p + k] = 0;
    }
    const int bsize = 32;
    for (int ii = 0; ii < m; ii += bsize) {
        const int il = ii + bsize > m ? m : ii + bsize;
        for (int kk = 0; kk < n; kk += bsize) {
            const int kl = kk + bsize > m ? m : kk + bsize;
            for (int i = ii; i < il; ++i) {
                for (int k = kk; k < kl; ++k) {
                    for (int j = 0; j < p; ++j) {
                        c[i * p + j] += a[i * n + k] * b[k * p + j];
                    }
                }
            }
        }
    }
    return c;
}

// Naive aligned mem multiplication
double *mul_al(const double *a, const double *b, int m, int n, int p) {
    double *c =
        static_cast<double *>(amalloc(m * p * sizeof(double), 64));
    for (int i = 0; i < m; ++i) {
        for (int j = 0; j < p; ++j) {
            double s = 0;
            for (int k = 0; k < n; ++k) {
                s += a[i * n + k] * b[k * p + j];
            }
            c[i * p + j] = s;
        }
    }
    return c;
}

// cache-friendly aligned mem matrix multiplication
double *cf_mul_al(const double *a, const double *b, int m, int n, int p) {
    double *c =
        static_cast<double *>(amalloc(m * p * sizeof(double), 64));
    for (int i = 0; i < m; ++i) {
        for (int k = 0; k < p; ++k) c[i * m + k] = 0;
    }
    for (int i = 0; i < m; ++i) {
        for (int k = 0; k < n; ++k) {
            for (int j = 0; j < p; ++j) {
                c[i * p + j] += a[i * n + k] * b[k * p + j];
            }
        }
    }
    return c;
}

// cache-friendly aligned mem blocked matrix multiplication
double *cf_block_mul_al(const double *a, const double *b, int m, int n, int p) {
    double *c =
        static_cast<double *>(amalloc(m * p * sizeof(double), 64));
    for (int i = 0; i < m; ++i) {
        for (int k = 0; k < p; ++k) c[i * p + k] = 0;
    }
    const int bsize = 32;
    for (int ii = 0; ii < m; ii += bsize) {
        const int il = ii + bsize > m ? m : ii + bsize;
        for (int kk = 0; kk < n; kk += bsize) {
            const int kl = kk + bsize > m ? m : kk + bsize;
            for (int i = ii; i < il; ++i) {
                for (int k = kk; k < kl; ++k) {
                    for (int j = 0; j < p; ++j) {
                        c[i * p + j] += a[i * n + k] * b[k * p + j];
                    }
                }
            }
        }
    }
    return c;
}

// cache-friendly aligned mem simd-blocked matrix multiplication
double *cf_block_mul_al_tr(const double *a, const double *b, int m, int n,
                           int p) {
    double *c =
        static_cast<double *>(amalloc(m * p * sizeof(double), 64));
    double *d =
        static_cast<double *>(amalloc(n * p * sizeof(double), 64));

    for (int i = 0; i < n; ++i) {
        for (int j = 0; j < p; ++j) {
            d[j * n + i] = b[i * p + j];
        }
    }
    for (int i = 0; i < m; ++i) {
        for (int j = 0; j < p; ++j) {
            c[i * p + j] = 0;
        }
    }
    const int bsize = 128;
    for (int ii = 0; ii < m; ii += bsize) {
        const int il = ii + bsize > m ? m : ii + bsize;
        for (int jj = 0; jj < p; jj += bsize) {
            const int jl = jj + bsize > p ? p : jj + bsize;
            for (int i = ii; i < il; ++i) {
                for (int j = jj; j < jl; ++j) {
                    double ans = 0;
                    for (int k = 0; k < n; ++k) {
                        ans += a[i * n + k] * d[j * n + k];
                    }
                    c[i * p + j] = ans;
                }
            }
        }
    }
    afree(d);
    return c;
}

// cache-friendly simd matrix multiplication
double *cf_simd_mul(const double *a, const double *b, int m, int n, int p) {
    double *c = static_cast<double *>(malloc(m * p * sizeof(double)));
    for (int i = 0; i < m; ++i) {
        for (int j = 0; j < p; j += 4) {
            _mm256_storeu_pd((c + (i * p + j)), _mm256_setzero_pd());
        }
    }
    for (int i = 0; i < m; ++i) {
        for (int k = 0; k < n; ++k) {
            auto val = a[i * n + k];
            __m256d av = _mm256_set1_pd(val);
            for (int j = 0; j < p; j += 4) {
                __m256d cv = _mm256_loadu_pd(c + (i * p + j));
                __m256d bv = _mm256_loadu_pd(b + (k * p + j));
                __m256d res = _mm256_fmadd_pd(av, bv, cv);
                _mm256_storeu_pd(c + (i * p + j), res);
            }
        }
    }
    return c;
}

// cache-friendly blocked simd matrix multiplication
double *cf_simd_block_mul(const double *a, const double *b, int m, int n,
                          int p) {
    double *c = static_cast<double *>(malloc(m * p * sizeof(double)));
    for (int i = 0; i < m; ++i) {
        for (int j = 0; j < p; j += 4) {
            _mm256_storeu_pd((c + (i * p + j)), _mm256_setzero_pd());
        }
    }
    const int bsize = 32;
    for (int ii = 0; ii < m; ii += bsize) {
        const int il = ii + bsize > m ? m : ii + bsize;
        for (int kk = 0; kk < n; kk += bsize) {
            const int kl = kk + bsize > m ? m : kk + bsize;
            for (int i = ii; i < il; ++i) {
                for (int k = kk; k < kl; ++k) {
                    auto val = a[i * n + k];
                    __m256d av = _mm256_set1_pd(val);
                    for (int j = 0; j < p; j += 4) {
                        __m256d cv = _mm256_loadu_pd(c + (i * p + j));
                        __m256d bv = _mm256_loadu_pd(b + (k * p + j));
                        __m256d res = _mm256_fmadd_pd(av, bv, cv);
                        _mm256_storeu_pd(c + (i * p + j), res);
                    }
                }
            }
        }
    }
    return c;
}

// cache-friendly aligned mem simd matrix multiplication
double *cf_simd_mul_al(const double *a, const double *b, int m, int n, int p) {
    double *c =
        static_cast<double *>(amalloc(m * p * sizeof(double), 64));
    for (int i = 0; i < m; ++i) {
        for (int j = 0; j < p; j += 4) {
            _mm256_store_pd((c + (i * p + j)), _mm256_setzero_pd());
        }
    }
    for (int i = 0; i < m; ++i) {
        for (int k = 0; k < n; ++k) {
            auto val = a[i * n + k];
            __m256d av = _mm256_set1_pd(val);
            for (int j = 0; j < p; j += 4) {
                __m256d cv = _mm256_load_pd(c + (i * p + j));
                __m256d bv = _mm256_load_pd(b + (k * p + j));
                _mm256_store_pd(c + (i * p + j), _mm256_fmadd_pd(av, bv, cv));
            }
        }
    }
    return c;
}

// cache-friendly aligned mem simd-blocked matrix multiplication
double *cf_simd_block_mul_al(const double *a, const double *b, int m, int n,
                             int p) {
    double *c =
        static_cast<double *>(amalloc(m * p * sizeof(double), 64));
    for (int i = 0; i < m; ++i) {
        for (int j = 0; j < p; j += 4) {
            _mm256_store_pd((c + (i * p + j)), _mm256_setzero_pd());
        }
    }
    const int bsize = 32;
    for (int ii = 0; ii < m; ii += bsize) {
        const int il = ii + bsize > m ? m : ii + bsize;
        for (int kk = 0; kk < n; kk += bsize) {
            const int kl = kk + bsize > n ? n : kk + bsize;
            for (int i = ii; i < il; ++i) {
                for (int k = kk; k < kl; ++k) {
                    auto val = a[i * n + k];
                    __m256d av = _mm256_set1_pd(val);
                    for (int j = 0; j < p; j += 4) {
                        __m256d cv = _mm256_load_pd(c + (i * p + j));
                        __m256d bv = _mm256_load_pd(b + (k * p + j));
                        _mm256_store_pd(c + (i * p + j),
                                        _mm256_fmadd_pd(av, bv, cv));
                    }
                }
            }
        }
    }
    return c;
}

// cache-friendly aligned mem simd-blocked matrix multiplication
// with transpose (dot-product results are taken in a width)
// to do: perform SIMD in parallel for the results
//
// Not a perfect algorithm for gemm, but a good start
double *cf_block_simd_mul_al_tr(const double *a, const double *b, int m, int n,
                                int p) {
    double *c =
        static_cast<double *>(amalloc(m * p * sizeof(double), 64));
    double *d =
        static_cast<double *>(amalloc(n * p * sizeof(double), 64));

    for (int i = 0; i < n; ++i) {
        for (int j = 0; j < p; ++j) {
            d[j * n + i] = b[i * p + j];
        }
    }
    for (int i = 0; i < m; ++i) {
        for (int j = 0; j < p; ++j) {
            c[i * p + j] = 0;
        }
    }

    const int ibsize = 128;
    const int jbsize = 256;
    const int kbsize = n;

    for (int ii = 0; ii < m; ii += ibsize) {
        const int il = ii + ibsize > m ? m : ii + ibsize;
        for (int jj = 0; jj < p; jj += jbsize) {
            const int jl = jj + jbsize > p ? p : jj + jbsize;
            
            for (int kk = 0; kk < n; kk += kbsize) {
                const int kl = kk + kbsize > n ? n : kk + kbsize;
                // Perform on 4 values in a row
                for (int i = ii; i < il; i += 4) {
                    // Perform on 4 values in a column
                    for (int j = jj; j < jl; j += 4) {
                        __m256d cvec_result00 = _mm256_setzero_pd();
                        __m256d cvec_result01 = _mm256_setzero_pd();
                        __m256d cvec_result02 = _mm256_setzero_pd();
                        __m256d cvec_result03 = _mm256_setzero_pd();

                        __m256d cvec_result10 = _mm256_setzero_pd();
                        __m256d cvec_result11 = _mm256_setzero_pd();
                        __m256d cvec_result12 = _mm256_setzero_pd();
                        __m256d cvec_result13 = _mm256_setzero_pd();

                        __m256d cvec_result20 = _mm256_setzero_pd();
                        __m256d cvec_result21 = _mm256_setzero_pd();
                        __m256d cvec_result22 = _mm256_setzero_pd();
                        __m256d cvec_result23 = _mm256_setzero_pd();

                        __m256d cvec_result30 = _mm256_setzero_pd();
                        __m256d cvec_result31 = _mm256_setzero_pd();
                        __m256d cvec_result32 = _mm256_setzero_pd();
                        __m256d cvec_result33 = _mm256_setzero_pd();

                        for (int k = kk; k < kl; k += 16) {
                            __m256d av0 = _mm256_load_pd(a + (i * n + k));

                            __m256d bv0 = _mm256_load_pd(d + (j * n + k));
                            __m256d bv1 = _mm256_load_pd(d + ((j + 1) * n + k));
                            __m256d bv2 = _mm256_load_pd(d + ((j + 2) * n + k));
                            __m256d bv3 = _mm256_load_pd(d + ((j + 3) * n + k));
                            cvec_result00 =
                                _mm256_fmadd_pd(av0, bv0, cvec_result00);
                            cvec_result01 =
                                _mm256_fmadd_pd(av0, bv1, cvec_result01);
                            cvec_result02 =
                                _mm256_fmadd_pd(av0, bv2, cvec_result02);
                            cvec_result03 =
                                _mm256_fmadd_pd(av0, bv3, cvec_result03);

                            __m256d av1 = _mm256_load_pd(a + ((i + 1) * n + k));
                            cvec_result10 =
                                _mm256_fmadd_pd(av1, bv0, cvec_result10);
                            cvec_result11 =
                                _mm256_fmadd_pd(av1, bv1, cvec_result11);
                            cvec_result12 =
                                _mm256_fmadd_pd(av1, bv2, cvec_result12);
                            cvec_result13 =
                                _mm256_fmadd_pd(av1, bv3, cvec_result13);

                            __m256d av2 = _mm256_load_pd(a + ((i + 2) * n + k));
                            cvec_result20 =
                                _mm256_fmadd_pd(av2, bv0, cvec_result20);
                            cvec_result21 =
                                _mm256_fmadd_pd(av2, bv1, cvec_result21);
                            cvec_result22 =
                                _mm256_fmadd_pd(av2, bv2, cvec_result22);
                            cvec_result23 =
                                _mm256_fmadd_pd(av2, bv3, cvec_result23);

                            __m256d av3 = _mm256_load_pd(a + ((i + 3) * n + k));
                            cvec_result30 =
                                _mm256_fmadd_pd(av3, bv0, cvec_result30);
                            cvec_result31 =
                                _mm256_fmadd_pd(av3, bv1, cvec_result31);
                            cvec_result32 =
                                _mm256_fmadd_pd(av3, bv2, cvec_result32);
                            cvec_result33 =
                                _mm256_fmadd_pd(av3, bv3, cvec_result33);

                            ////////////////////////////////////////////////////////
                            av0 = _mm256_load_pd(a + (i * n + k + 4));

                            bv0 = _mm256_load_pd(d + (j * n + k + 4));
                            bv1 = _mm256_load_pd(d + ((j + 1) * n + k + 4));
                            bv2 = _mm256_load_pd(d + ((j + 2) * n + k + 4));
                            bv3 = _mm256_load_pd(d + ((j + 3) * n + k + 4));
                            cvec_result00 =
                                _mm256_fmadd_pd(av0, bv0, cvec_result00);
                            cvec_result01 =
                                _mm256_fmadd_pd(av0, bv1, cvec_result01);
                            cvec_result02 =
                                _mm256_fmadd_pd(av0, bv2, cvec_result02);
                            cvec_result03 =
                                _mm256_fmadd_pd(av0, bv3, cvec_result03);

                            av1 = _mm256_load_pd(a + ((i + 1) * n + k + 4));
                            cvec_result10 =
                                _mm256_fmadd_pd(av1, bv0, cvec_result10);
                            cvec_result11 =
                                _mm256_fmadd_pd(av1, bv1, cvec_result11);
                            cvec_result12 =
                                _mm256_fmadd_pd(av1, bv2, cvec_result12);
                            cvec_result13 =
                                _mm256_fmadd_pd(av1, bv3, cvec_result13);

                            av2 = _mm256_load_pd(a + ((i + 2) * n + k + 4));
                            cvec_result20 =
                                _mm256_fmadd_pd(av2, bv0, cvec_result20);
                            cvec_result21 =
                                _mm256_fmadd_pd(av2, bv1, cvec_result21);
                            cvec_result22 =
                                _mm256_fmadd_pd(av2, bv2, cvec_result22);
                            cvec_result23 =
                                _mm256_fmadd_pd(av2, bv3, cvec_result23);

                            av3 = _mm256_load_pd(a + ((i + 3) * n + k + 4));
                            cvec_result30 =
                                _mm256_fmadd_pd(av3, bv0, cvec_result30);
                            cvec_result31 =
                                _mm256_fmadd_pd(av3, bv1, cvec_result31);
                            cvec_result32 =
                                _mm256_fmadd_pd(av3, bv2, cvec_result32);
                            cvec_result33 =
                                _mm256_fmadd_pd(av3, bv3, cvec_result33);

                            ////////////////////////////////////////////////////////
                            av0 = _mm256_load_pd(a + (i * n + k + 8));

                            bv0 = _mm256_load_pd(d + (j * n + k + 8));
                            bv1 = _mm256_load_pd(d + ((j + 1) * n + k + 8));
                            bv2 = _mm256_load_pd(d + ((j + 2) * n + k + 8));
                            bv3 = _mm256_load_pd(d + ((j + 3) * n + k + 8));
                            cvec_result00 =
                                _mm256_fmadd_pd(av0, bv0, cvec_result00);
                            cvec_result01 =
                                _mm256_fmadd_pd(av0, bv1, cvec_result01);
                            cvec_result02 =
                                _mm256_fmadd_pd(av0, bv2, cvec_result02);
                            cvec_result03 =
                                _mm256_fmadd_pd(av0, bv3, cvec_result03);

                            av1 = _mm256_load_pd(a + ((i + 1) * n + k + 8));
                            cvec_result10 =
                                _mm256_fmadd_pd(av1, bv0, cvec_result10);
                            cvec_result11 =
                                _mm256_fmadd_pd(av1, bv1, cvec_result11);
                            cvec_result12 =
                                _mm256_fmadd_pd(av1, bv2, cvec_result12);
                            cvec_result13 =
                                _mm256_fmadd_pd(av1, bv3, cvec_result13);

                            av2 = _mm256_load_pd(a + ((i + 2) * n + k + 8));
                            cvec_result20 =
                                _mm256_fmadd_pd(av2, bv0, cvec_result20);
                            cvec_result21 =
                                _mm256_fmadd_pd(av2, bv1, cvec_result21);
                            cvec_result22 =
                                _mm256_fmadd_pd(av2, bv2, cvec_result22);
                            cvec_result23 =
                                _mm256_fmadd_pd(av2, bv3, cvec_result23);

                            av3 = _mm256_load_pd(a + ((i + 3) * n + k + 8));
                            cvec_result30 =
                                _mm256_fmadd_pd(av3, bv0, cvec_result30);
                            cvec_result31 =
                                _mm256_fmadd_pd(av3, bv1, cvec_result31);
                            cvec_result32 =
                                _mm256_fmadd_pd(av3, bv2, cvec_result32);
                            cvec_result33 =
                                _mm256_fmadd_pd(av3, bv3, cvec_result33);

                            ////////////////////////////////////////////////////////
                            av0 = _mm256_load_pd(a + (i * n + k + 12));

                            bv0 = _mm256_load_pd(d + (j * n + k + 12));
                            bv1 = _mm256_load_pd(d + ((j + 1) * n + k + 12));
                            bv2 = _mm256_load_pd(d + ((j + 2) * n + k + 12));
                            bv3 = _mm256_load_pd(d + ((j + 3) * n + k + 12));
                            cvec_result00 =
                                _mm256_fmadd_pd(av0, bv0, cvec_result00);
                            cvec_result01 =
                                _mm256_fmadd_pd(av0, bv1, cvec_result01);
                            cvec_result02 =
                                _mm256_fmadd_pd(av0, bv2, cvec_result02);
                            cvec_result03 =
                                _mm256_fmadd_pd(av0, bv3, cvec_result03);

                            av1 = _mm256_load_pd(a + ((i + 1) * n + k + 12));
                            cvec_result10 =
                                _mm256_fmadd_pd(av1, bv0, cvec_result10);
                            cvec_result11 =
                                _mm256_fmadd_pd(av1, bv1, cvec_result11);
                            cvec_result12 =
                                _mm256_fmadd_pd(av1, bv2, cvec_result12);
                            cvec_result13 =
                                _mm256_fmadd_pd(av1, bv3, cvec_result13);

                            av2 = _mm256_load_pd(a + ((i + 2) * n + k + 12));
                            cvec_result20 =
                                _mm256_fmadd_pd(av2, bv0, cvec_result20);
                            cvec_result21 =
                                _mm256_fmadd_pd(av2, bv1, cvec_result21);
                            cvec_result22 =
                                _mm256_fmadd_pd(av2, bv2, cvec_result22);
                            cvec_result23 =
                                _mm256_fmadd_pd(av2, bv3, cvec_result23);

                            av3 = _mm256_load_pd(a + ((i + 3) * n + k + 12));
                            cvec_result30 =
                                _mm256_fmadd_pd(av3, bv0, cvec_result30);
                            cvec_result31 =
                                _mm256_fmadd_pd(av3, bv1, cvec_result31);
                            cvec_result32 =
                                _mm256_fmadd_pd(av3, bv2, cvec_result32);
                            cvec_result33 =
                                _mm256_fmadd_pd(av3, bv3, cvec_result33);
                        }
                        double cstr[4];
                        _mm256_store_pd(cstr, cvec_result00);
                        c[i * p + j] += cstr[0] + cstr[1] + cstr[2] + cstr[3];
                        _mm256_store_pd(cstr, cvec_result01);
                        c[i * p + j + 1] += cstr[0] + cstr[1] + cstr[2] + cstr[3];
                        _mm256_store_pd(cstr, cvec_result02);
                        c[i * p + j + 2] += cstr[0] + cstr[1] + cstr[2] + cstr[3];
                        _mm256_store_pd(cstr, cvec_result03);
                        c[i * p + j + 3] += cstr[0] + cstr[1] + cstr[2] + cstr[3];

                        _mm256_store_pd(cstr, cvec_result10);
                        c[(i + 1) * p + j] += cstr[0] + cstr[1] + cstr[2] + cstr[3];
                        _mm256_store_pd(cstr, cvec_result11);
                        c[(i + 1) * p + j + 1] +=
                            cstr[0] + cstr[1] + cstr[2] + cstr[3];
                        _mm256_store_pd(cstr, cvec_result12);
                        c[(i + 1) * p + j + 2] +=
                            cstr[0] + cstr[1] + cstr[2] + cstr[3];
                        _mm256_store_pd(cstr, cvec_result13);
                        c[(i + 1) * p + j + 3] +=
                            cstr[0] + cstr[1] + cstr[2] + cstr[3];

                        _mm256_store_pd(cstr, cvec_result20);
                        c[(i + 2) * p + j] += cstr[0] + cstr[1] + cstr[2] + cstr[3];
                        _mm256_store_pd(cstr, cvec_result21);
                        c[(i + 2) * p + j + 1] +=
                            cstr[0] + cstr[1] + cstr[2] + cstr[3];
                        _mm256_store_pd(cstr, cvec_result22);
                        c[(i + 2) * p + j + 2] +=
                            cstr[0] + cstr[1] + cstr[2] + cstr[3];
                        _mm256_store_pd(cstr, cvec_result23);
                        c[(i + 2) * p + j + 3] +=
                            cstr[0] + cstr[1] + cstr[2] + cstr[3];

                        _mm256_store_pd(cstr, cvec_result30);
                        c[(i + 3) * p + j] += cstr[0] + cstr[1] + cstr[2] + cstr[3];
                        _mm256_store_pd(cstr, cvec_result31);
                        c[(i + 3) * p + j + 1] +=
                            cstr[0] + cstr[1] + cstr[2] + cstr[3];
                        _mm256_store_pd(cstr, cvec_result32);
                        c[(i + 3) * p + j + 2] +=
                            cstr[0] + cstr[1] + cstr[2] + cstr[3];
                        _mm256_store_pd(cstr, cvec_result33);
                        c[(i + 3) * p + j + 3] +=
                            cstr[0] + cstr[1] + cstr[2] + cstr[3];
                    }
                }
            }
        }
    }
    afree(d);
    return c;
}

// cache-friendly aligned mem simd-blocked matrix multiplication
// with transpose (dot-product results are taken in a width)
// to do: perform SIMD in parallel for the results
//
// Not a perfect algorithm for gemm, but a good start
double *cf_block_simd_acc_mul_al_tr(const double *a, const double *b, int m,
                                    int n, int p) {
    double *c =
        static_cast<double *>(amalloc(m * p * sizeof(double), 64));
    double *d =
        static_cast<double *>(amalloc(n * p * sizeof(double), 64));

    for (int i = 0; i < n; ++i) {
        for (int j = 0; j < p; ++j) {
            d[j * n + i] = b[i * p + j];
        }
    }
    for (int i = 0; i < m; ++i) {
        for (int j = 0; j < p; ++j) {
            c[i * p + j] = 0;
        }
    }

    const int bsize = 256;

    for (int ii = 0; ii < m; ii += bsize) {
        const int il = ii + bsize > m ? m : ii + bsize;
        for (int jj = 0; jj < n; jj += bsize) {
            const int jl = jj + bsize > n ? n : jj + bsize;
            // Perform on 4 values in a row
            for (int i = ii; i < il; i += 4) {
                // Perform on 4 values in a column
                for (int j = jj; j < jl; j += 4) {
                    __m256d cvec0 = _mm256_setzero_pd();
                    __m256d cvec1 = _mm256_setzero_pd();
                    __m256d cvec2 = _mm256_setzero_pd();
                    __m256d cvec3 = _mm256_setzero_pd();
                    for (int k = 0; k < n; k += 4) {
                        __m256d avec = _mm256_set1_pd(a[i * n + k]);
                        __m256d bvec = _mm256_set_pd(
                            b[j * n + k], b[(j + 1) * n + k],
                            b[(j + 2) * n + k], b[(j + 3) * n + k]);

                        cvec0 = _mm256_fmadd_pd(avec, bvec, cvec0);

                        avec = _mm256_set1_pd(a[(i + 1) * n + k]);
                        cvec1 = _mm256_fmadd_pd(avec, bvec, cvec1);

                        avec = _mm256_set1_pd(a[(i + 2) * n + k]);
                        cvec2 = _mm256_fmadd_pd(avec, bvec, cvec2);

                        avec = _mm256_set1_pd(a[(i + 3) * n + k]);
                        cvec3 = _mm256_fmadd_pd(avec, bvec, cvec3);

                        avec = _mm256_set1_pd(a[i * n + k + 1]);
                        bvec = _mm256_set_pd(
                            b[j * n + k + 1], b[(j + 1) * n + k + 1],
                            b[(j + 2) * n + k + 1], b[(j + 3) * n + k + 1]);

                        cvec0 = _mm256_fmadd_pd(avec, bvec, cvec0);

                        avec = _mm256_set1_pd(a[(i + 1) * n + k + 1]);
                        cvec1 = _mm256_fmadd_pd(avec, bvec, cvec1);

                        avec = _mm256_set1_pd(a[(i + 2) * n + k + 1]);
                        cvec2 = _mm256_fmadd_pd(avec, bvec, cvec2);

                        avec = _mm256_set1_pd(a[(i + 3) * n + k + 1]);
                        cvec3 = _mm256_fmadd_pd(avec, bvec, cvec3);

                        avec = _mm256_set1_pd(a[i * n + k + 2]);
                        bvec = _mm256_set_pd(
                            b[j * n + k + 2], b[(j + 1) * n + k + 2],
                            b[(j + 2) * n + k + 2], b[(j + 3) * n + k + 2]);

                        cvec0 = _mm256_fmadd_pd(avec, bvec, cvec0);

                        avec = _mm256_set1_pd(a[(i + 1) * n + k + 2]);
                        cvec1 = _mm256_fmadd_pd(avec, bvec, cvec1);

                        avec = _mm256_set1_pd(a[(i + 2) * n + k + 2]);
                        cvec2 = _mm256_fmadd_pd(avec, bvec, cvec2);

                        avec = _mm256_set1_pd(a[(i + 3) * n + k + 2]);
                        cvec3 = _mm256_fmadd_pd(avec, bvec, cvec3);

                        avec = _mm256_set1_pd(a[i * n + k + 3]);
                        bvec = _mm256_set_pd(
                            b[j * n + k + 3], b[(j + 1) * n + k + 3],
                            b[(j + 2) * n + k + 3], b[(j + 3) * n + k + 3]);

                        cvec0 = _mm256_fmadd_pd(avec, bvec, cvec0);

                        avec = _mm256_set1_pd(a[(i + 1) * n + k + 3]);
                        cvec1 = _mm256_fmadd_pd(avec, bvec, cvec1);

                        avec = _mm256_set1_pd(a[(i + 2) * n + k + 3]);
                        cvec2 = _mm256_fmadd_pd(avec, bvec, cvec2);

                        avec = _mm256_set1_pd(a[(i + 3) * n + k + 3]);
                        cvec3 = _mm256_fmadd_pd(avec, bvec, cvec3);
                    }
                    _mm256_store_pd(c + (i * p) + j, cvec0);
                    _mm256_store_pd(c + ((i + 1) * p) + j, cvec1);
                    _mm256_store_pd(c + ((i + 2) * p) + j, cvec2);
                    _mm256_store_pd(c + ((i + 3) * p) + j, cvec3);
                }
            }
        }
    }
    afree(d);
    return c;
}

bool comp(double *a, double *b, int r, int c) {
    for (int i = 0; i < r; ++i) {
        for (int j = 0; j < c; ++j) {
            if (a[i * r + j] != b[i * r + j]) {
                return false;
            }
        }
    }
    return true;
}

void display(double *a, int m, int n) {
    for (int i = 0; i < m; ++i) {
        for (int j = 0; j < n; ++j) {
            std::cout << a[i * n + j] << ", ";
        }
        std::cout << '\n';
    }
}

void test_normal_malloc() {
    std::cout << "Testing normal malloc\n";
    const int m = 1024, n = 1024, p = 1024;
    double *a = static_cast<double *>(malloc(m * n * sizeof(double))),
           *b = static_cast<double *>(malloc(n * p * sizeof(double)));

    for (int i = 0; i < m; ++i) {
        for (int j = 0; j < n; ++j) {
            a[i * n + j] = i;
            b[i * n + j] = j;
        }
    }

    auto start = std::chrono::system_clock::now();
    auto ans = mul(a, b, m, n, p);
    auto end = std::chrono::system_clock::now();
    std::chrono::duration<double> time = end - start;

    // display(ans, m, p);
    std::cout << "naive: " << time.count() << "s\n";
    // free(ans);
    // ans = nullptr;

    start = std::chrono::system_clock::now();
    auto ans1 = cf_mul(a, b, m, n, p);
    end = std::chrono::system_clock::now();
    time = end - start;

    std::cout << "cache friendly: " << time.count()
              << "s, equal: " << comp(ans, ans1, m, p) << "\n";
    free(ans1);

    start = std::chrono::system_clock::now();
    auto ans2 = cf_block_mul(a, b, m, n, p);
    end = std::chrono::system_clock::now();
    time = end - start;

    std::cout << "cache friendly block mul: " << time.count()
              << "s, equal: " << comp(ans, ans2, m, p) << "\n";
    free(ans2);

    start = std::chrono::system_clock::now();
    auto ans3 = cf_simd_mul(a, b, m, n, p);
    end = std::chrono::system_clock::now();
    time = end - start;

    // display(ans2, m, p);
    std::cout << "cache friendly simd: " << time.count()
              << "s, equal: " << comp(ans, ans3, m, p) << "\n";

    free(ans3);

    start = std::chrono::system_clock::now();
    auto ans4 = cf_simd_block_mul(a, b, m, n, p);
    end = std::chrono::system_clock::now();
    time = end - start;

    // display(ans2, m, p);
    std::cout << "cache friendly blocked simd: " << time.count()
              << "s, equal: " << comp(ans, ans4, m, p) << "\n";

    free(ans4);

    free(a);
    free(b);
    free(ans);
}

void testamalloc() {
    int sz = 1024;
    std::cout << "Testing aligned malloc\n";
    int m = sz, n = sz, p = sz;
    double *a = static_cast<double *>(
               amalloc(m * n * sizeof(double), 64)),
           *b = static_cast<double *>(
               amalloc(m * n * sizeof(double), 64));

    for (int i = 0; i < m; ++i) {
        for (int j = 0; j < n; ++j) {
            a[i * m + j] = i;
            b[i * m + j] = j;
        }
    }

    double *ans = nullptr;

    auto start = std::chrono::system_clock::now();
    ans = mul_al(a, b, m, n, p);
    auto end = std::chrono::system_clock::now();
    std::chrono::duration<double> time = end - start;

    std::cout << "naive: " << time.count() << "s\n";

    // start = std::chrono::system_clock::now();
    // auto ans1 = cf_mul_al(a, b, m, n, p);
    // end = std::chrono::system_clock::now();
    // time = end - start;

    // std::cout << "cache friendly: " << time.count()
    //           << "s, equal: " << comp(ans, ans1, m, p) << "\n";
    // afree(ans1);

    // start = std::chrono::system_clock::now();
    // auto ans2 = cf_block_mul_al(a, b, m, n, p);
    // end = std::chrono::system_clock::now();
    // time = end - start;

    // std::cout << "cache friendly blocked mul: " << time.count()
    //           << "s, equal: " << comp(ans, ans2, m, p) << "\n";
    // afree(ans2);

    // start = std::chrono::system_clock::now();
    // auto ans3 = cf_simd_mul_al(a, b, m, n, p);
    // end = std::chrono::system_clock::now();
    // time = end - start;

    // std::cout << "cache friendly simd: " << time.count()
    //           << "s, equal: " << comp(ans, ans3, m, p) << "\n";
    // afree(ans3);

    // start = std::chrono::system_clock::now();
    // auto ans4 = cf_simd_block_mul_al(a, b, m, n, p);
    // end = std::chrono::system_clock::now();
    // time = end - start;

    // std::cout << "cache friendly blocked simd: " << time.count()
    //           << "s, equal: " << comp(ans, ans4, m, p) << "\n";
    // afree(ans4);

    // start = std::chrono::system_clock::now();
    // auto ans6 = cf_block_mul_al_tr(a, b, m, n, p);
    // end = std::chrono::system_clock::now();
    // time = end - start;

    // std::cout << "cache friendly blocked transposed: " << time.count()
    //           << "s, equal: " << comp(ans, ans6, m, p) << "\n";
    // afree(ans6);

    start = std::chrono::system_clock::now();
    auto ans5 = cf_block_simd_mul_al_tr(a, b, m, n, p);
    end = std::chrono::system_clock::now();
    time = end - start;

    std::cout << "cache friendly blocked simd transposed: " << time.count()
              << "s, equal: " << comp(ans, ans5, m, p) << "\n";
    afree(ans5);

    start = std::chrono::system_clock::now();
    auto ans7 = cf_block_simd_acc_mul_al_tr(a, b, m, n, p);
    end = std::chrono::system_clock::now();
    time = end - start;

    std::cout << "cache friendly blocked simd acc transposed: " << time.count()
              << "s, equal: " << comp(ans, ans7, m, p) << "\n";
    afree(ans7);

    afree(a);
    afree(b);
    afree(ans);
}

int main() {
    test_normal_malloc();
    testamalloc();
}