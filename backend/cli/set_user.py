"""set-user Command"""

from typing import List


async def set_user_command(_command: str, args: List[str], ctx: dict):
    """proccess set-user command"""
    if not args:
        return "Usage: set-user <name>"

    ctx["user"] = args[0]
    ctx["path"] = f"/home/{ctx['user']}"

    return ctx["user"]
