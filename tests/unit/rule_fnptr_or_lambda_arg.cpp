#include <algorithm>
#include <assert.h>
#include <vector>

struct Item {
  int key;
  int value;
};

static bool CompareItem(const Item &a, const Item &b) { return a.key < b.key; }

static bool CompareInt(int a, int b) { return a > b; }

int main() {
  std::vector<Item> v;
  v.push_back({3, 30});
  v.push_back({1, 10});
  v.push_back({2, 20});

  std::stable_sort(v.begin(), v.end(), CompareItem);
  assert(v[0].key == 1);
  std::stable_sort(v.begin(), v.end(),
                   [](const Item &a, const Item &b) { return a.key > b.key; });
  assert(v[0].key == 3);

  int arr[] = {5, 2, 8, 1, 3};
  std::stable_sort(arr, arr + 5, CompareInt);
  assert(arr[0] == 8);
  std::stable_sort(arr, arr + 5, [](int x, int y) { return x < y; });
  assert(arr[0] == 1);

  return 0;
}
