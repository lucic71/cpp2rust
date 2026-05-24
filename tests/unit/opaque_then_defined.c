#include <assert.h>

struct node;

struct list {
  struct node *head;
  int size;
};

struct node {
  int value;
  struct node *next;
};

int main(void) {
  struct node n = {42, 0};
  struct list l = {&n, 1};
  assert(l.head->value == 42);
  assert(l.size == 1);
  return 0;
}
