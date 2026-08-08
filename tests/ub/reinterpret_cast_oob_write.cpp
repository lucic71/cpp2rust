// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// panic-ub: refcount
#include <cstdint>

// Write past the end of a u32 viewed as u8
int main() {
  uint32_t val = 0x04030201;
  uint8_t *bytes = reinterpret_cast<uint8_t *>(&val);
  bytes[4] = 0xFF;
  return 0;
}
