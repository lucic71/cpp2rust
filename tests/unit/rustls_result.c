#include <stdio.h>

#include "../../rules/rustls/rustls.h"

static int to_code(rustls_result r) {
  switch (r) {
  case RUSTLS_RESULT_OK:
    return 0;
  case RUSTLS_RESULT_NULL_PARAMETER:
    return 1;
  default:
    return 2;
  }
}

static int classify(rustls_result r) {
  if (r == RUSTLS_RESULT_PLAINTEXT_EMPTY) {
    return 10;
  } else if (r == RUSTLS_RESULT_UNEXPECTED_EOF) {
    return 20;
  } else if (r != RUSTLS_RESULT_OK) {
    return 30;
  }
  return 0;
}

static void describe(rustls_result r, char *buf, size_t len, size_t *out_n) {
  size_t n = 0;
  if (r != RUSTLS_RESULT_OK && len > 0) {
    buf[n++] = '!';
  }
  *out_n = n;
}

static rustls_result init_zero(void) {
  rustls_result r = 0;
  return r;
}

static rustls_result reset(rustls_result r) {
  r = 0;
  return r;
}

static int is_unset(rustls_result r) { return r == 0; }

int main(void) {
  rustls_result r = RUSTLS_RESULT_OK;
  char buf[16];
  size_t n = 0;

  printf("%d\n", to_code(r));
  printf("%d\n", to_code(RUSTLS_RESULT_NULL_PARAMETER));
  printf("%d\n", classify(RUSTLS_RESULT_PLAINTEXT_EMPTY));
  printf("%d\n", classify(RUSTLS_RESULT_UNEXPECTED_EOF));
  printf("%d\n", classify(RUSTLS_RESULT_OK));

  describe(RUSTLS_RESULT_NULL_PARAMETER, buf, sizeof(buf), &n);
  printf("%d\n", (int)n);
  describe(RUSTLS_RESULT_OK, buf, sizeof(buf), &n);
  printf("%d\n", (int)n);

  printf("%d\n", is_unset(init_zero()));
  printf("%d\n", to_code(init_zero()));
  printf("%d\n", is_unset(reset(RUSTLS_RESULT_OK)));
  return 0;
}
