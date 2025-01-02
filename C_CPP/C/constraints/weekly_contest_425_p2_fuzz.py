import os
import subprocess
import random
import string

# TODO: Configure test case generation parameters
test_cases = 100  # Number of test cases to generate

# File Configs
output_file = "../../../fuzz_outputs/C/weekly_contest_425_p2/outputs"  # Output file to store test cases and results
c_folder = "../src"  # Folder containing the C source code
c_file = "weekly_contest_425_p2.c"  # C source file name
executable_name = "solution"  # Executable name

# Helper function to generate random lowercase letters string
def generate_random_string(length):
    return ''.join(random.choices(string.ascii_lowercase, k=length))

# Generate test input based on constraints
def generate_test_input():
    n = random.randint(1, 10) * 10  # Generate lengths that are multiples of 10
    k = random.randint(1, n)
    
    # Generate an anagram pair of strings `s` and `t`
    s = generate_random_string(n)
    t = list(s)  # Create a copy of `s`
    random.shuffle(t)  # Shuffle to create an anagram
    t = ''.join(t)  # Convert back to string
    
    return s, t, k

# Format test_input as a string for terminal input simulation
def format_test_input(test_input):
    s, t, k = test_input
    return f"{s}\n{t}\n{k}"

# Compile the C program
def compile_c():
    try:
        compile_command = ["gcc", os.path.join(c_folder, c_file), "-o", os.path.join(c_folder, executable_name)] # sometimes need to add -lm for math library
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
                f.write(f"input:\n{formatted_input}\n")
                f.write(f"output:\n{simulated_output}\n")
                f.write("-------------------------\n")
    finally:
        cleanup()
