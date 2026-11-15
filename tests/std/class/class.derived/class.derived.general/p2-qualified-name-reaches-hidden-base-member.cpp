// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.derived.general]/2, note 2: the scope resolution operator can
// refer to a direct or indirect base member explicitly, even if it is hidden
// in the derived class.
#include <cassert>

static int base_calls, first_calls, second_calls;

struct DeclaresTouch {
  void touch() { ++base_calls; }
};

struct HidesTouch : DeclaresTouch {
  void touch() { ++first_calls; }
};

struct HidesTouchAgain : HidesTouch {
  void touch() { ++second_calls; }
  void touchAll() {
    touch();
    HidesTouch::touch();
    DeclaresTouch::touch();
  }
};

int main() {
  HidesTouchAgain d;
  d.touch();
  assert(base_calls == 0 && first_calls == 0 && second_calls == 1);
  d.HidesTouch::touch();
  assert(base_calls == 0 && first_calls == 1 && second_calls == 1);
  d.DeclaresTouch::touch();
  assert(base_calls == 1 && first_calls == 1 && second_calls == 1);
  d.touchAll();
  assert(base_calls == 2 && first_calls == 2 && second_calls == 2);
  return 0;
}
