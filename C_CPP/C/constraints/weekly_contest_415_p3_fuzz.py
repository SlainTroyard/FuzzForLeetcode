import os
import subprocess
import random
import string
import time

# TODO: Configure test case generation parameters
test_cases = 20  # Number of test cases to generate
max_words = 100  # Maximum number of words in the list
min_words = 1  # Minimum number of words in the list
max_word_length = 5000  # Maximum length of each word
max_target_length = 5000  # Maximum length of the target string
min_word_length = 1  # Minimum length of each word
min_target_length = 1  # Minimum length of the target string
max_total_length = 100000  # Total length constraint for all words combined

# File Configs
output_file = "../../../fuzz_outputs/C/weekly_contest_415_p3/outputs"  # Output file to store test cases and results
c_folder = "../src"  # Folder containing the C source code
c_file = "weekly_contest_415_p3.c"  # C source file name
executable_name = "solution"  # Executable name

# TODO: Generate a single test case
# Generate a single test case ensuring at least 50% solvable cases
def generate_test_input():
    random.seed(time.time())
    words = []
    total_length = 0
    num_words = random.randint(min_words, max_words)

    # Decide whether to generate a solvable test case
    solvable = random.random() < 0.5  # 50% chance to generate a solvable test case

    if solvable:
        # Generate words first and ensure the target can be formed from them
        for _ in range(num_words):
            word_length = random.randint(min_word_length, max_word_length)
            total_length += word_length
            if total_length > max_total_length:
                break
            word = "".join(random.choices(string.ascii_lowercase, k=word_length))
            words.append(word)
            shuffled_words = words.copy()
            random.shuffle(shuffled_words)

            target = ""
        # Randomly choose some words with length > 1 and allow repetition
            while len(target) < max_target_length:
                selected_word = random.choice([word for word in shuffled_words if len(word) > 1])
                cut_index = random.randint(1, len(selected_word))  # Cut index cannot be zero to avoid empty slices
                target += selected_word[:cut_index]
                
                # Stop if target exceeds the length constraint
                if len(target) >= max_target_length:
                    target = target[:max_target_length]
                    break
        
    else:
        # Generate words randomly without ensuring the target is constructible
        for _ in range(num_words):
            word_length = random.randint(min_word_length, max_word_length)
            total_length += word_length
            if total_length > max_total_length:
                break
            word = "".join(random.choices(string.ascii_lowercase, k=word_length))
            words.append(word)

        # Generate a completely random target string
        target_length = random.randint(min_target_length, len(word))
        target = "".join(random.choices(string.ascii_lowercase, k=target_length))

    return words, target

# Format test_input as a string for terminal input simulation
def format_test_input(test_input):
    words, target = test_input
    formatted_input = f"{len(words)}\n"
    for word in words:
        formatted_input += f"{len(word)} {word}\n"
    formatted_input += f"{len(target)}\n{target}\n"
    return formatted_input
    
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
        # 输出当前运行目录
        print(os.getcwd())
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
