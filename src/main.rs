use std::{
    env::args,
    fs::File,
    io::{BufReader, Read, Write},
    process::Command,
};

fn main() {
    let path = args().nth(1).expect("need program to parse");

    let mut program: Vec<u8> = vec![];
    program.extend(
        b"
        #include <stdint.h>
        #include <stdio.h>
        int main() {
        uint16_t index = 0;
        char band[0x10000] = {0};
        ",
    );

    let file = BufReader::new(File::open(path).expect("file needs to exist"));
    for chr in file.bytes() {
        let chr = chr.unwrap();
        program.extend(match chr {
            b'>' => b"index++;\n" as &[u8],
            b'<' => b"index--;\n",
            b'+' => b"band[index]++;\n",
            b'-' => b"band[index]--;\n",
            b'[' => b"while (band[index]) {\n",
            b']' => b"}\n",
            b'.' => b"printf(\"%c\", band[index]);\n",
            b',' => b"band[index] = getc();\n",
            _ => b"",
        });
    }
    program.extend(b"return 0;\n}");
    File::create("tmp.c").unwrap().write_all(&program).unwrap();
    Command::new("gcc")
        .args(["-O3", "-o", "bf", "tmp.c"])
        .output()
        .unwrap();
}
