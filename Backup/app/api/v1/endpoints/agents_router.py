
from fastapi import APIRouter, HTTPException
from fastapi.responses import StreamingResponse
from app.schemas import ChatRequest, ChatResponse


from app.services.agents import generate_stream, generate_sync

router = APIRouter()


@router.post("/chat")
async def chat(req: ChatRequest):
    """流式对话接口，支持 Function Call"""
    if not req.message or not req.message.strip():
        raise HTTPException(status_code=400, detail="message is required")

    return StreamingResponse(
        generate_stream(req.message),
        media_type="text/event-stream",
        headers={
            "Cache-Control": "no-cache",
            "Connection": "keep-alive",
            "X-Accel-Buffering": "no",
        },
    )


@router.post("/chat/sync", response_model=ChatResponse)
async def chat_sync(req: ChatRequest):
    """同步对话接口，支持 Function Call"""
    if not req.message or not req.message.strip():
        raise HTTPException(status_code=400, detail="message is required")

    reply = generate_sync(req.message)
    return ChatResponse(reply=reply)
