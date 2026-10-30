// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <sys/ipc.h>
#include <sys/sem.h>
#include <sys/types.h>

typedef key_t t1;
typedef struct sembuf t2;
typedef struct semid_ds t3;

#ifdef __GLIBC__
typedef __key_t t4;
#endif

int f1(key_t key, int nsems, int semflg) { return semget(key, nsems, semflg); }

int f2(int semid, struct sembuf *sops, size_t nsops) {
  return semop(semid, sops, nsops);
}

int (*f3)(int, int, int, ...) = semctl;
