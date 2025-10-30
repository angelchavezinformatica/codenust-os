"""Codenust OS CLI"""

import shlex

from typing import List

from .calc import calc_program
from .echo import echo_command
from .whoami import whoami_command
from .set_user import set_user_command

context = {
    "user": 'user',
    "path": '/home/user',
}


actions = {
    "calc": calc_program,
    "echo": echo_command,
    "whoami": whoami_command,
    "set-user": set_user_command,
}


async def cli(value: str):
    """CLI"""

    args_splited = shlex.split(value)

    command = args_splited[0] if args_splited else None
    args = args_splited[1:]

    action = actions.get(command)

    if not action:
        return {
            "type": 'message',
            "path": context.get('path'),
            "data": f'Command not found: {value}'
        }

    response = await action(command, args, context)

    return {
        "type": 'message',
        "path": context.get('path'),
        "data": response
    }
