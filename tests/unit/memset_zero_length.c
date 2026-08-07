#include <assert.h>
#include <string.h>

int main(void) {
  int arr[4] = {1, 2, 3, 4};
  int *end = arr + 4;

  memset(end, 0, 0);
  memcpy(end, arr, 0);

  assert(arr[0] == 1);
  assert(arr[3] == 4);
  return 0;
}
