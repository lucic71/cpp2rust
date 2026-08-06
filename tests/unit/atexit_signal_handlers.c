#include <assert.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>

static int got = 0;

static void on_signal(int sig) { got = sig; }

static void first_exit(void) { printf("first\n"); }

static void second_exit(void) { printf("second\n"); }

int main(void) {
  assert(signal(SIGUSR1, on_signal) == SIG_DFL);
  assert(raise(SIGUSR1) == 0);
  assert(got == SIGUSR1);

  got = 0;
  void (*prev)(int) = signal(SIGUSR1, SIG_IGN);
  assert(prev == on_signal);
  assert(raise(SIGUSR1) == 0);
  assert(got == 0);

  assert(atexit(first_exit) == 0);
  assert(atexit(second_exit) == 0);
  printf("main\n");
  return 0;
}
