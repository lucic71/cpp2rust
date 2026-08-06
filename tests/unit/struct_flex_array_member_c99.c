#include <assert.h>
#include <stddef.h>
#include <stdlib.h>
#include <string.h>

struct entry {
  int id;
  int weight;
};

struct table {
  int n;
  struct entry a[];
};

static struct table *table_create(int n) {
  void *raw = malloc(offsetof(struct table, a) + n * sizeof(struct entry));
  struct table *t = (struct table *)raw;
  t->n = n;
  for (int i = 0; i < n; i++) {
    t->a[i].id = i * 10;
    t->a[i].weight = i + 1;
  }
  return t;
}

int main(void) {
  assert(sizeof(struct table) == offsetof(struct table, a));

  struct table *t = table_create(3);
  assert(t->n == 3);
  assert(t->a[0].id == 0);
  assert(t->a[2].id == 20);
  assert(t->a[2].weight == 3);

  t->a[1].id = 99;
  assert(t->a[1].id == 99);
  assert(t->a[0].id == 0);

  struct table *next = 0;
  assert(next == 0);

  free(t);
  return 0;
}
