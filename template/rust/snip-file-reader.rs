for line in BufReader::new(File::open({{_input_:file_name}}).unwrap()).lines() {
    let line = line.unwrap();
    {{_cursor_}}
}
