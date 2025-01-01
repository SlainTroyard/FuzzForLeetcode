import os
import string
import subprocess
import random

# TODO: Configure test case generation parameters
test_cases = 100  # Number of test cases to generate

# File Configs
output_file = "../../../fuzz_outputs/C/weekly_contest_428_p2/outputs"  # Output file to store test cases and results
c_folder = "../src"  # Folder containing the C source code
c_file = "weekly_contest_428_p2.c"  # C source file name
executable_name = "solution"  # Executable name

# Generate a random currency code
def random_currency():
    return ''.join(random.choices(string.ascii_uppercase, k=random.randint(1, 3)))

# Generate a single test case following the specified rules
def generate_random_test_input():
    initial_currency = random_currency()
    n1 = random.randint(1, 10)  # Number of pairs in day 1
    n2 = random.randint(1, 10)  # Number of pairs in day 2

    # Ensure pairs1 starts with initialCurrency and no target is initialCurrency
    pairs1 = []
    rates1 = []
    current_currency = initial_currency
    for _ in range(n1):
        next_currency = random_currency()
        while next_currency == initial_currency:
            next_currency = random_currency()
        pairs1.append((current_currency, next_currency))
        rates1.append(round(random.uniform(1.0, 10.0), 2))
        current_currency = next_currency

    # Ensure pairs2 starts with the last currency from pairs1 and ends with initialCurrency
    pairs2 = []
    rates2 = []
    for i in range(n2 - 1):
        next_currency = random_currency()
        pairs2.append((current_currency, next_currency))
        rates2.append(round(random.uniform(1.0, 10.0), 2))
        current_currency = next_currency
    pairs2.append((current_currency, initial_currency))
    rates2.append(round(random.uniform(1.0, 10.0), 2))

    return initial_currency, pairs1, rates1, pairs2, rates2

# Format test_input as a string for terminal input simulation
def format_test_input(initial_currency, pairs1, rates1, pairs2, rates2):
    result = [initial_currency]
    result.append(str(len(pairs1)))
    result.extend(f"{p[0]} {p[1]} {rates1[i]}" for i, p in enumerate(pairs1))
    result.append(str(len(pairs2)))
    result.extend(f"{p[0]} {p[1]} {rates2[i]}" for i, p in enumerate(pairs2))
    return "\n".join(result)


# Compile the C program
def compile_cpp():
    try:
        compile_command = ["gcc", os.path.join(c_folder, c_file), "-o", os.path.join(c_folder, executable_name), "-lm"]
        subprocess.run(compile_command, check=True)
        print("Compilation successful.")
    except subprocess.CalledProcessError as e:
        print(f"Error during compilation: {e}")
        raise

# Simulate output for a test case by running the C program
def simulate_output(test_input):
    try:
        run_command = [os.path.join(c_folder, executable_name)]
        process = subprocess.run(run_command, input=test_input, text=True, capture_output=True, check=True)
        return process.stdout.strip()
    except subprocess.CalledProcessError as e:
        print(f"Error during execution: {e}")
        return "Error"

# Main program for testing
if __name__ == "__main__":
    try:
        compile_cpp()
        with open(output_file, "w") as f:
            for case_id in range(1, test_cases + 1):
                # Use either random test cases or fixed test cases
                if case_id == 1:
                    # Example fixed test case
                    initial_currency = "EUR"
                    pairs1 = [["EUR", "USD"], ["USD", "JPY"]]
                    rates1 = [2.0, 3.0]
                    pairs2 = [["JPY", "USD"], ["USD", "CHF"], ["CHF", "EUR"]]
                    rates2 = [4.0, 5.0, 6.0]
                else:
                    initial_currency, pairs1, rates1, pairs2, rates2 = generate_random_test_input()

                test_input = format_test_input(initial_currency, pairs1, rates1, pairs2, rates2)
                simulated_output = simulate_output(test_input)
                f.write(f"Input:\n{test_input}\n")
                f.write(f"Output:\n{simulated_output}\n")
                f.write("-------------------------\n")
    finally:
        executable_path = os.path.join(c_folder, executable_name)
        if os.path.exists(executable_path):
            os.remove(executable_path)
