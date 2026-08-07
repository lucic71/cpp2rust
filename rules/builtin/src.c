int f12(long a, long b, long *r) { return __builtin_mul_overflow(a, b, r); }
int f13(long long a, long long b, long long *r) { return __builtin_mul_overflow(a, b, r); }
unsigned long long f14(unsigned long long x) { return __builtin_bswap64(x); }
int f15(unsigned long long x) { return __builtin_ctzll(x); }
double f16(void) { return __builtin_huge_val(); }
double f18(double x) { return __builtin_ceil(x); }
double f19(double x) { return __builtin_floor(x); }
int f20(unsigned long long x) { return __builtin_clzll(x); }
float f21(void) { return __builtin_inff(); }
float f22(const char *s) { return __builtin_nanf(s); }
void f23(void) { return __builtin_unreachable(); }
