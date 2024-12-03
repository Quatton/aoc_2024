use std::io::Read as _;

fn get_file_v1(day: usize) -> std::fs::File {
    let path = format!("input/day{day}.txt");
    std::fs::File::open(path).unwrap()
}

pub fn read_input_v1(day: usize) -> String {
    let mut f = get_file_v1(day);
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    buf
}
