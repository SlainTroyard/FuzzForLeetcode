import json
import os
import re
import argparse
import requests
from pathlib import Path

def load_config(config_path):
    """加载配置文件"""
    if not os.path.exists(config_path):
        print(f"Config file {config_path} not found.")
        return None
    
    with open(config_path) as f:
        config = json.load(f)
    
    return config

def create_prompt(source_code, language, contest, problem):
    """创建翻译提示"""
    return f"""You are a programming expert specialized in translating {language} code to Rust.

Translate this {language} code from LeetCode Weekly Contest {contest} Problem {problem} to idiomatic Rust:

```{language}
{source_code}
```

REQUIREMENTS:
1. Translate the ENTIRE file as a complete program, including the main function and I/O handling
2. Preserve the algorithm logic exactly
3. Use idiomatic Rust with proper error handling
4. Maintain the EXACT SAME stdin/stdout format as the original code
5. Add helpful comments where needed

IMPORTANT FOR I/O HANDLING:
- CAREFULLY analyze the original code to understand the exact input/output format
- The input may consist of multiple lines or multiple values per line
- Preserve the exact same input parsing logic as the original code
- Ensure your output matches the exact format of the original code
- Do not make assumptions about the input format - follow the original code

Your response MUST contain ONLY the Rust code wrapped in a ```rust code block.
"""

def call_llm_api_streaming(prompt, config):
    """使用流式输出调用API"""
    headers = {
        "Content-Type": "application/json",
        "Authorization": f"Bearer {config['api_key']}"
    }

    messages = []
    if config["system_message"]:
        messages.append({"role": "system", "content": config["system_message"]})
    messages.append({"role": "user", "content": prompt})

    payload = {
        "model": config["default_model"],
        "messages": messages,
        "temperature": config.get("temperature", 0.7),
        "max_tokens": config.get("max_tokens", 4000),
        "top_p": config.get("top_p", 1.0),
        "stream": True  # 启用流式输出
    }

    full_response = ""
    
    try:
        # 使用stream=True参数来获取流式响应
        with requests.post(config["api_url"], json=payload, headers=headers, stream=True) as response:
            response.raise_for_status()
            
            print("\nStreaming response:\n")
            for line in response.iter_lines():
                if line:
                    # 移除 "data: " 前缀并解析JSON
                    line_text = line.decode('utf-8')
                    if line_text.startswith("data: ") and line_text != "data: [DONE]":
                        json_str = line_text[6:]  # 去掉 "data: " 前缀
                        try:
                            chunk = json.loads(json_str)
                            # 更安全的数据提取方式
                            choices = chunk.get("choices", [])
                            if choices and len(choices) > 0:
                                delta = choices[0].get("delta", {})
                                content = delta.get("content", "")
                                if content:
                                    print(content, end="", flush=True)  # 实时打印内容
                                    full_response += content
                        except json.JSONDecodeError as e:
                            print(f"\nJSON解析错误: {e}, 行: {line_text}")
                            continue
            
            print("\n\nStreaming complete.\n")
            return full_response
            
    except requests.exceptions.RequestException as e:
        print(f"API请求错误: {e}")
        return None

def extract_rust_code(response):
    """从响应中提取Rust代码块"""
    match = re.search(r"```rust\n(.*?)\n```", response, re.DOTALL)
    return match.group(1).strip() if match else response

def translate_code(source_path, config, contest, problem):
    """执行翻译流程"""
    # 读取源代码
    with open(source_path) as f:
        source_code = f.read()
    
    # 确定源代码语言
    language = "C++" if source_path.endswith(".cpp") else "C"
    
    # 创建提示
    prompt = create_prompt(source_code, language, contest, problem)
    
    # 调用API（流式输出）
    print("Calling LLM API with streaming enabled...")
    response = call_llm_api_streaming(prompt, config)
    
    if response:
        rust_code = extract_rust_code(response)
    else:
        # 生成占位代码
        rust_code = f"// Error: Failed to get translation\n{response}"
    
    # 保存结果到llm_outputs/{model_name}/Path(source_path).stem.rs
    os.makedirs("llm_outputs", exist_ok=True)
    model_name = config["default_model"].replace("/", "_")
    output_path = "llm_outputs/" + model_name + "/" + Path(source_path).stem + ".rs"
    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    with open(output_path, "w") as f:
        f.write(rust_code)
    
    print(f"Translation saved to {output_path}")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="C/C++ to Rust Translator")
    parser.add_argument("--source", help="Path to source code file (.c/.cpp)", default="C_CPP/C/src/weekly_contest_413_p2.c", required=False)
    parser.add_argument("-c", "--config", default="config_example.json", help="Config file path", required=False)
    
    args = parser.parse_args()

    # source code file path 可能是：C_CPP/C/src/weekly_contest_413_p2.c
    contest, problem = re.search(r"weekly_contest_(\d+)_p(\d+)", args.source).groups()
    
    # 加载配置，默认路径为 config_example.json
    config = load_config(args.config)
    
    # 验证配置
    if not config["api_key"]:
        raise ValueError("API key missing in configuration")
    
    # 执行翻译
    translate_code(args.source, config, contest, problem)