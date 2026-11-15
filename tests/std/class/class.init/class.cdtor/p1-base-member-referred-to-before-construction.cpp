// panic-ub: refcount
// nondet-result: unsafe
// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.cdtor]/1: for an object with a non-trivial constructor,
// referring to any non-static member or base class of the object before the
// constructor begins execution is undefined. The pointer to the base member is
// formed before the object is constructed.

struct HasMember {
  int value;
};

struct HasNonTrivialConstructor : HasMember {
  HasNonTrivialConstructor() { value = 7; }
};

extern HasNonTrivialConstructor object;
int *formedBeforeConstruction = &object.value;
HasNonTrivialConstructor object;

int main() { return *formedBeforeConstruction; }
