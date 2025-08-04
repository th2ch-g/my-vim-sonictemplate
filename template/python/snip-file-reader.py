with open({{_input_:file_name}}) as f:
    for idx, line in enumerate(f):
        line = line.rstrip()
        {{_cursor_}}
