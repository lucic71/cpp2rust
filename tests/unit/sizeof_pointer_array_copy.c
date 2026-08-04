#include <assert.h>
#include <stdlib.h>
#include <string.h>

static const char *names[] = {"alpha", "beta", "gamma", NULL};

int main(void) {
  size_t count = 0;
  const char *const *walk;
  for (walk = names; *walk; walk++) {
    ++count;
  }
  assert(count == 3);
  assert(sizeof(names) == sizeof(*names) * 4);

  const char **copy = malloc(sizeof(*names) * (count + 1));
  assert(copy != NULL);
  memcpy((void *)copy, names, sizeof(*names) * count);
  copy[count] = NULL;

  assert(strcmp(copy[0], "alpha") == 0);
  assert(strcmp(copy[1], "beta") == 0);
  assert(strcmp(copy[2], "gamma") == 0);
  assert(copy[3] == NULL);

  count = 0;
  for (walk = copy; *walk; walk++) {
    ++count;
  }
  assert(count == 3);

  free(copy);
  return 0;
}
