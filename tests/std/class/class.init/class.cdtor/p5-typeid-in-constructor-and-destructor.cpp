// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.cdtor]/5: when typeid is used in a constructor or destructor,
// or in a function called from one, and its operand refers to the object under
// construction or destruction, it yields the type_info of the constructor's or
// destructor's class.
#include <cassert>
#include <typeinfo>

struct ChecksTypeid;
struct DerivesFromChecksTypeid;

static int base_checks, derived_checks;

static void recordType(const ChecksTypeid &object);

struct ChecksTypeid {
  ChecksTypeid() { recordType(*this); }
  virtual ~ChecksTypeid() { recordType(*this); }
};

struct DerivesFromChecksTypeid : ChecksTypeid {
  DerivesFromChecksTypeid() { recordType(*this); }
  ~DerivesFromChecksTypeid() override { recordType(*this); }
};

static void recordType(const ChecksTypeid &object) {
  if (typeid(object) == typeid(ChecksTypeid)) {
    ++base_checks;
  }
  if (typeid(object) == typeid(DerivesFromChecksTypeid)) {
    ++derived_checks;
  }
}

int main() {
  {
    DerivesFromChecksTypeid d;
    assert(base_checks == 1 && derived_checks == 1);
    recordType(d);
    assert(base_checks == 1 && derived_checks == 2);
  }
  assert(base_checks == 2 && derived_checks == 3);
  return 0;
}
