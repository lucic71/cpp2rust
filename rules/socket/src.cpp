#include <sys/types.h>
#include <sys/socket.h>

int f1() {
  return MSG_NOSIGNAL;
}

int f2() {
  return SOCK_STREAM;
}

int f3() {
  return SOCK_DGRAM;
}

int f4() {
  return SOCK_RAW;
}

int f5() {
  return SOCK_RDM;
}

int f6() {
  return SOCK_SEQPACKET;
}

int f7() {
  return SOCK_DCCP;
}

int f8() {
  return SOCK_PACKET;
}

int f9() {
  return SOCK_CLOEXEC;
}

int f10() {
  return SOCK_NONBLOCK;
}
