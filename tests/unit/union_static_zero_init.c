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

static const union {
  char a[8];
  short align;
} blob = {"0123456"};

union Num {
  int i;
  unsigned char b[4];
};
static const union Num num = {0x01020304};

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

  assert(blob.a[0] == '0');
  assert(blob.a[6] == '6');
  assert(blob.a[7] == 0);
  assert(num.i == 0x01020304);
  assert(num.b[0] == 4);
  assert(num.b[3] == 1);
  return 0;
}
