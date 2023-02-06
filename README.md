tracer 收集js报错日志并统计分析相关错误报错数据服务

http 服务框架: actix_web
数据库: rbatis
日志: tracing-subscriber

## 本地调试

### 创建本地环境
在根目录下面创建一个 `.env` 的文件, 里面的内容可以根据自己情况调整.

```ini
HOST = 0.0.0.0
PORT = 14001

# database
DATABASE_URL = "sqlite://sqlite_data/trace_data.sqlite?mode=rwc"

# log
LOG_ROLLING_TYPE = daily
LOG_PREFIX_PATH = ./logs
LOG_PREFIX_FILE_NAME = tracer.log
```