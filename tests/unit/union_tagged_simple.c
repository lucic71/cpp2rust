#include <assert.h>

typedef enum {
  KIND_NONE,
  KIND_DONE,
} Kind;

struct Event {
  Kind kind;
  void *handle;
  union {
    void *obj;
    int code;
  } payload;
};

static struct Event make_event(int code) {
  return (struct Event){KIND_DONE, 0, {.code = code}};
}

static struct Event make_ref(void *p) {
  return (struct Event){KIND_NONE, 0, {.obj = p}};
}

int main(void) {
  int dummy = 0;

  struct Event m1;
  m1.kind = KIND_DONE;
  m1.handle = &dummy;
  m1.payload.code = 42;
  assert(m1.kind == KIND_DONE);
  assert(m1.payload.code == 42);

  struct Event m2;
  m2.kind = KIND_NONE;
  m2.handle = &dummy;
  m2.payload.obj = &dummy;
  assert(m2.payload.obj == &dummy);

  struct Event m3 = make_event(dummy + 7);
  assert(m3.kind == KIND_DONE);
  assert(m3.payload.code == 7);

  struct Event m4 = make_ref(&dummy);
  assert(m4.payload.obj == &dummy);

  return 0;
}
