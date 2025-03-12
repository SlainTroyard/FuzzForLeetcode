import os
import subprocess
import random
import string
import time

# TODO: Configure test case generation parameters
test_cases = 100  # Number of test cases to generate
max_string_length = 100  # Maximum length of the string
min_string_length = 3  # Minimum length of the string

# File Configs
output_file = "../../../fuzz_outputs/CPP/weekly_contest_435_p1/outputs"  # Output file to store test cases and results
cpp_folder = "../src"  # Folder containing the C++ source code
cpp_file = "weekly_contest_435_p1.cpp"  # C++ source file name
executable_name = "solution"  # Executable name

# TODO: Generate a single test case
def generate_test_input():
    random.seed(time.time())
    # 3 <= s.length <= 100, s consists only of lowercase English letters. s contains at least one character with an odd frequency and one with an even frequency.
    s_length = random.randint(min_string_length, max_string_length)
    all_chars = list(string.ascii_lowercase)
    char1, char2 = random.sample(all_chars, 2)
    
    # 初始化字符出现次数：char1奇数次，char2偶数次
    count_char1 = 1  # 奇数
    count_char2 = 2  # 偶数
    remaining_length = s_length - count_char1 - count_char2
    
    # 生成剩余字符，排除char1和char2以确保它们的次数奇偶性不变
    other_chars = [c for c in all_chars if c not in {char1, char2}]
    remaining_parts = []
    if remaining_length > 0:
        remaining_parts = random.choices(other_chars, k=remaining_length)
    
    # 构建字符串并打乱顺序
    s = [char1] * count_char1 + [char2] * count_char2 + remaining_parts
    random.shuffle(s)
    s = ''.join(s)
    
    # 验证条件
    freq = {}
    for c in s:
        freq[c] = freq.get(c, 0) + 1
    has_odd = any(v % 2 != 0 for v in freq.values())
    has_even = any(v % 2 == 0 for v in freq.values())
    assert has_odd and has_even, "生成的字符串不符合条件"
    
    return s

# TODO: Format test_input as a string for terminal input simulation
def format_test_input(test_input):
    s = test_input
    formatted_input = f"{s}\n"
    return formatted_input

# Compile the C++ program
def compile_cpp():
    try:
        compile_command = ["g++", os.path.join(cpp_folder, cpp_file), "-o", os.path.join(cpp_folder, executable_name)]
        subprocess.run(compile_command, check=True)
        print("Compilation successful.")
    except subprocess.CalledProcessError as e:
        print(f"Error during compilation: {e}")
        raise

# Simulate output for a test case by running the C++ program
def simulate_output(test_input):
    # Format input for the C++ program
    formatted_input = format_test_input(test_input)
    try:
        # Run the C++ program with the generated input
        run_command = [os.path.join(cpp_folder, executable_name)]
        process = subprocess.run(run_command, input=formatted_input, text=True, capture_output=True, check=True)
        return process.stdout.strip()
    except subprocess.CalledProcessError as e:
        print(f"Error during execution: {e}")
        return "Error"

# Clean up the compiled executable
def cleanup():
    executable_path = os.path.join(cpp_folder, executable_name)
    if os.path.exists(executable_path):
        os.remove(executable_path)
        print("Cleaned up compiled executable.")

# Main program
if __name__ == "__main__":
    try:
        compile_cpp()
        with open(output_file, "w") as f:
            for case_id in range(1, test_cases + 1):
                test_input = generate_test_input()
                formatted_input = format_test_input(test_input)
                simulated_output = simulate_output(test_input)
                # TODO: if the formatted_input ends with a newline, remove the newline
                if formatted_input.endswith("\n"):
                    formatted_input = formatted_input[:-1]
                f.write(f"input:\n{formatted_input}\n")
                f.write(f"output:\n{simulated_output}\n")
                f.write("-------------------------\n")
    finally:
        cleanup()
