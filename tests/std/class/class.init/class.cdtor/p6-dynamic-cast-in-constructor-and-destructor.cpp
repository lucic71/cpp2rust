// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.cdtor]/6: when a dynamic_cast is used in a constructor or
// destructor, or in a function called from one, and its operand refers to the
// object under construction or destruction, that object is considered to be a
// most derived object of the constructor's or destructor's class. A cast to
// the derived class fails there and succeeds once construction is complete.
#include <cassert>

struct CastsToDerived;
struct DerivesFromCastsToDerived;

static int failed_casts, successful_casts;

static void castToDerived(CastsToDerived *object);

struct CastsToDerived {
  CastsToDerived() { castToDerived(this); }
  virtual ~CastsToDerived() { castToDerived(this); }
};

struct DerivesFromCastsToDerived : CastsToDerived {
  DerivesFromCastsToDerived() { castToDerived(this); }
  ~DerivesFromCastsToDerived() override { castToDerived(this); }
};

static void castToDerived(CastsToDerived *object) {
  if (dynamic_cast<DerivesFromCastsToDerived *>(object) == nullptr) {
    ++failed_casts;
  } else {
    ++successful_casts;
  }
}

int main() {
  {
    DerivesFromCastsToDerived d;
    assert(failed_casts == 1 && successful_casts == 1);
    castToDerived(&d);
    assert(failed_casts == 1 && successful_casts == 2);
  }
  assert(failed_casts == 2 && successful_casts == 3);
  return 0;
}
