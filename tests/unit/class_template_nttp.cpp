#include <array>
#include <cassert>
#include <vector>

template <int N> struct Step {
  int value;
  void advance() { value += N; }
  int scaled() const { return value * N; }
};

template <> struct Step<0> {
  int value;
  void advance() { value = -1; }
  int scaled() const { return 0; }
};

template <bool B> struct Choice {
  int pick(int a, int b) const { return B ? a : b; }
};

template <typename T, int N> struct Buffer {
  T data[N];
  int size() const { return N; }
  T sum() const {
    T total = 0;
    for (int i = 0; i < N; ++i) {
      total += data[i];
    }
    return total;
  }
};

template <char C, bool Repeat> struct Mark {
  int count(const char *s) const {
    int n = 0;
    for (; *s; ++s) {
      if (*s == C) {
        ++n;
        if (!Repeat) {
          break;
        }
      }
    }
    return n;
  }
};

template <int Lo, int Hi> struct Range {
  bool contains(int x) const { return x >= Lo && x <= Hi; }
  int width() const { return Hi - Lo; }
};

template <unsigned char U, short S, long long L> struct Mix {
  long long total() const { return U + S + L; }
};

int main() {
  Step<2> s2{1};
  Step<3> s3{1};
  s2.advance();
  s3.advance();
  assert(s2.value == 3);
  assert(s3.value == 4);
  assert(s2.scaled() == 6);
  assert(s3.scaled() == 12);

  Step<-1> sn{5};
  sn.advance();
  assert(sn.value == 4);
  assert(sn.scaled() == -4);

  Step<0> s0{5};
  s0.advance();
  assert(s0.value == -1);
  assert(s0.scaled() == 0);

  Choice<true> yes;
  Choice<false> no;
  assert(yes.pick(1, 2) == 1);
  assert(no.pick(1, 2) == 2);

  Buffer<int, 2> b2{{1, 2}};
  Buffer<int, 3> b3{{1, 2, 3}};
  assert(b2.size() == 2 && b2.sum() == 3);
  assert(b3.size() == 3 && b3.sum() == 6);

  std::vector<Step<2>> v2;
  std::vector<Step<3>> v3;
  v2.push_back(Step<2>{0});
  v3.push_back(Step<3>{0});
  v2[0].advance();
  v3[0].advance();
  assert(v2.size() == 1 && v2[0].value == 2);
  assert(v3.size() == 1 && v3[0].value == 3);

  std::array<Step<2>, 2> a2{{{1}, {2}}};
  std::array<Step<3>, 3> a3{{{1}, {2}, {3}}};
  for (auto &s : a2) {
    s.advance();
  }
  for (auto &s : a3) {
    s.advance();
  }
  assert(a2.size() == 2 && a2[1].scaled() == 8);
  assert(a3.size() == 3 && a3[2].scaled() == 18);

  std::vector<Buffer<int, 2>> vb2;
  std::vector<Buffer<int, 3>> vb3;
  vb2.push_back(b2);
  vb3.push_back(b3);
  assert(vb2[0].size() == 2 && vb2[0].sum() == 3);
  assert(vb3[0].size() == 3 && vb3[0].sum() == 6);

  std::array<Choice<true>, 1> ayes{};
  std::array<Choice<false>, 1> ano{};
  assert(ayes[0].pick(1, 2) == 1);
  assert(ano[0].pick(1, 2) == 2);

  Mark<'a', true> all_a;
  Mark<'a', false> first_a;
  Mark<'b', true> all_b;
  assert(all_a.count("banana") == 3);
  assert(first_a.count("banana") == 1);
  assert(all_b.count("banana") == 1);

  Range<0, 10> wide;
  Range<5, 6> narrow;
  Range<-3, 0> neg;
  assert(wide.contains(7) && !narrow.contains(7));
  assert(wide.width() == 10 && narrow.width() == 1);
  assert(neg.contains(-2) && neg.width() == 3);

  Mix<200, -3, 10000000000LL> big;
  Mix<1, 1, 1> small;
  assert(big.total() == 10000000197LL);
  assert(small.total() == 3);

  std::vector<Range<0, 10>> vwide{wide};
  std::vector<Range<5, 6>> vnarrow{narrow};
  assert(vwide[0].contains(7) && !vnarrow[0].contains(7));

  std::array<Mark<'a', true>, 2> amarks{};
  std::array<Mark<'a', false>, 2> afirst{};
  assert(amarks[1].count("aaa") == 3);
  assert(afirst[1].count("aaa") == 1);

  std::vector<Mix<200, -3, 10000000000LL>> vbig{big, big};
  assert(vbig.size() == 2 && vbig[1].total() == 10000000197LL);
  return 0;
}
