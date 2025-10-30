"""echo Command"""

from typing import List


async def echo_command(_command: str, args: List[str], _ctx: dict):
    """Proccess echo command"""

    return ' '.join(args)
