from datetime import datetime
from typing import Dict, Any


class TimestampService:
    """时间戳服务类，提供时间戳转换功能"""

    @staticmethod
    def convert_timestamp(timestamp: int, action: str) -> Dict[str, Any]:
        """
        转换时间戳

        Args:
            timestamp: 时间戳
            action: 转换类型 ('to_datetime' 或 'to_timestamp')

        Returns:
            包含转换结果的字典
        """
        if action == "to_datetime":
            dt = datetime.fromtimestamp(timestamp)
            return {
                "success": True,
                "result": dt.strftime("%Y-%m-%d %H:%M:%S"),
                "timestamp": timestamp,
                "action": action
            }
        elif action == "to_timestamp":
            return {
                "success": True,
                "result": str(timestamp),
                "timestamp": timestamp,
                "action": action
            }
        else:
            raise ValueError("不支持的操作类型")
