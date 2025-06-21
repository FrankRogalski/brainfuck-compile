use std::{
    env::args,
    fs::File,
    io::{BufReader, Read, Write},
    process::{Command, Stdio},
};

fn main() {
    let path = args().nth(1).expect("need program to parse");

    let mut program: Vec<u8> = Vec::with_capacity(32 * 1024);
    program.extend(
        b"#include <stdint.h>
#include <stdio.h>

int main() {
    uint16_t index = 0;
    char band[0x10000] = {0};
",
    );

    let file = BufReader::new(File::open(path).expect("file needs to exist"));
    let mut depth = 1;
    for chr in file.bytes() {
        let chr = chr.unwrap();
        for _ in 0..depth {
            program.extend(b"    ");
        }
        program.extend(match chr {
            b'>' => b"index++;\n" as &[u8],
            b'<' => b"index--;\n",
            b'+' => b"band[index]++;\n",
            b'-' => b"band[index]--;\n",
            b'[' => {
                depth += 1;
                b"while (band[index]) {\n"
            }
            b']' => {
                depth -= 1;
                b"}\n"
            }
            b'.' => b"printf(\"%c\", band[index]);\n",
            b',' => b"band[index] = getc();\n",
            _ => b"",
        });
    }
    program.extend(b"return 0;\n}");
    let mut child = Command::new("gcc")
        .stdin(Stdio::piped())
        .args(["-O3", "-x", "c", "-o", "bf", "-"])
        .spawn()
        .unwrap();

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(&program).unwrap();
    }

    child.wait().unwrap();
}
