#include <assert.h>
#include <stddef.h>
#include <string.h>

struct flags {
  unsigned char tag;
  unsigned a : 1;
  unsigned b : 3;
  int x;
  unsigned c : 1;
};

struct outer {
  char lead;
  struct flags f;
};

struct mixed_sign {
  int s : 3;
  unsigned u : 5;
  unsigned wide : 12;
};

struct with_fn_ptr {
  void (*fn)(void);
  unsigned flag : 1;
  unsigned kind : 3;
  int n;
};

static struct flags g = {2, 1, 5, 7, 0};

int main(void) {
  assert(sizeof(struct flags) == 12);
  assert(offsetof(struct flags, tag) == 0);
  assert(offsetof(struct flags, x) == 4);
  assert(offsetof(struct outer, f) == 4);
  assert(sizeof(struct mixed_sign) == 4);

  assert(g.tag == 2 && g.a == 1 && g.b == 5 && g.x == 7 && g.c == 0);

  struct flags f;
  memset(&f, 0, sizeof(f));
  assert(f.tag == 0 && f.a == 0 && f.b == 0 && f.x == 0 && f.c == 0);

  f.a = 1;
  assert(f.a == 1 && f.b == 0 && f.c == 0 && f.tag == 0 && f.x == 0);

  f.b = 5;
  assert(f.b == 5 && f.a == 1);

  f.tag = 0xFF;
  assert(f.tag == 0xFF && f.a == 1 && f.b == 5);

  f.b++;
  assert(f.b == 6 && f.a == 1);
  f.b += 1;
  assert(f.b == 7 && f.a == 1);

  f.c = 1;
  assert(f.c == 1 && f.a == 1 && f.b == 7);

  f.x = -3;
  assert(f.x == -3 && f.a == 1 && f.b == 7 && f.c == 1 && f.tag == 0xFF);

  int *px = &f.x;
  *px = 42;
  assert(f.x == 42 && f.b == 7 && f.c == 1);

  unsigned char raw[sizeof(struct flags)];
  memset(&f, 0, sizeof(f));
  f.b = 7;
  memcpy(raw, &f, sizeof(f));
  assert(raw[0] == 0x00);
  assert(raw[1] == 0x0E);

  struct flags copy = f;
  assert(copy.b == 7 && copy.a == 0 && copy.tag == 0);

  struct flags dup;
  memcpy(&dup, &f, sizeof(f));
  assert(dup.b == 7 && dup.a == 0 && dup.tag == 0);

  struct mixed_sign m;
  memset(&m, 0, sizeof(m));
  m.s = -4;
  assert(m.s == -4);
  m.s = 3;
  assert(m.s == 3);
  m.u = 31;
  assert(m.u == 31 && m.s == 3);
  m.wide = 0xABC;
  assert(m.wide == 0xABC && m.u == 31 && m.s == 3);
  m.s = -1;
  assert(m.s == -1 && m.u == 31 && m.wide == 0xABC);

  m.s = 1;
  m.s -= 3;
  assert(m.s == -2 && m.u == 31);

  f.a = 1;
  f.b = 5;
  assert((!f.a) == 0);
  assert((!f.c) == 1);
  assert(~f.b == ~5);
  assert(-f.b == -5);
  assert(f.b == 5);
  if (f.b) {
    assert(f.b == 5);
  }

  unsigned char step = 2;
  f.b = 1;
  f.b += step;
  assert(f.b == 3);
  f.b <<= 1;
  assert(f.b == 6);
  f.b &= ~1u;
  assert(f.b == 6);
  f.b -= g.tag;
  assert(f.b == 4 && f.a == 1);

  int t = (f.b = 3);
  assert(t == 3 && f.b == 3);
  int u = f.b++;
  assert(u == 3 && f.b == 4);
  int v = ++f.b;
  assert(v == 5 && f.b == 5);

  struct with_fn_ptr w;
  memset(&w, 0, sizeof(w));
  assert(w.fn == 0 && w.flag == 0 && w.kind == 0 && w.n == 0);
  w.flag = 1;
  w.kind = 5;
  w.n = -7;
  assert(w.flag == 1 && w.kind == 5 && w.n == -7);
  assert(offsetof(struct with_fn_ptr, n) == sizeof(void *) + 4);

  return 0;
}
