#include "kernel/types.h"
#include "kernel/stat.h"
#include "kernel/fcntl.h"
#include "user.h"

int main()
{
  int p[2];
  pipe(p);

  if(fork() == 0){
    // Child receiver
    close(p[1]);

    int msg_clock;
    read(p[0], &msg_clock, sizeof(msg_clock));

    lamport_recv(msg_clock);
    printf("Receiver: clock = %d\n", lamport_time());

    exit(0);
  }

  // Parent sender
  close(p[0]);

  int ts = lamport_tick();   // evento de envío
  write(p[1], &ts, sizeof(ts));

  wait(0);
  exit(0);
}
