"""Mailbox command"""

from typing import List
from programs.mailbox import mailbox_handler


USAGE = """Usage:
  mailbox create <name>
  mailbox list
  mailbox delete <name>
  mailbox send <from> <to> <message>
  mailbox send-to-process <mailbox> <pid> <message>
  mailbox read <mailbox>
"""


async def mailbox_command(_command: str, args: List[str], _ctx: dict):
    """Process mailbox command"""

    if not args:
        return USAGE

    subcommand = args[0]

    try:
        match subcommand:
            case "create":
                if len(args) < 2:
                    return "Error: missing mailbox name"
                name = args[1]
                new_mailbox = mailbox_handler.create_mailbox(name)
                return f"Mailbox '{name}' created with PID: {new_mailbox.pid}"

            case "list":
                mailbox_list = mailbox_handler.list()
                if not mailbox_list:
                    return "No mailboxes found."
                return "\n".join(
                    f"[PID: {m.pid}] {m.mailbox_name}" for m in mailbox_list
                )

            case "delete":
                if len(args) < 2:
                    return "Error: missing mailbox name"
                name = args[1]
                mailbox_handler.delete(name)
                return f"Mailbox '{name}' deleted."

            case "send":
                if len(args) < 4:
                    return "Usage: mailbox send <from> <to> <message>"
                mb_from, mb_to = args[1], args[2]
                message = " ".join(args[3:])
                await mailbox_handler.send(mb_from, mb_to, message)
                return "Message sent."

            case "send-to-process":
                if len(args) < 4:
                    return "Usage: mailbox send-to-process <mailbox> <pid> <message>"
                mailbox, process_pid = args[1], args[2]
                message = " ".join(args[3:])
                await mailbox_handler.send_message_to_process(mailbox, process_pid, message)
                return "Message sent to process."

            case "read":
                if len(args) < 2:
                    return "Error: missing mailbox name"
                mailbox = args[1]
                messages = mailbox_handler.read(mailbox)
                return messages or "No messages."

            case _:
                return f"Unknown subcommand: {subcommand}"

    except ValueError as e:
        return f"Error: {e}"
