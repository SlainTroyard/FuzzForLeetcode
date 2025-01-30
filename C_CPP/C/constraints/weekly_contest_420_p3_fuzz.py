import os
import subprocess
import random
import string
import time

# TODO: Configure test case generation parameters
test_cases = 20  # Number of test cases to generate
max_length = 10**5  # Maximum length of the input string
min_length = 1  # Minimum length of the input string
max_value = 10**6  # Maximum value of any integer in the input
min_value = 1  # Minimum value of any integer in the input

# File Configs
output_file = "../../../fuzz_outputs/C/weekly_contest_420_p3/outputs"  # Output file to store test cases and results
c_folder = "../src"  # Folder containing the C source code
c_file = "weekly_contest_420_p3.c"  # C source file name
executable_name = "solution"  # Executable name

def is_prime(n):
    if n <= 1:
        return False
    if n <= 3:
        return True
    if n % 2 == 0 or n % 3 == 0:
        return False
    i = 5
    w = 2
    while i * i <= n:
        if n % i == 0:
            return False
        i += w
        w = 6 - w
    return True

def generate_s(n):
    s = []
    if n == 0:
        return s
    # Generate the first element
    if random.random() < 0.5:
        s_prev = 1
    else:
        small_primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]
        s_prev = random.choice(small_primes)
    s.append(s_prev)
    for i in range(1, n):
        if random.random() < 0.5:
            # Keep the previous value
            s.append(s_prev)
        else:
            # Generate next prime greater than s_prev
            current = s_prev
            if current == 1:
                current = 2
            else:
                current += 1
            while True:
                if is_prime(current):
                    break
                current += 1
            s_prev = current
            s.append(s_prev)
    return s

def generate_x(s_i, max_val):
    if s_i == 1:
        return 1
    # Calculate maximum exponent k where s_i^k <= max_val
    max_k = 0
    current = 1
    while True:
        next_val = current * s_i
        if next_val > max_val:
            break
        current = next_val
        max_k += 1
    if max_k == 0:
        return s_i
    k = random.randint(1, max_k)
    return s_i ** k

def generate_test_input():
    random.seed(time.time())
    n = random.randint(min_length, max_length)
    s_list = generate_s(n)
    arr = []
    for si in s_list:
        x = generate_x(si, max_value)
        arr.append(x)
    return n, arr

def format_test_input(test_input):
    n, arr = test_input
    formatted_input = f"{n}\n"
    formatted_input += " ".join(map(str, arr))
    return formatted_input

# Compile the C program
def compile_c():
    try:
        compile_command = ["gcc", os.path.join(c_folder, c_file), "-o", os.path.join(c_folder, executable_name), "-lm"] # sometimes need to add -lm for math library
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
