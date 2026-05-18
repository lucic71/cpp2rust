#include <map>

int main() {
  std::map<int, int> m;
  m[0] = 1;
  std::map<int, int>::iterator end = m.end();
  std::map<int, int>::const_iterator const_it = m.find(0);
  // Comparing const_iterator with iterator forces an implicit conversion of
  // `end` to const_iterator. The AST shape differs between platforms:
  //
  //   Linux: const_it == ConvertingCtor(end)
  //   macOS: const_it == ConvertingCtor(CopyCtor(end))
  //
  // The extra inner CopyCtor on macOS would emit a redundant .clone() in the
  // generated Rust. cpp2rust suppresses it so the output matches Linux.
  return const_it == end ? 0 : 1;
}
