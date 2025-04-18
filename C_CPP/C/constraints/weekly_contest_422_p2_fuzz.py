import os
import subprocess
import random
import string
import time

# Configure test case generation parameters
test_cases = 100  # Number of test cases to generate

# File Configs
output_file = "../../../fuzz_outputs/C/weekly_contest_422_p2/outputs"  # Output file to store test cases and results
c_folder = "../src"  # Folder containing the C source code
c_file = "weekly_contest_422_p2.c"  # C source file name
executable_name = "solution"  # Executable name

# Generate a single test case
def generate_test_input():
    random.seed(time.time() + random.random())
    # 生成较大的矩阵以便测试
    n = random.randint(2, 50)
    m = random.randint(2, 50)
    matrix = []
    for _ in range(n):
        row = [random.randint(0, 10**9) for _ in range(m)]
        matrix.append(row)
    
    return (n, m, matrix)

# Format test_input as a string for terminal input simulation
def format_test_input(test_input):
    try:
        n, m, matrix = test_input
        # 使用简单的行列格式
        formatted_input = f"{n} {m}\n"
        
        for row in matrix:
            formatted_input += " ".join(map(str, row)) + "\n"
            
        return formatted_input
    except Exception as e:
        print(f"Error formatting test input: {e}")
        # 返回一个简单的有效矩阵作为备用
        return "2 2\n0 1\n1 0\n"

# Compile the C program
def compile_c():
    try:
        print("Compiling C program...")
        compile_command = ["gcc", os.path.join(c_folder, c_file), "-o", os.path.join(c_folder, executable_name), "-lm"] # 添加-lm链接数学库
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
    print(f"Running test with input: {formatted_input.strip()}")
    try:
        # Run the C program with the generated input
        run_command = [os.path.join(c_folder, executable_name)]
        process = subprocess.run(run_command, input=formatted_input, text=True, capture_output=True, timeout=10)
        if process.returncode != 0:
            print(f"Program exited with code {process.returncode}")
            if process.stderr:
                print(f"Error output: {process.stderr}")
            return f"Error (code {process.returncode})"
        return process.stdout.strip()
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
                    
                    # 如果格式化输入以换行符结尾，删除它
                    if formatted_input.endswith("\n"):
                        formatted_input = formatted_input[:-1]
                        
                    f.write(f"input:\n{formatted_input}\n")
                    f.write(f"output:\n{simulated_output}\n")
                    f.write("-------------------------\n")
                    f.flush()  # 确保结果立即写入文件
                except Exception as e:
                    print(f"Error processing test case {case_id}: {e}")
    finally:
        cleanup()
