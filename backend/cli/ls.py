"""FileSystem commands"""

import os
from typing import List

PATH = os.path.join(os.getcwd(), "filesystem")

USAGE = """Usage:
  ls [path]

Description:
  Lists the contents of the specified directory.
  If no path is given, lists the current directory.
  Supports both absolute and relative paths.

Examples:
  ls
  ls projects
  ls /etc
"""


async def ls_command(_command: str, _args: List[str], ctx: dict):
    """Process ls command"""

    if _args and _args[0] in {"--help", "-h"}:
        return USAGE

    current_path = ctx.get("path", "/")

    if _args:
        target = _args[0]
        if not target.startswith("/"):
            current_path = os.path.join(current_path, target)
        else:
            current_path = target

    relative_path = current_path.strip("/")
    real_path = os.path.normpath(os.path.join(PATH, relative_path))

    if not real_path.startswith(PATH):
        return "ls: access denied (out of sandbox)"

    if not os.path.exists(real_path):
        return f"ls: cannot access '{current_path}': No such file or directory"

    if os.path.isfile(real_path):
        return os.path.basename(real_path)

    entries = sorted(os.listdir(real_path))
    return "\n".join(entries) if entries else "(empty)"
