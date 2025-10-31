"""Change Directory command"""

import os
from typing import List

PATH = os.path.join(os.getcwd(), "filesystem")

USAGE = """Usage:
  cd [path]

Description:
  Changes the current working directory within the virtual filesystem.
  Supports relative and absolute paths.

Examples:
  cd projects
  cd /etc
  cd ..
  cd /
"""


async def cd_command(_command: str, _args: List[str], ctx: dict):
    """Process cd command"""

    if _args and _args[0] in {"--help", "-h"}:
        return USAGE

    current_path = ctx.get("path", "/")

    if not _args:
        ctx["path"] = "/"
        return ""

    target = _args[0]

    if target.startswith("/"):
        new_virtual_path = target
    else:
        new_virtual_path = os.path.join(current_path, target)

    normalized_virtual_path = os.path.normpath(new_virtual_path)

    if normalized_virtual_path == "." or normalized_virtual_path == "":
        normalized_virtual_path = "/"

    if not normalized_virtual_path.startswith("/"):
        normalized_virtual_path = "/" + normalized_virtual_path

    relative_path = normalized_virtual_path.strip("/")
    real_path = os.path.join(PATH, relative_path)

    if not os.path.abspath(real_path).startswith(PATH):
        return "cd: access denied (out of sandbox)"

    if not os.path.exists(real_path):
        return f"cd: no such file or directory: {target}"

    if not os.path.isdir(real_path):
        return f"cd: not a directory: {target}"

    ctx["path"] = normalized_virtual_path
    return ""
