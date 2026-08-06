#include <assert.h>

struct Rec {
  int kind;
  union {
    int i;
    char *z;
    long *pBig;
  } u;
};

static const struct Rec zeroRec = {0};

static struct Rec *get_rec(void) {
  static struct Rec dummy;
  return &dummy;
}

int main(void) {
  assert(zeroRec.kind == 0);
  assert(zeroRec.u.z == 0);

  struct Rec *p = get_rec();
  assert(p->u.pBig == 0);
  p->u.i = 5;
  assert(p->u.i == 5);

  struct Rec r;
  r.kind = 3;
  r.u.i = 9;
  r = zeroRec;
  assert(r.kind == 0);
  assert(r.u.i == 0);
  return 0;
}
