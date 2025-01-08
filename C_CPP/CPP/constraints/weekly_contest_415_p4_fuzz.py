import os
import subprocess
import random
import string
import time

# TODO: Configure test case generation parameters
test_cases = 20  # Number of test cases to generate
max_words = 100  # Maximum number of words in the list
min_words = 1  # Minimum number of words in the list
max_word_length = 50000  # Maximum length of each word
max_target_length = 50000  # Maximum length of the target string
min_word_length = 1  # Minimum length of each word
min_target_length = 1  # Minimum length of the target string


# File Configs
output_file = "../../../fuzz_outputs/CPP/weekly_contest_415_p4/outputs"  # Output file to store test cases and results
cpp_folder = "../src"  # Folder containing the C++ source code
cpp_file = "weekly_contest_415_p4.cpp"  # C++ source file name
executable_name = "solution"  # Executable name

# TODO: Generate a single test case
def generate_test_input():
    random.seed(time.time())
    # 1 <= words.length <= 100, 1 <= words[i].length <= 5 * 10^4. The input is generated such that sum(words[i].length) <= 10^5. words[i] consists only of lowercase English letters. 1 <= target.length <= 5 * 10^4. target consists only of lowercase English letters.
    words = []
    target = ""
    total_length = 0
    num_words = random.randint(min_words, max_words)
    for _ in range(num_words):
        word_length = random.randint(min_word_length, max_word_length)
        total_length += word_length
        if total_length > 100000:
            break
        word = "".join(random.choices(string.ascii_lowercase, k=word_length))
        words.append(word)
    target_length = random.randint(min_target_length, max_target_length)
    target = "".join(random.choices(string.ascii_lowercase, k=target_length))
    return words, target

# TODO: Format test_input as a string for terminal input simulation
def format_test_input(test_input):
    words, target = test_input
    formatted_input = f"{len(words)}\n"
    for word in words:
        formatted_input += f"{len(word)} {word}\n"
    formatted_input += f"{len(target)}\n"
    formatted_input += f"{target}\n"
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
