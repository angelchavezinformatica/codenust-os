"""Message Bus"""

from .process_table import Process


class MessageBus:
    """Message Bus"""

    def __init__(self):
        self.subscribers = {}

    def subscribe(self, event_name: str, process: Process):
        """Suscribe process"""
        self.subscribers.setdefault(event_name, []).append(process)

    async def publish(self, event_name: str, data: dict):
        """Publish event"""
        for process in self.subscribers.get(event_name, []):
            await process.handle_event(event_name, data)


message_bus = MessageBus()
