"""Create base filesystem"""


import os
from typing import Dict, Any

FAKE_FS: Dict[str, Any] = {
    "home": {
        "user": {
            "README.md": "Bienvenido a Codenus OS",
            "projects": {
                "os_sim": {
                    "main.py": "print('os')",
                    "README": "Proyecto OS"
                }
            },
            "notes.txt": "ToDo:\n - terminar kernel"
        }
    },
    "etc": {
        "hosts": "127.0.0.1 localhost",
    },
    "var": {
        "log": {
            "sys.log": "init..."
        }
    },
    "tmp": {}
}


def create_filesystem(node: Dict[str, Any], path: str) -> None:
    """
    Crea en disco la estructura representada por `node` bajo la ruta `path`.
    - Si el valor es un dict -> crea directorio y recursa.
    - Si el valor es una str  -> crea/reescribe archivo con ese contenido.
    """
    os.makedirs(path, exist_ok=True)

    for name, content in node.items():
        target = os.path.join(path, name)

        if isinstance(content, dict):
            create_filesystem(content, target)
        else:
            parent = os.path.dirname(target)
            if parent:
                os.makedirs(parent, exist_ok=True)
            with open(target, "w", encoding="utf-8") as f:
                f.write(str(content))


if __name__ == "__main__":
    out_dir = os.path.join(os.getcwd(), 'filesystem')

    if not os.path.isdir(out_dir):
        os.mkdir(out_dir)

    create_filesystem(FAKE_FS, out_dir)
