#include <assert.h>
#include <stdlib.h>
#include <string.h>

struct entry {
  union {
    struct {
      char **elem;
    } set;
    int other;
  } c;
};

struct holder {
  struct entry *table;
};

int main(void) {
  struct holder *h = malloc(sizeof(struct holder));
  h->table = malloc(sizeof(struct entry));
  h->table[0].c.set.elem = malloc(sizeof(char *));
  h->table[0].c.set.elem[0] = strdup("alpha");
  assert(strcmp(h->table[0].c.set.elem[0], "alpha") == 0);

  free(h->table[0].c.set.elem[0]);
  h->table[0].c.set.elem[0] = NULL;
  assert(h->table[0].c.set.elem[0] == NULL);

  free(h->table[0].c.set.elem);
  h->table[0].c.set.elem = NULL;
  assert(h->table[0].c.set.elem == NULL);

  free(h->table);
  free(h);
  return 0;
}
