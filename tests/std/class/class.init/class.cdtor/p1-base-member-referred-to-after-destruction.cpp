// panic-ub: refcount
// nondet-result: unsafe
// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.cdtor]/1: for an object with a non-trivial destructor,
// referring to any non-static member or base class of the object after the
// destructor finishes execution is undefined. The base member is read after
// an explicit destructor call.

struct HasMember {
  int value = 7;
};

struct HasNonTrivialDestructor : HasMember {
  ~HasNonTrivialDestructor() { value = 0; }
};

int main() {
  HasNonTrivialDestructor *p = new HasNonTrivialDestructor;
  p->~HasNonTrivialDestructor();
  return p->value;
}
