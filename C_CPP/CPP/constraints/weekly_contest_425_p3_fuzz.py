import os
import subprocess
import random
import string

# Configure test case generation parameters
test_cases = 100  # Number of test cases to generate

# File Configs
output_file = "../../../fuzz_outputs/CPP/weekly_contest_425_p3/outputs"  # Output file to store test cases and results
cpp_folder = "../src"  # Folder containing the C++ source code
cpp_file = "weekly_contest_425_p3.cpp"  # C++ source file name
executable_name = "solution"  # Executable name

# Generate a single test case
def generate_test_input():
    n = random.randint(1, 100)  # Length of the nums array
    nums = [random.randint(0, 100000) for _ in range(n)]  # Generate nums array
    k = random.randint(0, 100000)  # Random k
    op1 = random.randint(0, n)  # Random op1 (between 0 and n)
    op2 = random.randint(0, n)  # Random op2 (between 0 and n)
    
    # Return the generated test case as a tuple
    return (n, k, op1, op2, nums)

# Format test_input as a string for terminal input simulation
def format_test_input(test_input):
    n, k, op1, op2, nums = test_input
    # Format the input for the C++ program
    formatted_input = f"{n} {k} {op1} {op2}\n"
    formatted_input += " ".join(map(str, nums)) + "\n"
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
                # Write the input and output to the file
                f.write(f"input:\n{formatted_input}")
                f.write(f"output:\n{simulated_output}\n")
                f.write("-------------------------\n")
    finally:
        cleanup()
