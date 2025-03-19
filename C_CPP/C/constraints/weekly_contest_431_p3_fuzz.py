import os
import subprocess
import random
import string
import time

# TODO: Configure test case generation parameters
test_cases = 10  # Number of test cases to generate
max_coins = 10**5  # Maximum number of coins
max_k = 10**9  # Maximum value of k
max_li = 10**9  # Maximum value of li
max_ri = 10**9  # Maximum value of ri
max_ci = 1000  # Maximum value of ci


# File Configs
output_file = "../../../fuzz_outputs/C/weekly_contest_431_p3/outputs"  # Output file to store test cases and results
c_folder = "../src"  # Folder containing the C source code
c_file = "weekly_contest_431_p3.c"  # C source file name
executable_name = "solution"  # Executable name

# TODO: Generate a single test case
def generate_test_input():
    random.seed(time.time())
    # 1 <= coins.length <= 10^5, 1 <= k <= 10^9, coins[i] == [li, ri, ci], 1 <= li <= ri <= 10^9, 1 <= ci <= 1000. The given segments are non-overlapping.
    coins = []
    n = random.randint(1, max_coins)
    k = random.randint(1, max_k)
    # 在1-10^9中随机取出n个不重复区间
    ranges = random.sample(range(1, 10**9), n*2)
    ranges.sort()
    for i in range(n):
        li = ranges[i*2]
        ri = ranges[i*2+1]
        ci = random.randint(1, max_ci)
        coins.append([li, ri, ci])
    # 打乱顺序
    random.shuffle(coins)
    return n, k, coins

# TODO: Format test_input as a string for terminal input simulation
def format_test_input(test_input):
    n, k, coins = test_input
    formatted_input = f"{n} {k}\n"
    for coin in coins:
        formatted_input += f"{coin[0]} {coin[1]} {coin[2]}\n"
    return formatted_input

# Compile the C program
def compile_c():
    try:
        compile_command = ["gcc", os.path.join(c_folder, c_file), "-o", os.path.join(c_folder, executable_name)]
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
        process = subprocess.run(run_command, input=formatted_input, text=True, capture_output=True, check=True)
        return process.stdout.strip()
    except subprocess.CalledProcessError as e:
        print(f"Error during execution: {e}")
        return "Error"

# Clean up the compiled executable
def cleanup():
    executable_path = os.path.join(c_folder, executable_name)
    if os.path.exists(executable_path):
        os.remove(executable_path)
        print("Cleaned up compiled executable.")

# Main program
if __name__ == "__main__":
    try:
        compile_c()
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
