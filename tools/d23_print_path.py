def print_path(map_raw: str, paths: str, show_path: str):
    """Quick & dirty visualisation of the output of src/d23.rs:get_max_path_len
    Only works for the original, fully recursive version.

    input: AOC map, walk log, and path that should be printed
    """
    map = [[e for e in l] for l in map_raw.split('\n')]

    for path_line in paths:
        path_split = path_line.split('; ')
        line_is_in = path_split[0].lstrip('[').rstrip(']').split(', ')
        if show_path in line_is_in:
            x = int(path_split[2].lstrip('x='))
            y = int(path_split[3].lstrip('y='))
            map[y][x] = "O"

    print("\n".join("".join(l) for l in map))
