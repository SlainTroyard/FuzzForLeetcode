import os
import subprocess
import random
import string
import time
import re
import signal

# Configure test case generation parameters
test_cases = 100  # Number of test cases to generate

# File Configs
output_file = "../../../fuzz_outputs/CPP/weekly_contest_434_p2/outputs"  # Output file to store test cases and results
cpp_folder = "../src"  # Folder containing the C++ source code
cpp_file = "weekly_contest_434_p2.cpp"  # C++ source file name
executable_name = "solution"  # Executable name

# 创建简单的测试样例，用于验证程序是否能正常运行
def create_simple_test():
    # 使用LeetCode提供的示例 - 确保使用数字ID而不是id前缀
    return 2, [
        ["MESSAGE", "10", "0 1"],  # 改为直接使用数字ID
        ["OFFLINE", "11", "0"],
        ["MESSAGE", "71", "HERE"]
    ]

# Generate a single test case
def generate_test_input():
    random.seed(time.time() + random.random())
    
    # 从较小的值开始，避免边界条件问题
    number_of_users = random.randint(1, 10)
    events_length = random.randint(1, 10)
    
    events = []
    online_users = set(range(number_of_users))  # 初始时所有用户都在线
    
    for i in range(events_length):
        # 事件类型
        if len(online_users) > 0 and random.random() < 0.3:  # 30%的概率生成OFFLINE事件
            event_type = "OFFLINE"
            # 随机选择一个在线用户
            user_id = str(random.choice(list(online_users)))
            online_users.remove(int(user_id))  # 用户下线
            content = user_id
        else:
            event_type = "MESSAGE"
            # 消息内容类型
            message_type = random.choice(["mention", "ALL", "HERE"])
            
            if message_type == "mention":
                # 限制提及数量，避免过长输入，并使用纯数字ID
                num_mentions = random.randint(1, min(5, number_of_users))
                mentions = []
                for _ in range(num_mentions):
                    user_num = random.randint(0, number_of_users - 1)
                    # 使用纯数字ID
                    mentions.append(str(user_num))
                content = " ".join(mentions)
            else:
                content = message_type
        
        # 时间戳：使用较小的时间戳范围
        timestamp = str(random.randint(1, 1000))
        
        events.append([event_type, timestamp, content])
    
    return number_of_users, events

# Format test_input as a string for terminal input simulation
def format_test_input(test_input):
    try:
        number_of_users, events = test_input
        events_length = len(events)
        
        formatted_input = f"{number_of_users} {events_length}\n"
        
        for event in events:
            formatted_input += f"{event[0]} {event[1]}"
            if event[0] == "MESSAGE" and event[2] not in ["ALL", "HERE"]:
                # 确保多个ID有空格分隔
                formatted_input += f" {event[2]}\n"
            else:
                formatted_input += f" {event[2]}\n"
            
        return formatted_input
    except Exception as e:
        print(f"Error formatting test input: {e}")
        # 返回一个简单的有效输入作为备用
        return "2 3\nMESSAGE 10 0 1\nOFFLINE 11 0\nMESSAGE 71 HERE\n"

# Compile the C++ program
def compile_cpp():
    try:
        print("Compiling C++ program...")
        # 添加更多警告和调试选项
        compile_command = ["g++", "-std=c++17", "-g", "-Wall", "-Wextra", os.path.join(cpp_folder, cpp_file), "-o", os.path.join(cpp_folder, executable_name)]
        process = subprocess.run(compile_command, check=True, capture_output=True, text=True)
        print("Compilation successful.")
        return True
    except subprocess.CalledProcessError as e:
        print(f"Error during compilation: {e}")
        print(f"Compilation errors: {e.stderr}")
        return False

# Simulate output for a test case by running the C++ program
def simulate_output(test_input):
    # Format input for the C++ program
    formatted_input = format_test_input(test_input)
    print(f"Running test with input:\n{formatted_input.strip()}")
    
    try:
        # 设置更长的超时时间
        run_command = [os.path.join(cpp_folder, executable_name)]
        
        # 使用subprocess.Popen以获取更详细的错误信息
        process = subprocess.Popen(
            run_command, 
            stdin=subprocess.PIPE, 
            stdout=subprocess.PIPE, 
            stderr=subprocess.PIPE,
            text=True
        )
        
        # 设置10秒超时
        try:
            stdout, stderr = process.communicate(input=formatted_input, timeout=10)
            
            if process.returncode != 0:
                print(f"Program exited with code {process.returncode}")
                if stderr:
                    print(f"Error output: {stderr}")
                return f"Error (code {process.returncode}): {stderr}"
            
            return stdout.strip()
        except subprocess.TimeoutExpired:
            process.kill()
            print("Program execution timed out")
            return "Timeout"
            
    except Exception as e:
        print(f"Error during execution: {str(e)}")
        return f"Error: {str(e)}"
    
# Clean up the compiled executable
def cleanup():
    executable_path = os.path.join(cpp_folder, executable_name)
    if os.path.exists(executable_path):
        os.remove(executable_path)
        print("Cleaned up compiled executable.")

# 测试程序是否可以正常运行基本用例
def test_with_simple_input():
    simple_test = create_simple_test()
    formatted_input = format_test_input(simple_test)
    print("\n=== Testing with simple input ===")
    print(formatted_input)
    print("=== End of test input ===\n")
    
    try:
        run_command = [os.path.join(cpp_folder, executable_name)]
        process = subprocess.run(run_command, input=formatted_input, text=True, capture_output=True, timeout=5)
        
        print(f"Return code: {process.returncode}")
        print(f"Output: {process.stdout}")
        if process.stderr:
            print(f"Error: {process.stderr}")
            
        return process.returncode == 0
    except Exception as e:
        print(f"Error during test: {e}")
        return False

# Main program
if __name__ == "__main__":
    try:
        # 创建输出目录
        os.makedirs(os.path.dirname(output_file), exist_ok=True)
        
        if not compile_cpp():
            print("Compilation failed. Exiting.")
            exit(1)
        
        # 先用简单用例测试程序
        if not test_with_simple_input():
            print("Program failed with simple input. Please check the implementation.")
            print("Continuing with fuzzing using simpler test cases...")
        
        with open(output_file, "w") as f:
            for case_id in range(1, test_cases + 1):
                print(f"\nTest case {case_id}/{test_cases}")
                try:
                    # 从简单情况开始，然后逐渐增加复杂度
                    if case_id <= 10:
                        test_input = create_simple_test()  # 前10个用例用简单测试
                    else:
                        test_input = generate_test_input()
                        
                    formatted_input = format_test_input(test_input)
                    simulated_output = simulate_output(test_input)
                    
                    # 写入实际的输入格式，而不是格式化后的
                    f.write(f"input:\n{formatted_input}")
                    f.write(f"output:\n{simulated_output}\n")
                    f.write("-------------------------\n")
                    f.flush()  # 确保结果立即写入文件
                    
                    # 如果出错，尝试休息一下再继续
                    if "Error" in simulated_output:
                        print("Detected error, pausing for 1 second...")
                        time.sleep(1)
                        
                except Exception as e:
                    print(f"Error processing test case {case_id}: {e}")
    finally:
        cleanup()
