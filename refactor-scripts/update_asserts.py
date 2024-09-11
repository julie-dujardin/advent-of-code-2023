import os
import re

main_repo_dir = '../src'

def update_tests_in_mod_rs():
    for day_dir in os.listdir(main_repo_dir):
        day_path = os.path.join(main_repo_dir, day_dir)

        # Skip non-directory files
        if not os.path.isdir(day_path):
            continue

        mod_rs_file = os.path.join(day_path, 'mod.rs')
        if not os.path.isfile(mod_rs_file):
            continue

        with open(mod_rs_file, 'r') as f:
            mod_rs_content = f.read()

        # Add load_results import inside `mod tests {`
        new_mod_rs_content = ''
        inside_tests_mod = False
        for line in mod_rs_content.splitlines():
            # Detect start of `mod tests` block
            if 'mod tests {' in line:
                inside_tests_mod = True
                new_mod_rs_content += line + '\n'
                new_mod_rs_content += '    use crate::load_output::load_results;\n'
            elif inside_tests_mod and 'use super::*;' in line:
                new_mod_rs_content += line + '\n'
                # Ensure `use crate::load_output::load_results;` comes after `use super::*;`
                continue
            else:
                new_mod_rs_content += line + '\n'

        # Update test functions by replacing assert_eq calls
        final_mod_rs_content = ''
        inside_p1 = False
        inside_p2 = False

        for line in new_mod_rs_content.splitlines():
            # Detect test functions (p1 or p2)
            if re.search(r'fn p1\s*\(\s*\)\s*{', line):
                inside_p1 = True
                final_mod_rs_content += line + '\n'
                final_mod_rs_content += f'    let (expected_p1, _) = load_results("{day_dir}").unwrap();\n'
            elif re.search(r'fn p2\s*\(\s*\)\s*{', line):
                inside_p2 = True
                final_mod_rs_content += line + '\n'
                final_mod_rs_content += f'    let (_, expected_p2) = load_results("{day_dir}").unwrap();\n'

            # Replace assert_eq lines in the detected part of the test function
            elif inside_p1 and 'assert_eq!' in line:
                match = re.search(r'assert_eq!\((\w+)\("([^"]+)"(?:,\s*\d+)?\),\s*(\d+)\);', line)
                if match:
                    func_name = match.group(1)
                    input_file = match.group(2)
                    result_key = input_file.split('/')[-1].split('.')[0]  # Get the filename without extension
                    new_line = f'    assert_eq!({func_name}("{input_file}"), expected_p1["{result_key}"]);'
                    final_mod_rs_content += new_line + '\n'
                else:
                    final_mod_rs_content += line + '\n'
            elif inside_p2 and 'assert_eq!' in line:
                match = re.search(r'assert_eq!\((\w+)\("([^"]+)"(?:,\s*\d+)?\),\s*(\d+)\);', line)
                if match:
                    func_name = match.group(1)
                    input_file = match.group(2)
                    result_key = input_file.split('/')[-1].split('.')[0]  # Get the filename without extension
                    new_line = f'    assert_eq!({func_name}("{input_file}"), expected_p2["{result_key}"]);'
                    final_mod_rs_content += new_line + '\n'
                else:
                    final_mod_rs_content += line + '\n'

            # Handle the end of the function
            elif inside_p1 and '}' in line:
                inside_p1 = False
                final_mod_rs_content += line + '\n'
            elif inside_p2 and '}' in line:
                inside_p2 = False
                final_mod_rs_content += line + '\n'

            else:
                final_mod_rs_content += line + '\n'

        # Write the updated mod.rs content back to the file
        with open(mod_rs_file, 'w') as f:
            f.write(final_mod_rs_content)

if __name__ == "__main__":
    update_tests_in_mod_rs()
    print("mod.rs files updated successfully.")
