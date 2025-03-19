import os
import subprocess
import random
import string
import time

# Configure test case generation parameters
test_cases = 100  # Number of test cases to generate
max_length = 256  # Maximum length of the input string
small_case_ratio = 0.7  # 70% of test cases will be small
small_case_max_length = 50  # Maximum length for small test cases
medium_case_max_length = 150  # Maximum length for medium test cases

# File Configs
output_file = "../../../fuzz_outputs/C/weekly_contest_434_p4/outputs"  # Output file to store test cases and results
c_folder = "../src"  # Folder containing the C source code
c_file = "weekly_contest_434_p4.c"  # C source file name
executable_name = "solution"  # Executable name

# Generate a single test case with controlled size
def generate_test_input(case_id):
    random.seed(time.time() + case_id)  # 使用case_id增加随机性
    
    # 根据case_id确定测试用例大小
    if case_id <= test_cases * small_case_ratio:  # 70%的小规模测试用例
        n = random.randint(1, small_case_max_length)
        max_unique_letters = min(8, random.randint(4, 10))  # 限制小规模测试用例的字母数量
    elif case_id <= test_cases * 0.9:  # 20%的中等规模测试用例
        n = random.randint(small_case_max_length + 1, medium_case_max_length)
        max_unique_letters = min(12, random.randint(6, 12))
    else:  # 10%的大规模测试用例
        n = random.randint(medium_case_max_length + 1, max_length)
        max_unique_letters = min(16, random.randint(8, 16))
    
    # 确保n不超过最大长度
    n = min(n, max_length)
    
    # 生成唯一字母集合（最多16个，但根据测试用例大小可能更少）
    lowercase_letters = string.ascii_lowercase
    unique_letters = random.sample(lowercase_letters, k=max_unique_letters)
    
    # 生成单词
    words = []
    attempts = 0
    max_attempts = n * 5  # 避免死循环
    
    while len(words) < n and attempts < max_attempts:
        attempts += 1
        word = ''.join(random.choices(unique_letters, k=2))
        if word not in words:  # 确保单词唯一
            words.append(word)
    
    # 如果无法生成足够多的唯一单词，调整n
    n = len(words)
    
    print(f"Generated test case {case_id}: n={n}, unique_letters={max_unique_letters}")
    return n, words

# Format test_input as a string for terminal input simulation
def format_test_input(test_input):
    n, words = test_input
    formatted_input = f"{n}\n"
    for word in words:
        formatted_input += word + "\n"
    return formatted_input

# Compile the C program
def compile_c():
    try:
        compile_command = ["gcc", "-O2", os.path.join(c_folder, c_file), "-o", os.path.join(c_folder, executable_name)]
        subprocess.run(compile_command, check=True)
        print("Compilation successful.")
    except subprocess.CalledProcessError as e:
        print(f"Error during compilation: {e}")
        raise

# Simulate output for a test case by running the C program
def simulate_output(test_input):
    # Format input for the C program
    formatted_input = format_test_input(test_input)
    try:
        # Run the C program with the generated input
        run_command = [os.path.join(c_folder, executable_name)]
        process = subprocess.run(run_command, input=formatted_input, text=True, capture_output=True, check=True, timeout=30)  # 添加30秒超时限制
        return process.stdout.strip()
    except subprocess.TimeoutExpired:
        return "Timeout Error: Execution took too long"
    except subprocess.CalledProcessError as e:
        print(f"Error during execution: {e}")
        return f"Error: {e.stderr}"

# Clean up the compiled executable
def cleanup():
    executable_path = os.path.join(c_folder, executable_name)
    if os.path.exists(executable_path):
        os.remove(executable_path)
        print("Cleaned up compiled executable.")

# Main program
if __name__ == "__main__":
    try:
        # 确保输出目录存在
        os.makedirs(os.path.dirname(output_file), exist_ok=True)
        
        compile_c()
        with open(output_file, "w") as f:
            for case_id in range(1, test_cases + 1):
                print(f"\nGenerating test case {case_id}/{test_cases}...")
                test_input = generate_test_input(case_id)
                formatted_input = format_test_input(test_input)
                
                start_time = time.time()
                simulated_output = simulate_output(test_input)
                execution_time = time.time() - start_time
                
                # 如果formatted_input以换行符结尾，则删除
                if formatted_input.endswith("\n"):
                    formatted_input = formatted_input[:-1]
                
                f.write(f"input:\n{formatted_input}\n")
                f.write(f"output:\n{simulated_output}\n")
                f.write("-------------------------\n")
                
                print(f"Test case {case_id} completed in {execution_time:.4f} seconds")
    except Exception as e:
        print(f"Error in main execution: {e}")
    finally:
        cleanup()
