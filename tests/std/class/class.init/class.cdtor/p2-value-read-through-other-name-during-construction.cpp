// nondet-result
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.cdtor]/2: during the construction of an object, if its value is
// accessed through a glvalue that is not obtained from the constructor's this
// pointer, the value obtained is unspecified. The constructor reads the member
// first through the name of the object being constructed.

struct ReadsItselfThroughItsName {
  int first, second;
  ReadsItselfThroughItsName(int a);
};

extern ReadsItselfThroughItsName object;

ReadsItselfThroughItsName::ReadsItselfThroughItsName(int a)
    : first(a), second(object.first) {}

ReadsItselfThroughItsName object = ReadsItselfThroughItsName(1);

int main() { return object.second; }
