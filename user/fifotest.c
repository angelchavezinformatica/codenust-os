#include "kernel/types.h"
#include "kernel/fcntl.h"
#include "user/user.h"

struct msg {
  int lamport;
  int payload;
};

int
main(void)
{
  mkfifo("canal");

  int pid = fork();
  if(pid == 0){
    // writer
    int w = open("canal", O_WRONLY);

    for(int i = 0; i < 5; i++){
      lamport_tick();  // evento local antes de enviar

      struct msg m;
      m.lamport = lamport_time();  // clock del proceso writer
      m.payload = i;

      printf("writer (L=%d): sending %d\n", m.lamport, m.payload);
      write(w, &m, sizeof(m));

      sleep(30); // simula trabajo
    }

    close(w);
    exit(0);
  }

  // reader
  int r = open("canal", O_RDONLY);

  for(int i = 0; i < 5; i++){
    struct msg m;
    read(r, &m, sizeof(m));

    // sincroniza el reloj con el mensaje recibido
    lamport_recv(m.lamport);

    printf("reader (L=%d): got %d\n", lamport_time(), m.payload);
  }

  close(r);

  wait(0);
  exit(0);
}
