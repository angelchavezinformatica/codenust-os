#include "types.h"
#include "param.h"
#include "memlayout.h"
#include "riscv.h"
#include "defs.h"

volatile static int started = 0;

// start() jumps here in supervisor mode on all CPUs.
void
main()
{
  if(cpuid() == 0){
    consoleinit();
    printfinit();
    kinit();         // physical page allocator
    kvminit();       // create kernel page table
    kvminithart();   // turn on paging
    procinit();      // process table
    trapinit();      // trap vectors
    trapinithart();  // install kernel trap vector
    plicinit();      // set up interrupt controller
    plicinithart();  // ask PLIC for device interrupts
    binit();         // buffer cache
    iinit();         // inode table
    fileinit();      // file table
    virtio_disk_init(); // emulated hard disk
    userinit();      // first user process
    fifo_init();     // init fifo
    __sync_synchronize();
    started = 1;
    printf("\x1b[2J\x1b[H");
    printf("   _____          _                      _    ____   _____ \n");
    printf("  / ____|        | |                    | |  / __ \\ / ____|\n");
    printf(" | |     ___   __| | ___ _ __  _   _ ___| |_| |  | | (___  \n");
    printf(" | |    / _ \\ / _` |/ _ \\ '_ \\| | | / __| __| |  | |\\___ \\ \n");
    printf(" | |___| (_) | (_| |  __/ | | | |_| \\__ \\ |_| |__| |____) |\n");
    printf("  \\_____\\___/ \\__,_|\\___|_| |_|\\__,_|___/\\__|\\____/|_____/ \n");
    printf("\n");
  } else {
    while(started == 0)
      ;
    __sync_synchronize();
    kvminithart();    // turn on paging
    trapinithart();   // install kernel trap vector
    plicinithart();   // ask PLIC for device interrupts
  }

  scheduler();        
}
