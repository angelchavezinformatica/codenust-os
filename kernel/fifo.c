#include <stdarg.h>

#include "types.h"
#include "param.h"
#include "spinlock.h"
#include "sleeplock.h"
#include "fs.h"
#include "file.h"
#include "memlayout.h"
#include "riscv.h"
#include "defs.h"
#include "proc.h"

#define FIFO_BUF 512

static struct {
  struct spinlock lock;
  char buf[FIFO_BUF];
  int r;
  int w;
  int size;
} fifodev;

int
fifo_read(int user_dst, uint64 dst, int n)
{
  int i;
  acquire(&fifodev.lock);

  for(i = 0; i < n; i++){
    while(fifodev.size == 0){
      sleep(&fifodev.r, &fifodev.lock);
    }

    char c = fifodev.buf[fifodev.r];
    fifodev.r = (fifodev.r + 1) % FIFO_BUF;
    fifodev.size--;

    if(copyout(myproc()->pagetable, dst+i, &c, 1) < 0){
      release(&fifodev.lock);
      return -1;
    }

    wakeup(&fifodev.w);
  }

  release(&fifodev.lock);
  return i;
}

int
fifo_write(int user_src, uint64 src, int n)
{
  int i;
  acquire(&fifodev.lock);

  for(i = 0; i < n; i++){
    while(fifodev.size == FIFO_BUF){
      sleep(&fifodev.w, &fifodev.lock);
    }

    char c;
    if(copyin(myproc()->pagetable, &c, src+i, 1) < 0){
      release(&fifodev.lock);
      return -1;
    }

    fifodev.buf[fifodev.w] = c;
    fifodev.w = (fifodev.w + 1) % FIFO_BUF;
    fifodev.size++;

    wakeup(&fifodev.r);
  }

  release(&fifodev.lock);
  return i;
}

void
fifo_init(void)
{
  initlock(&fifodev.lock, "fifo");
  fifodev.r = fifodev.w = fifodev.size = 0;
  devsw[FIFO].read = fifo_read;
  devsw[FIFO].write = fifo_write;
}
