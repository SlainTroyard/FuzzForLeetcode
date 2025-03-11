import os
import subprocess
import random
import time  

# TODO: Configure test case generation parameters
test_cases = 100  # Number of test cases to generate
min_nums_length = 3
max_nums_length = 10**5
min_value = -1000
max_value = 1000

# File Configs
output_file = "../../../fuzz_outputs/CPP/weekly_contest_426_p2/outputs"  # Output file to store test cases and results
cpp_folder = "../src"  # Folder containing the C++ source code
cpp_file = "weekly_contest_426_p2.cpp"  # C++ source file name
executable_name = "solution"  # Executable name

def generate_test_input():
    random.seed(time.time())
    # Step 1: Randomly generate the array length n
    nums_length = random.randint(4, 1000)  # Avoid extremely large arrays, example size
    num_special = nums_length  # n - 2 special numbers

    # Step 2: Calculate the range for generating special numbers
    max_value = 1000 // num_special  # Ensure the sum does not exceed 1000
    min_value = -1000 // num_special  # Ensure the sum is not less than -1000

    # Step 3: Randomly generate special numbers
    special_numbers = [random.randint(min_value, max_value) for _ in range(num_special)]

    # Step 4: Calculate the sum of special numbers
    sum_element = sum(special_numbers)
    while abs(sum_element) > 1000:  # Ensure the sum is within the valid range
        special_numbers = [random.randint(min_value, max_value) for _ in range(num_special)]
        sum_element = sum(special_numbers)

    # Step 5: Generate the outlier
    outlier = random.randint(-1000, 1000)
    while outlier in special_numbers or outlier == sum_element:
        outlier = random.randint(-1000, 1000)

    # Step 6: Combine the array
    nums = special_numbers + [sum_element, outlier]

    # Step 7: Shuffle the array
    random.shuffle(nums)

    return nums

# Format test_input as a string for terminal input simulation
def format_test_input(test_input):
    n = len(test_input)
    formatted_input = f"{n}\n" + " ".join(map(str, test_input)) + "\n"
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
                f.write(f"input:\n{formatted_input}")
                f.write(f"output:\n{simulated_output}\n")
                f.write("-------------------------\n")
    finally:
        cleanup()
