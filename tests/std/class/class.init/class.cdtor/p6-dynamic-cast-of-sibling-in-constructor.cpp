// panic-ub: refcount
// nondet-result: unsafe
// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.cdtor]/6: if the operand of a dynamic_cast refers to the object
// under construction and its static type is not a pointer to the
// constructor's own class or one of its bases, the behavior is undefined. The
// constructor casts a pointer to its sibling base to its own class.

struct VirtualBase {
  virtual void f() {}
};

struct Sibling : virtual VirtualBase {};

struct CastsSiblingInConstructor : virtual VirtualBase {
  bool castSucceeded;
  CastsSiblingInConstructor(Sibling *sibling)
      : castSucceeded(dynamic_cast<CastsSiblingInConstructor *>(sibling) !=
                      nullptr) {}
};

struct MostDerived : Sibling, CastsSiblingInConstructor {
  MostDerived() : CastsSiblingInConstructor(this) {}
};

int main() {
  MostDerived d;
  return d.castSucceeded ? 1 : 2;
}
