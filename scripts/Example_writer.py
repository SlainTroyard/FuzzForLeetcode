import os

examples = [
    {
        "input": "4 2\n1 2\n3 4\n2 3\n-3 0",
        "output": "-1 7 5 3"
    },
    {
        "input": "3 1\n5 5\n4 4\n3 3",
        "output": "10 8 6"
    }
]

output_file = "fuzz_outputs/CPP/weekly_contest_413_p2/outputs"

def write_examples_to_file():
    with open(output_file, "r") as f:
        existing_content = f.read()

    example_content = ""
    for example in examples:
        example_content += f"input:\n{example['input']}\noutput:\n{example['output']}\n-------------------------\n"

    with open(output_file, "w") as f:
        f.write(example_content + existing_content)

if __name__ == "__main__":
    write_examples_to_file()