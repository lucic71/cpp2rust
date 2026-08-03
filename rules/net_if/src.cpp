// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <net/if.h>

unsigned int f1(const char *ifname) { return if_nametoindex(ifname); }

int f2(void) { return IFF_UP; }
int f3(void) { return IFF_LOOPBACK; }
