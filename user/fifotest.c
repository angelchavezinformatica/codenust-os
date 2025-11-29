#include "kernel/types.h"
#include "kernel/fcntl.h"
#include "user.h"

int
main(void)
{
  mkfifo("canal");

  int pid = fork();
  if(pid == 0){
    // writer
    int w = open("canal", O_WRONLY);
    for(int i = 0; i < 5; i++){
      printf("writer: sending %d\n", i);
      write(w, &i, sizeof(i));
      sleep(30); // simula trabajo
    }
    close(w);
    exit(0);
  }

  // reader
  int r = open("canal", O_RDONLY);
  for(int i = 0; i < 5; i++){
    int v;
    read(r, &v, sizeof(v));
    printf("reader: got %d\n", v);
  }
  close(r);

  wait(0);
  exit(0);
}
