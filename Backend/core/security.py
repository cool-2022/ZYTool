import time
from collections import defaultdict
from fastapi import Request, HTTPException
from starlette.middleware.base import BaseHTTPMiddleware
from typing import Dict, Tuple
from core import settings, get_logger

logger = get_logger(__name__)


class RateLimitMiddleware(BaseHTTPMiddleware):
    
    def __init__(self, app):
        super().__init__(app)
        self.requests: Dict[str, list] = defaultdict(list)
        self.rate_limit_enabled = settings.rate_limit_enabled
        self.max_requests = settings.rate_limit_requests
        self.period = settings.rate_limit_period
    
    def _is_rate_limited(self, client_id: str) -> Tuple[bool, int]:
        if not self.rate_limit_enabled:
            return False, 0
        
        current_time = time.time()
        request_times = self.requests[client_id]
        
        request_times[:] = [t for t in request_times if current_time - t < self.period]
        
        if len(request_times) >= self.max_requests:
            return True, int(request_times[0] + self.period - current_time)
        
        request_times.append(current_time)
        return False, 0
    
    async def dispatch(self, request: Request, call_next):
        client_id = request.client.host if request.client else "unknown"
        
        is_limited, retry_after = self._is_rate_limited(client_id)
        
        if is_limited:
            logger.warning(f"Rate limit exceeded for client: {client_id}")
            raise HTTPException(
                status_code=429,
                detail=f"请求过于频繁，请在 {retry_after} 秒后重试",
                headers={"Retry-After": str(retry_after)}
            )
        
        response = await call_next(request)
        return response
