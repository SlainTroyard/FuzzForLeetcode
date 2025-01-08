import os

examples = [
    {
        "input": "3\n3 abc\n5 aaaaa\n5 bcdef\n8\naabcdabc",
        "output": "3"
    },
    {
        "input": "2\n8 abababab\n2 ab\n10\nababaababa",
        "output": "2"
    }
]

output_file = "fuzz_outputs/C/weekly_contest_415_p4/outputs"

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