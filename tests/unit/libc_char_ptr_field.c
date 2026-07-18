#include <dirent.h>
#include <pwd.h>
#include <unistd.h>

int main(void) {
  struct passwd *pw = getpwuid(geteuid());
  if (!pw) {
    return 0;
  }
  char *home = pw->pw_dir;

  struct dirent *d = readdir(opendir("/tmp"));
  // d_name is a char array which gets translated as i8. We model chars as u8 in
  // Rust.
  char *dname = d->d_name;

  return 0;
}
