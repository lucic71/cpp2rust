// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <math.h>

double f1(double x) { return cos(x); }

double f2(double x) { return round(x); }

double f3(double x) { return sin(x); }

double f4(double a0) { return acos(a0); }

float f5(float a0) { return acosf(a0); }

double f6(double a0) { return acosh(a0); }

float f7(float a0) { return acoshf(a0); }

long double f8(long double a0) { return acoshl(a0); }

long double f9(long double a0) { return acosl(a0); }

double f10(double a0) { return asin(a0); }

float f11(float a0) { return asinf(a0); }

double f12(double a0) { return asinh(a0); }

float f13(float a0) { return asinhf(a0); }

long double f14(long double a0) { return asinhl(a0); }

long double f15(long double a0) { return asinl(a0); }

double f16(double a0) { return atan(a0); }

double f17(double a0, double a1) { return atan2(a0, a1); }

float f18(float a0, float a1) { return atan2f(a0, a1); }

long double f19(long double a0, long double a1) { return atan2l(a0, a1); }

float f20(float a0) { return atanf(a0); }

double f21(double a0) { return atanh(a0); }

float f22(float a0) { return atanhf(a0); }

long double f23(long double a0) { return atanhl(a0); }

long double f24(long double a0) { return atanl(a0); }

double f25(double a0) { return cbrt(a0); }

float f26(float a0) { return cbrtf(a0); }

long double f27(long double a0) { return cbrtl(a0); }

double f28(double a0) { return ceil(a0); }

float f29(float a0) { return ceilf(a0); }

long double f30(long double a0) { return ceill(a0); }

double f31(double a0, double a1) { return copysign(a0, a1); }

float f32(float a0, float a1) { return copysignf(a0, a1); }

long double f33(long double a0, long double a1) { return copysignl(a0, a1); }

float f34(float a0) { return cosf(a0); }

double f35(double a0) { return cosh(a0); }

float f36(float a0) { return coshf(a0); }

long double f37(long double a0) { return coshl(a0); }

long double f38(long double a0) { return cosl(a0); }

double f39(double a0) { return erf(a0); }

double f40(double a0) { return erfc(a0); }

float f41(float a0) { return erfcf(a0); }

long double f42(long double a0) { return erfcl(a0); }

float f43(float a0) { return erff(a0); }

long double f44(long double a0) { return erfl(a0); }

double f45(double a0) { return exp(a0); }

double f46(double a0) { return exp2(a0); }

float f47(float a0) { return exp2f(a0); }

long double f48(long double a0) { return exp2l(a0); }

float f49(float a0) { return expf(a0); }

long double f50(long double a0) { return expl(a0); }

double f51(double a0) { return expm1(a0); }

float f52(float a0) { return expm1f(a0); }

long double f53(long double a0) { return expm1l(a0); }

double f54(double a0, double a1) { return fdim(a0, a1); }

float f55(float a0, float a1) { return fdimf(a0, a1); }

long double f56(long double a0, long double a1) { return fdiml(a0, a1); }

double f57(double a0) { return floor(a0); }

float f58(float a0) { return floorf(a0); }

long double f59(long double a0) { return floorl(a0); }

double f60(double a0, double a1, double a2) { return fma(a0, a1, a2); }

float f61(float a0, float a1, float a2) { return fmaf(a0, a1, a2); }

long double f62(long double a0, long double a1, long double a2) { return fmal(a0, a1, a2); }

double f63(double a0, double a1) { return fmax(a0, a1); }

float f64(float a0, float a1) { return fmaxf(a0, a1); }

long double f65(long double a0, long double a1) { return fmaxl(a0, a1); }

double f66(double a0, double a1) { return fmin(a0, a1); }

float f67(float a0, float a1) { return fminf(a0, a1); }

long double f68(long double a0, long double a1) { return fminl(a0, a1); }

float f69(float a0, float a1) { return fmodf(a0, a1); }

long double f70(long double a0, long double a1) { return fmodl(a0, a1); }

double f71(double a0, int *a1) { return frexp(a0, a1); }

float f72(float a0, int *a1) { return frexpf(a0, a1); }

long double f73(long double a0, int *a1) { return frexpl(a0, a1); }

double f74(double a0, double a1) { return hypot(a0, a1); }

float f75(float a0, float a1) { return hypotf(a0, a1); }

long double f76(long double a0, long double a1) { return hypotl(a0, a1); }

int f77(double a0) { return ilogb(a0); }

int f78(float a0) { return ilogbf(a0); }

int f79(long double a0) { return ilogbl(a0); }

double f80(double a0, int a1) { return ldexp(a0, a1); }

float f81(float a0, int a1) { return ldexpf(a0, a1); }

long double f82(long double a0, int a1) { return ldexpl(a0, a1); }

double f83(double a0) { return lgamma(a0); }

float f84(float a0) { return lgammaf(a0); }

long double f85(long double a0) { return lgammal(a0); }

long long f86(double a0) { return llrint(a0); }

long long f87(float a0) { return llrintf(a0); }

long long f88(long double a0) { return llrintl(a0); }

long long f89(double a0) { return llround(a0); }

long long f90(float a0) { return llroundf(a0); }

long long f91(long double a0) { return llroundl(a0); }

double f92(double a0) { return log(a0); }

double f93(double a0) { return log10(a0); }

float f94(float a0) { return log10f(a0); }

long double f95(long double a0) { return log10l(a0); }

double f96(double a0) { return log1p(a0); }

float f97(float a0) { return log1pf(a0); }

long double f98(long double a0) { return log1pl(a0); }

double f99(double a0) { return log2(a0); }

float f100(float a0) { return log2f(a0); }

long double f101(long double a0) { return log2l(a0); }

double f102(double a0) { return logb(a0); }

float f103(float a0) { return logbf(a0); }

long double f104(long double a0) { return logbl(a0); }

float f105(float a0) { return logf(a0); }

long double f106(long double a0) { return logl(a0); }

long f107(double a0) { return lrint(a0); }

long f108(float a0) { return lrintf(a0); }

long f109(long double a0) { return lrintl(a0); }

long f110(double a0) { return lround(a0); }

long f111(float a0) { return lroundf(a0); }

long f112(long double a0) { return lroundl(a0); }

float f113(float a0, float *a1) { return modff(a0, a1); }

long double f114(long double a0, long double *a1) { return modfl(a0, a1); }

double f115(double a0) { return nearbyint(a0); }

float f116(float a0) { return nearbyintf(a0); }

long double f117(long double a0) { return nearbyintl(a0); }

double f118(double a0, double a1) { return nextafter(a0, a1); }

float f119(float a0, float a1) { return nextafterf(a0, a1); }

long double f120(long double a0, long double a1) { return nextafterl(a0, a1); }

double f121(double a0, long double a1) { return nexttoward(a0, a1); }

float f122(float a0, long double a1) { return nexttowardf(a0, a1); }

long double f123(long double a0, long double a1) { return nexttowardl(a0, a1); }

double f124(double a0, double a1) { return pow(a0, a1); }

float f125(float a0, float a1) { return powf(a0, a1); }

long double f126(long double a0, long double a1) { return powl(a0, a1); }

float f127(float a0, float a1) { return remainderf(a0, a1); }

long double f128(long double a0, long double a1) { return remainderl(a0, a1); }

float f129(float a0, float a1, int *a2) { return remquof(a0, a1, a2); }

long double f130(long double a0, long double a1, int *a2) { return remquol(a0, a1, a2); }

double f131(double a0) { return rint(a0); }

float f132(float a0) { return rintf(a0); }

long double f133(long double a0) { return rintl(a0); }

float f134(float a0) { return roundf(a0); }

long double f135(long double a0) { return roundl(a0); }

double f136(double a0, long a1) { return scalbln(a0, a1); }

float f137(float a0, long a1) { return scalblnf(a0, a1); }

long double f138(long double a0, long a1) { return scalblnl(a0, a1); }

double f139(double a0, int a1) { return scalbn(a0, a1); }

float f140(float a0, int a1) { return scalbnf(a0, a1); }

long double f141(long double a0, int a1) { return scalbnl(a0, a1); }

float f142(float a0) { return sinf(a0); }

double f143(double a0) { return sinh(a0); }

float f144(float a0) { return sinhf(a0); }

long double f145(long double a0) { return sinhl(a0); }

long double f146(long double a0) { return sinl(a0); }

float f147(float a0) { return sqrtf(a0); }

long double f148(long double a0) { return sqrtl(a0); }

double f149(double a0) { return tan(a0); }

float f150(float a0) { return tanf(a0); }

double f151(double a0) { return tanh(a0); }

float f152(float a0) { return tanhf(a0); }

long double f153(long double a0) { return tanhl(a0); }

long double f154(long double a0) { return tanl(a0); }

double f155(double a0) { return tgamma(a0); }

float f156(float a0) { return tgammaf(a0); }

long double f157(long double a0) { return tgammal(a0); }

double f158(double a0) { return trunc(a0); }

float f159(float a0) { return truncf(a0); }

long double f160(long double a0) { return truncl(a0); }

double f161(double a0) { return sqrt(a0); }

double f162(double a0) { return fabs(a0); }

double f163(double a0, double a1) { return fmod(a0, a1); }

double f164(double a0, double a1) { return remainder(a0, a1); }

float f165(float a0) { return fabsf(a0); }

long double f166(long double a0) { return fabsl(a0); }
