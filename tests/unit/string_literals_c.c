void foo(char *str) {}

int main() {
  // warning: ISO C++11 does not allow conversion from string literal to 'char *' [-Wwritable-strings]
  char *mutable_strings[] = {"a", "b", "c"};
  const char *immutable_strings[] = {"a", "b", "c"};

  char *mutable_string = "hello";
  const char *immutable_string = "hello";

  foo("world");
  foo(mutable_string);
  // This is not allowed, only string literals to char* are allowed: foo(immutable_string);
  return 0;
}
