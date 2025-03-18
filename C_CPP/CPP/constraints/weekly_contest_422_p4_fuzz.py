import os
import subprocess
import random
import string
import time

test_cases = 100  # Number of test cases to generate
min_nonzero_cases = 50  # Minimum number of test cases that should produce non-zero results


# File Configs
output_file = "../../../fuzz_outputs/CPP/weekly_contest_422_p4/outputs"  # Output file to store test cases and results
cpp_folder = "../src"  # Folder containing the C++ source code
cpp_file = "weekly_contest_422_p4.cpp"  # C++ source file name
executable_name = "solution"  # Executable name

def generate_test_input(force_nonzero=False):
    random.seed(time.time() + random.random())

    length = random.randint(2, 80)
    
    # 生成随机数字
    digits = [str(random.randint(0, 9)) for _ in range(length)]
    
    # 考虑题目要求，最终结果如果全是随机，很容易出现0
    if force_nonzero:
        # 增加非零结果的概率
        # 策略1: 确保第一个数字不为0
        digits[0] = str(random.randint(1, 9))
        
        # 策略2: 确保至少有一些非零数字
        non_zero_count = max(length // 3, 1)
        positions = random.sample(range(length), non_zero_count)
        for pos in positions:
            digits[pos] = str(random.randint(1, 9))
            
        # 策略3: 避免所有数字都是偶数或都是5的倍数等导致结果容易为0的情况
        has_odd = False
        for i in range(length):
            if int(digits[i]) % 2 == 1 and int(digits[i]) != 5:
                has_odd = True
                break
        
        if not has_odd:
            digits[random.randint(0, length-1)] = str(random.choice([1, 3, 7, 9]))
    
    # 连接数字成字符串
    return "".join(digits)

def format_test_input(test_input):
    try:
        # 直接返回字符串，不带引号
        return f"{test_input}\n"
    except Exception as e:
        print(f"Error formatting test input: {e}")
        # 返回一个简单的有效字符串作为备用
        return "123\n"

# 编译C++程序
def compile_cpp():
    try:
        print("Compiling C++ program...")
        compile_command = ["g++", "-std=c++17", os.path.join(cpp_folder, cpp_file), "-o", os.path.join(cpp_folder, executable_name)]
        process = subprocess.run(compile_command, check=True, capture_output=True, text=True)
        print("Compilation successful.")
        return True
    except subprocess.CalledProcessError as e:
        print(f"Error during compilation: {e}")
        print(f"Compilation errors: {e.stderr}")
        return False

# 通过运行C++程序来模拟输出
def simulate_output(test_input):
    # 格式化输入
    formatted_input = format_test_input(test_input)
    print(f"Running test with input: {formatted_input.strip()}")
    try:
        # 运行C++程序
        run_command = [os.path.join(cpp_folder, executable_name)]
        process = subprocess.run(run_command, input=formatted_input, text=True, capture_output=True, timeout=5)
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

# 清理编译的可执行文件
def cleanup():
    executable_path = os.path.join(cpp_folder, executable_name)
    if os.path.exists(executable_path):
        os.remove(executable_path)
        print("Cleaned up compiled executable.")

# 检查输出是否为零
def is_output_zero(output):
    try:
        # 尝试解析输出，看看结果是不是0
        # 移除所有空白字符，并尝试转换为整数
        cleaned_output = output.strip().replace(" ", "")
        if cleaned_output.isdigit() or (cleaned_output.startswith("-") and cleaned_output[1:].isdigit()):
            return int(cleaned_output) == 0
        
        # 如果输出包含多行，尝试检查最后一行的结果
        lines = output.strip().split("\n")
        last_line = lines[-1].strip()
        if last_line.isdigit() or (last_line.startswith("-") and last_line[1:].isdigit()):
            return int(last_line) == 0
            
        # 如果看起来像是数组或列表，检查是否所有元素都是0
        if "[" in output and "]" in output:
            array_part = output[output.find("[")+1:output.rfind("]")]
            elements = [e.strip() for e in array_part.split(",")]
            return all(e == "0" for e in elements)
            
        # 如果无法确定，假设是非零
        return False
    except:
        # 如果解析失败，默认认为结果非零
        return False

# 主程序
if __name__ == "__main__":
    try:
        # 创建输出目录
        os.makedirs(os.path.dirname(output_file), exist_ok=True)
        
        if not compile_cpp():
            print("Compilation failed. Exiting.")
            exit(1)
            
        nonzero_count = 0
        remaining_cases = test_cases
        
        with open(output_file, "w") as f:
            for case_id in range(1, test_cases + 1):
                print(f"\nTest case {case_id}/{test_cases}")
                try:
                    # 如果我们需要更多非零结果且已测试用例数量较多，则强制生成非零输入
                    force_nonzero = nonzero_count < min_nonzero_cases and (test_cases - case_id) < (min_nonzero_cases - nonzero_count)
                    
                    test_input = generate_test_input(force_nonzero)
                    formatted_input = format_test_input(test_input)
                    simulated_output = simulate_output(test_input)
                    
                    # 检查输出是否为零
                    if not is_output_zero(simulated_output):
                        nonzero_count += 1
                        print(f"Non-zero result detected. Total non-zero results: {nonzero_count}/{case_id}")
                    else:
                        print(f"Zero result detected. Total non-zero results: {nonzero_count}/{case_id}")
                    
                    # 如果格式化输入以换行符结尾，删除它
                    if formatted_input.endswith("\n"):
                        formatted_input = formatted_input[:-1]
                        
                    f.write(f"input:\n{formatted_input}\n")
                    f.write(f"output:\n{simulated_output}\n")
                    f.write("-------------------------\n")
                    f.flush()  # 确保结果立即写入文件
                except Exception as e:
                    print(f"Error processing test case {case_id}: {e}")
        
        print(f"\nFuzzing completed. Generated {nonzero_count} test cases with non-zero results out of {test_cases} total cases.")
    finally:
        cleanup()
