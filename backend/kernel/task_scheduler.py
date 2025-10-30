"""Task Scheduler"""

import threading
import asyncio
from typing import List, Callable, Awaitable


class Task:
    """Task wrapper"""

    def __init__(self, task: Callable[[], Awaitable[None]]):
        self.task = task


class TaskScheduler:
    """Task runner"""

    def __init__(self):
        self.tasks: List[Task] = []
        self.running = True

    def add_task(self, task: Task):
        """Add new task to TaskScheduler"""
        self.tasks.append(task)

    async def run(self):
        """Loop runner"""
        while self.running:
            if self.tasks:
                task = self.tasks.pop(0)
                await task.task()
            await asyncio.sleep(0.5)


task_scheduler = TaskScheduler()


def start_scheduler_thread():
    """Run task scheduler"""

    loop = asyncio.new_event_loop()
    asyncio.set_event_loop(loop)
    loop.run_until_complete(task_scheduler.run())


thread = threading.Thread(target=start_scheduler_thread, daemon=True)
thread.start()
