#pragma once

struct opaque;

struct container {
  struct opaque *p;
  int x;
};

void touch(struct container *c);
