#include <immintrin.h>
#include <omp.h>

#include <chrono>
#include <iostream>

// How to run:
// g++ -march=native matrix_multiplication_simd_basic.cpp -O2

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
        static_cast<double *>(_aligned_malloc(m * p * sizeof(double), 64));
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
        static_cast<double *>(_aligned_malloc(m * p * sizeof(double), 64));
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
        static_cast<double *>(_aligned_malloc(m * p * sizeof(double), 64));
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
        static_cast<double *>(_aligned_malloc(m * p * sizeof(double), 64));
    double *d =
        static_cast<double *>(_aligned_malloc(n * p * sizeof(double), 64));

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
    const int bsize = 32;
    for (int ii = 0; ii < m; ii += bsize) {
        const int il = ii + bsize > m ? m : ii + bsize;
        for (int kk = 0; kk < n; kk += bsize) {
            const int kl = kk + bsize > n ? n : kk + bsize;
            for (int i = ii; i < il; ++i) {
                for (int j = 0; j < p; ++j) {
                    double ans = 0;
                    for (int k = kk; k < kl; ++k) {
                        ans += a[i * n + k] * d[j * n + k];
                    }
                    c[i * p + j] = ans;
                }
            }
        }
    }
    _aligned_free(d);
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
        static_cast<double *>(_aligned_malloc(m * p * sizeof(double), 64));
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
        static_cast<double *>(_aligned_malloc(m * p * sizeof(double), 64));
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

    // display(a, m, n);
    // std::cout << "\n\n\n";
    // display(b, n, p);

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

void test_aligned_malloc() {
    // omp_set_num_threads(16);
    std::cout << "Testing aligned malloc\n";
    int m = 1024, n = 1024, p = 1024;
    double *a = static_cast<double *>(
               _aligned_malloc(m * n * sizeof(double), 64)),
           *b = static_cast<double *>(
               _aligned_malloc(m * n * sizeof(double), 64));

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
    // free(ans);
    // ans = nullptr;

    start = std::chrono::system_clock::now();
    auto ans1 = cf_mul_al(a, b, m, n, p);
    end = std::chrono::system_clock::now();
    time = end - start;

    std::cout << "cache friendly: " << time.count()
              << "s, equal: " << comp(ans, ans1, m, p) << "\n";
    _aligned_free(ans1);

    start = std::chrono::system_clock::now();
    auto ans2 = cf_block_mul_al(a, b, m, n, p);
    end = std::chrono::system_clock::now();
    time = end - start;

    std::cout << "cache friendly blocked mul: " << time.count()
              << "s, equal: " << comp(ans, ans2, m, p) << "\n";
    _aligned_free(ans2);

    start = std::chrono::system_clock::now();
    auto ans3 = cf_simd_mul_al(a, b, m, n, p);
    end = std::chrono::system_clock::now();
    time = end - start;

    std::cout << "cache friendly simd: " << time.count()
              << "s, equal: " << comp(ans, ans3, m, p) << "\n";
    _aligned_free(ans3);

    start = std::chrono::system_clock::now();
    auto ans4 = cf_simd_block_mul_al(a, b, m, n, p);
    end = std::chrono::system_clock::now();
    time = end - start;

    std::cout << "cache friendly blocked simd: " << time.count()
              << "s, equal: " << comp(ans, ans4, m, p) << "\n";
    _aligned_free(ans4);

    _aligned_free(a);
    _aligned_free(b);
    _aligned_free(ans);
}

int main() {
    test_normal_malloc();
    test_aligned_malloc();
}
