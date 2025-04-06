with open({{_input_:file_name}}) as ref:
    for idx, line in enumerate(ref):
        line = line.rstrip()
        {{_cursor_}}
