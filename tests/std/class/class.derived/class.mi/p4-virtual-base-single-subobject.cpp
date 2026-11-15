// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.mi]/4 (figure 5): for each distinct base class that is
// specified virtual, the most derived object contains a single base class
// subobject of that type. A JoinsWriterAndReader object has one SharedBase,
// shared by its WritesSharedBase and ReadsSharedBase subobjects.
#include <cassert>

static int shared_base_ctors;

struct SharedBase {
  int value;
  SharedBase() : value(0) { ++shared_base_ctors; }
};

struct WritesSharedBase : virtual SharedBase {
  void write(int x) { value = x; }
};

struct ReadsSharedBase : virtual SharedBase {
  int read() { return value; }
};

struct JoinsWriterAndReader : WritesSharedBase, ReadsSharedBase {};

int main() {
  JoinsWriterAndReader d;
  assert(shared_base_ctors == 1);
  d.write(4);
  assert(d.read() == 4);
  assert(d.value == 4);

  WritesSharedBase *writer = &d;
  ReadsSharedBase *reader = &d;
  SharedBase *throughWriter = writer;
  SharedBase *throughReader = reader;
  SharedBase *direct = &d;
  assert(throughWriter == throughReader);
  assert(throughWriter == direct);
  throughReader->value = 5;
  assert(writer->value == 5);
  assert(d.WritesSharedBase::value == 5);
  assert(d.ReadsSharedBase::value == 5);

  WritesSharedBase onlyWriter;
  ReadsSharedBase onlyReader;
  assert(shared_base_ctors == 3);
  onlyWriter.write(6);
  assert(onlyReader.read() == 0);
  assert(d.value == 5);
  return 0;
}
