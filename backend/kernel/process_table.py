"""Process Table"""

import asyncio

from abc import ABC, abstractmethod
from typing import List

from .task_scheduler import task_scheduler, Task


class Process(ABC):
    """Process"""

    pid: int | None = None
    running: bool = True
    name: str | None = None

    @abstractmethod
    async def run(self):
        """Should be implemented"""

    @abstractmethod
    async def handle_event(self, event_name: str, data: dict):
        """Should be implemented"""

    def create_task(self, task: object):
        """Create async subtask"""
        task_scheduler.add_task(Task(task))


class ProcessTable:
    """Process Table"""

    def __init__(self):
        self.counter: int = 0
        self.processes: List[Process] = []

    def add_process(self, process: Process):
        """Add new process"""

        self.counter += 1
        process.pid = self.counter
        self.processes.append(process)

        async def process_runner():
            while process.running:
                await process.run()
                await asyncio.sleep(0.1)

        task_scheduler.add_task(Task(process_runner))

    def remove_process(self, process: Process):
        """Remove process"""
        process.running = False
        if process in self.processes:
            self.processes.remove(process)


process_table = ProcessTable()
