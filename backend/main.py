"""Codenust OS"""

from fastapi import FastAPI, WebSocket
from starlette.websockets import WebSocketDisconnect


from cli import cli, context


app = FastAPI()


@app.websocket('/ws')
async def websocket_endpoint(websocket: WebSocket):
    """Codenust OS entry point"""
    await websocket.accept()

    await websocket.send_json({
        "type": 'init',
        "path": context['path'],
        "data": {
            "user": context['user'],
        }
    })

    try:
        while True:
            data = await websocket.receive_text()

            response = await cli(data)

            if data.startswith('set-user '):
                await websocket.send_json({
                    "type": 'init',
                    "path": context['path'],
                    "data": {
                        "user": context['user'],
                    }
                })

            await websocket.send_json(response)
    except WebSocketDisconnect:
        pass
