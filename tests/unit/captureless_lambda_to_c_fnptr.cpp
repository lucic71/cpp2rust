#include <assert.h>
#include <stdlib.h>

int main() {
  int arr[] = {5, 2, 8, 1, 3};
  qsort(arr, 5, sizeof(int), [](const void *a, const void *b) {
    return *static_cast<const int *>(b) - *static_cast<const int *>(a);
  });
  assert(arr[0] == 8);
  return 0;
}
