import os
import subprocess
import random
import string
import time
import re

# Configure test case generation parameters
test_cases = 100  # Number of test cases to generate

# File Configs
output_file = "../../../fuzz_outputs/C/weekly_contest_434_p2/outputs"  # Output file to store test cases and results
c_folder = "../src"  # Folder containing the C source code
c_file = "weekly_contest_434_p2.c"  # C source file name
executable_name = "solution"  # Executable name

# Generate a single test case
def generate_test_input():
    random.seed(time.time() + random.random())
    
    # 根据约束条件：1 <= numberOfUsers <= 100
    number_of_users = random.randint(1, 100)
    
    # 根据约束条件：1 <= events.length <= 100
    events_length = random.randint(1, 100)
    
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
                # 生成1到100个id<number>提及
                num_mentions = random.randint(1, min(100, number_of_users))
                mentions = []
                for _ in range(num_mentions):
                    user_num = random.randint(0, number_of_users - 1)
                    mentions.append(f"id{user_num}")
                content = " ".join(mentions)
            else:
                content = message_type
        
        # 时间戳：1 <= int(events[i][1]) <= 10^5
        timestamp = str(random.randint(1, 10**5))
        
        events.append([event_type, timestamp, content])
    
    return number_of_users, events

# Format test_input as a string for terminal input simulation
def format_test_input(test_input):
    try:
        number_of_users, events = test_input
        events_length = len(events)
        
        formatted_input = f"{number_of_users} {events_length}\n"
        
        for event in events:
            formatted_input += f"{event[0]} {event[1]} {event[2]}\n"
            
        return formatted_input
    except Exception as e:
        print(f"Error formatting test input: {e}")
        # 返回一个简单的有效输入作为备用
        return "2 3\nMESSAGE 10 id1 id0\nOFFLINE 11 0\nMESSAGE 71 HERE\n"

# Compile the C program
def compile_c():
    try:
        print("Compiling C program...")
        compile_command = ["gcc", os.path.join(c_folder, c_file), "-o", os.path.join(c_folder, executable_name)]
        process = subprocess.run(compile_command, check=True, capture_output=True, text=True)
        print("Compilation successful.")
        return True
    except subprocess.CalledProcessError as e:
        print(f"Error during compilation: {e}")
        print(f"Compilation errors: {e.stderr}")
        return False

# Simulate output for a test case by running the C program
def simulate_output(test_input):
    # Format input for the C program
    formatted_input = format_test_input(test_input)
    print(f"Running test with input:\n{formatted_input.strip()}")
    try:
        # Run the C program with the generated input
        run_command = [os.path.join(c_folder, executable_name)]
        process = subprocess.run(run_command, input=formatted_input, text=True, capture_output=True, timeout=5)
        if process.returncode != 0:
            print(f"Program exited with code {process.returncode}")
            if process.stderr:
                print(f"Error output: {process.stderr}")
            return f"Error (code {process.returncode})"
        
        # 提取输出中的数组部分
        output = process.stdout.strip()
        mentions_match = re.search(r"Mentions: (.*)", output)
        if mentions_match:
            return mentions_match.group(1)
        return output
    except subprocess.TimeoutExpired:
        print("Program execution timed out")
        return "Timeout"
    except Exception as e:
        print(f"Error during execution: {e}")
        return f"Error: {str(e)}"
    
# Clean up the compiled executable
def cleanup():
    executable_path = os.path.join(c_folder, executable_name)
    if os.path.exists(executable_path):
        os.remove(executable_path)
        print("Cleaned up compiled executable.")

# Main program
if __name__ == "__main__":
    try:
        # 创建输出目录
        os.makedirs(os.path.dirname(output_file), exist_ok=True)
        
        if not compile_c():
            print("Compilation failed. Exiting.")
            exit(1)
            
        with open(output_file, "w") as f:
            for case_id in range(1, test_cases + 1):
                print(f"\nTest case {case_id}/{test_cases}")
                try:
                    test_input = generate_test_input()
                    formatted_input = format_test_input(test_input)
                    simulated_output = simulate_output(test_input)
                    
                    # 写入实际的输入格式，而不是格式化后的
                    f.write(f"input:\n{formatted_input}")
                    f.write(f"output:\n{simulated_output}\n")
                    f.write("-------------------------\n")
                    f.flush()  # 确保结果立即写入文件
                except Exception as e:
                    print(f"Error processing test case {case_id}: {e}")
    finally:
        cleanup()
