import os
import subprocess
import random
import string

# TODO: Configure test case generation parameters
test_cases = 20  # Number of test cases to generate


# File Configs
output_file = "../../../fuzz_outputs/CPP/weekly_contest_424_p2/outputs"  # Output file to store test cases and results
cpp_folder = "../src"  # Folder containing the C++ source code
cpp_file = "weekly_contest_424_p2.cpp"  # C++ source file name
executable_name = "solution"  # Executable name

# Constants
MAX_VAL = 10**5

# Generate random test case input
def generate_queries(nums_size, queries_size):
    queries = []
    for _ in range(queries_size):
        l = random.randint(0, nums_size - 1)
        r = random.randint(l, nums_size - 1)
        queries.append([l, r])
    return queries

def calculate_coverage(nums_size, queries):
    # Initialize coverage count for each index in nums
    coverage_count = [0] * nums_size
    for query in queries:
        li, ri = query
        for i in range(li, ri + 1):
            coverage_count[i] += 1
    return coverage_count

def generate_nums(nums_size, coverage_count, valid=True):
    # Generate nums array based on coverage_count
    nums = []
    for i in range(nums_size):
        max_cover = coverage_count[i]
        if valid:
            # For valid nums, the value should be <= the number of times it is covered
            nums.append(random.randint(0, max_cover))
        else:
            # For invalid nums, the value should be > the number of times it is covered
            nums.append(random.randint(0, MAX_VAL))
    return nums

def generate_test_input(nums_size=random.randint(1,MAX_VAL), queries_size=random.randint(1,MAX_VAL)):
    # Step 1: Generate random queries
    queries = generate_queries(nums_size, queries_size)

    # Step 2: Calculate coverage count for each index in nums
    coverage_count = calculate_coverage(nums_size, queries)
    
    # Step 3: Randomly decide whether to generate a valid or invalid nums array
    is_valid = random.choice([True, False])  # 50% chance to generate valid or invalid nums
    
    # Step 4: Generate the nums array based on the coverage count and validity
    nums = generate_nums(nums_size, coverage_count, valid=is_valid)

    print("finished generating test input")
    
    return nums, queries

# Format the test input as a string for terminal input simulation
def format_test_input(test_input):
    nums, queries = test_input

    # Format the array and queries into a string
    nums_str = ' '.join(map(str, nums))
    queries_str = '\n'.join([f"{query[0]} {query[1]}" for query in queries])
    
    # Return the formatted input for the C program
    return f"{len(nums)}\n{nums_str}\n{len(queries)}\n{queries_str}"

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
