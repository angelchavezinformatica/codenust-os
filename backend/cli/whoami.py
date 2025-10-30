"""whoami Command"""

from typing import List


async def whoami_command(_command: str, _args: List[str], ctx: dict):
    """Proccess whoami command"""

    return ctx.get('user')
