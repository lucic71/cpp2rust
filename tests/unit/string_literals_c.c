void foo(char *str) {}

int main() {
  char *mutable_strings[] = {"a", "b", "c"};
  const char *immutable_strings[] = {"a", "b", "c"};

  char *mutable_string = "hello";
  const char *immutable_string = "hello";

  foo("world");
  foo(mutable_string);
  // Calling foo with immutable_string is an error
  return 0;
}
