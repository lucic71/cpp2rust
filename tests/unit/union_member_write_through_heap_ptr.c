#include <assert.h>
#include <stdlib.h>
#include <string.h>

struct entry {
  int kind;
  union {
    struct {
      char **elem;
      long size;
      long idx;
    } set;
    struct {
      int min;
      int max;
    } range;
  } c;
};

int main(void) {
  struct entry *table = malloc(2 * sizeof(struct entry));
  assert(table != NULL);
  memset(table, 0, 2 * sizeof(struct entry));

  struct entry *e = &table[0];
  e->kind = 1;
  e->c.set.size = 7;
  e->c.set.idx = 3;
  e->c.set.elem = malloc(sizeof(char *));
  assert(e->c.set.elem != NULL);

  e->c.set.elem[0] = strdup("alpha");
  assert(e->c.set.elem[0] != NULL);
  assert(strcmp(e->c.set.elem[0], "alpha") == 0);
  assert(e->c.set.size == 7);
  assert(e->c.set.idx == 3);
  assert(e->kind == 1);

  e = &table[1];
  e->kind = 2;
  e->c.range.min = 10;
  e->c.range.max = 20;
  assert(e->c.range.min == 10);
  assert(e->c.range.max == 20);
  assert(table[0].c.set.size == 7);

  free(table[0].c.set.elem[0]);
  free(table[0].c.set.elem);
  free(table);
  return 0;
}
