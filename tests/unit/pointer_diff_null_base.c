// panic-ub: refcount
// nondet-result: unsafe

static unsigned char buf[16];

int main(void) {
  const unsigned char *p = buf;
  // Alignment check written as a subtraction against a null pointer.
  return ((p - (const unsigned char *)0) & 7) == 0;
}
