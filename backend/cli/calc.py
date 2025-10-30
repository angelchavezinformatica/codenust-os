"""Calculator"""

from typing import List
from programs.calc import CalculatorProcess
from kernel.process_table import process_table


async def calc_program(_command: str, args: List[str], _ctx: dict):
    """Handle calc program execution."""

    if not args:
        return """Usage: calc <expression>
Example: calc 5 + 3
Flags:
  --daemon   Run process in background"""

    is_daemon = "--daemon" in args

    if not is_daemon:
        calc = CalculatorProcess()
        process_table.add_process(calc)
        exp = ' '.join(args)
        result = await calc.calculate(exp)
        process_table.remove_process(calc)
        return f"""Result: {result}
[log - info] Process {calc.pid} finished."""

    calc = CalculatorProcess()
    process_table.add_process(calc)
    return f"Process {calc.pid} running in background (daemon mode)."
