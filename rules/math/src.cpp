// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <math.h>

double f1(double x) { return cos(x); }

double f2(double x) { return round(x); }

double f3(double x) { return sin(x); }

double f4(double x) { return fabs(x); }

double f5(double x) { return trunc(x); }

double f6(double x) { return floor(x); }

double f7(double x, double y) { return fmod(x, y); }

double f8(double x) { return ceil(x); }

double f9(double x) { return sqrt(x); }

double f10(double x) { return sin(x); }

double f11(double x) { return tan(x); }

double f12(double x) { return asin(x); }

double f13(double x) { return acos(x); }

double f14(double x) { return atan(x); }

double f15(double x) { return sinh(x); }

double f16(double x) { return cosh(x); }

double f17(double x) { return tanh(x); }

double f18(double x) { return asinh(x); }

double f19(double x) { return acosh(x); }

double f20(double x) { return atanh(x); }

double f21(double x) { return exp(x); }

double f22(double x) { return expm1(x); }

double f23(double x) { return log(x); }

double f24(double x) { return log10(x); }

double f25(double x) { return log1p(x); }

double f26(double x) { return cbrt(x); }

long f27(double x) { return lrint(x); }

double f28(double x, double y) { return atan2(x, y); }

double f29(double x, double y) { return hypot(x, y); }

double f30(double x, double y) { return copysign(x, y); }

double f31(double x, double y) { return pow(x, y); }

double f32(double x, int n) { return scalbn(x, n); }

double f33(double x, int *e) { return frexp(x, e); }
