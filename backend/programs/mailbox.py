"""Mailbox Program"""

from typing import List
from kernel.process_table import Process, process_table
from kernel.message_bus import message_bus


class MailboxMessage:
    """Mailbox Message"""

    def __init__(self, pid: int, message: any):
        self.pid = pid
        self.message = message

    def read(self):
        """Read message"""

        return f'[Message from PID: {self.pid}] {self.message}'


class MailboxProcess(Process):
    """Mailbox Process"""

    def __init__(self):
        super().__init__()
        self.running = True
        self.name = 'Mailbox'
        self.mailbox_name: str | None = None
        self.messages: List[MailboxMessage] = []

    async def run(self):
        """Main loop"""

    async def handle_event(self, event_name: str, data: dict):
        """Main loop"""

        new_message = MailboxMessage(data.get('sender'), data.get('message'))

        self.messages.append(new_message)

    def create_message(self, pid: int, message: str):
        """Create message"""

        message = MailboxMessage(pid, message)
        self.messages.append(message)

        return message


class Mailbox(Process):
    """Mailbox handler"""

    def __init__(self):
        super().__init__()
        self.running = True
        self.name = 'Mailbox handler'
        self.mailboxes: List[MailboxProcess] = []

    async def run(self):
        """Main loop"""

    async def handle_event(self, event_name: str, data: dict):
        """Handle event"""

    def create_mailbox(self, name: str):
        """Create mailbox"""

        for mailbox in self.mailboxes:
            if mailbox.mailbox_name == name:
                raise ValueError('Mailbox already exists')

        new_mailbox = MailboxProcess()
        new_mailbox.mailbox_name = name
        process_table.add_process(new_mailbox)

        message_bus.subscribe(str(new_mailbox.pid), new_mailbox)

        self.mailboxes.append(new_mailbox)

        return new_mailbox

    def list(self):
        """List mailboxes"""

        return self.mailboxes

    def delete(self, name: str):
        """Delete mailbox"""

        for mailbox in self.mailboxes:
            if mailbox.mailbox_name == name:
                self.mailboxes.remove(mailbox)
                process_table.remove_process(mailbox)
                message_bus.unsubscribe(str(mailbox.pid), mailbox)
                break

    async def send(self, mailbox_from: str, mailbox_to: str, message: str):
        """Send message from mailbox to another mailbox"""
        mb_from = self.get_mailbox(mailbox_from)
        mb_to = self.get_mailbox(mailbox_to)

        mb_to.create_message(mb_from.pid, message)

    async def send_message_to_process(self, mailbox_name: str, process_pid: str, message: str):
        """Send message from mailbox to process"""

        mailbox = self.get_mailbox(mailbox_name)

        await message_bus.publish(process_pid, {'message': message, 'sender': str(mailbox.pid)})

    def read(self, mailbox_name: str):
        """Read messages of mailbox"""
        mailbox = self.get_mailbox(mailbox_name)

        result = ''

        for message in mailbox.messages:
            result += f'{message.read()}\n'

        return result

    def get_mailbox(self, mailbox_name: str):
        """Return mailbox"""

        for mb in self.mailboxes:
            if mb.mailbox_name == mailbox_name:
                return mb

        raise ValueError('Mailbox not found')


mailbox_handler = Mailbox()

process_table.add_process(mailbox_handler)
