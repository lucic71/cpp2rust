#include <map>

int main() {
  std::map<int, int> m;
  m[0] = 1;
  std::map<int, int>::iterator end = m.end();
  std::map<int, int>::const_iterator const_it = m.find(0);
  return const_it == end ? 0 : 1;
}
