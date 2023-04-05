with open({{_input_:file_name}}) as ref:
    for line in ref:
        line = line.rstrip()
        {{_cursor_}}
