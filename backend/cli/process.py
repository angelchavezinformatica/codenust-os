"""process Command"""

from typing import List
from kernel.process_table import process_table


async def process_command(_command: str, _args: List[str], _ctx: dict):
    """process process command"""

    result = f'#Num. of processes: {len(process_table.processes)}'

    for process in process_table.processes:
        result += '\n'
        result += f'[PID: {process.pid}] {process.name} '
        result += f'({'running' if process.running else 'stop'})'

    return result
