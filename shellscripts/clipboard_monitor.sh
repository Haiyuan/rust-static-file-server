#!/bin/bash

# 初始化上次剪贴板内容为空
last_clipboard=""

# 无限循环监控剪贴板内容
while true; do
  # 当前剪贴板内容
  current_clipboard=$(pbpaste)

  # 比较剪贴板内容
  if [ "$current_clipboard" != "$last_clipboard" ]; then
    # 更新上次剪贴板内容
    last_clipboard="$current_clipboard"

    # 检查内容是否为特定URL
    if [[ "$current_clipboard" =~ ^http://127\.0\.0\.1:8000 ]]; then
      # 触发Alfred工作流
      osascript -e 'tell application "Alfred 5" to run trigger "clipboard_trigger" in workflow "com.haiyuan.easydict.helper.workflow.alfred" with argument "'"$current_clipboard"'"'
    fi
  fi

  # 每秒检查一次
  sleep 1
done