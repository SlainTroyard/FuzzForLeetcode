import os

examples = [
    {
        "input": "3 2\n6\n0\n3",
        "output": "4"
    },
    {
        "input": "4 5\n2\n6\n13\n13",
        "output": "5"
    }
]

output_file = "fuzz_outputs/CPP/weekly_contest_414_p2/outputs"

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