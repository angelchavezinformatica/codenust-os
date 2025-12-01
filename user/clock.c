#include "kernel/types.h"
#include "user.h"

int
main(void)
{
  printf("lamport: %d\n", lamport_time());
  exit(0);
}
