import os
import subprocess
import random
import string

# TODO: Configure test case generation parameters
test_cases = 100  # Number of test cases to generate

# File Configs
output_file = "../../../fuzz_outputs/CPP/weekly_contest_425_p2/outputs"  # Output file to store test cases and results
cpp_folder = "../src"  # Folder containing the C++ source code
cpp_file = "weekly_contest_425_p2.cpp"  # C++ source file name
executable_name = "solution"  # Executable name

def generate_random_string(length):
    """
    Generates a random string of lowercase English letters with the specified length.
    """
    return ''.join(random.choices(string.ascii_lowercase, k=length))

def get_divisors(n):
    """
    Returns a sorted list of all divisors of a given number n.
    """
    divisors = set()
    for i in range(1, int(n**0.5) + 1):
        if n % i == 0:
            divisors.add(i)
            divisors.add(n // i)
    return sorted(divisors)

def generate_test_input():
    """
    Generates a single test case consisting of two anagram strings s and t, and an integer k.
    
    Returns:
        tuple: (s, t, k) where:
            - s (str): Original string
            - t (str): Anagram of s
            - k (int): Divisor of the length of s
    """
    # Generate n, the length of the strings, ensuring 1 <= n <= 200,000
    n = random.randint(1, 200000)
    
    # Get all divisors of n to ensure k divides n
    divisors = get_divisors(n)
    
    # Randomly select k from the list of divisors
    k = random.choice(divisors)
    
    # Generate string s with random lowercase letters
    s = generate_random_string(n)
    
    # Generate string t by shuffling the characters of s to ensure it's an anagram
    t = list(s)
    random.shuffle(t)
    t = ''.join(t)
    
    return s, t, k

# Format test_input as a string for terminal input simulation
def format_test_input(test_input):
    s, t, k = test_input
    return f"{s}\n{t}\n{k}"

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
                # Write formatted input and output to the file
                f.write(f"input:\n{formatted_input}\n")
                f.write(f"output:\n{simulated_output}\n")
                f.write("-------------------------\n")
    finally:
        cleanup()
