"""Cat command"""

import os
from typing import List

PATH = os.path.join(os.getcwd(), "filesystem")

USAGE = """Usage:
  cat [file]

Description:
  Displays the content of a file inside the virtual filesystem.

Examples:
  cat README.md
  cat /etc/hosts
  cat ~/notes.txt
"""


async def cat_command(_command: str, args: List[str], ctx: dict):
    """Process cat command"""

    if not args or args[0] in {"--help", "-h"}:
        return USAGE

    current_path = ctx.get("path", "/")
    target = args[0]

    # Si es ruta absoluta, usarla directamente
    if target.startswith("/"):
        virtual_path = target
    else:
        virtual_path = os.path.join(current_path, target)

    # Normalizar
    normalized_virtual_path = os.path.normpath(virtual_path)
    if not normalized_virtual_path.startswith("/"):
        normalized_virtual_path = "/" + normalized_virtual_path

    # Resolver ruta física
    relative_path = normalized_virtual_path.strip("/")
    real_path = os.path.join(PATH, relative_path)

    # 🔒 Evitar salir del sandbox
    if not os.path.abspath(real_path).startswith(PATH):
        return "cat: access denied (out of sandbox)"

    # Validar archivo
    if not os.path.exists(real_path):
        return f"cat: no such file: {target}"

    if not os.path.isfile(real_path):
        return f"cat: {target}: is a directory"

    # Leer contenido
    try:
        with open(real_path, "r", encoding="utf-8") as file:
            content = file.read()
        return content
    except Exception as e:
        return f"cat: error reading file: {e}"
