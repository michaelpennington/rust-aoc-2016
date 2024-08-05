use std::{fs::File, io::Write};

fn main() -> std::io::Result<()> {
    let mut input = std::env::args().nth(1).unwrap();
    let output = std::env::args().nth(2).unwrap();
    let v2 = std::env::args()
        .nth(3)
        .map(|n| n.parse::<bool>().unwrap_or(false))
        .unwrap_or(false);
    let input_len = input.len();
    let mut output = File::create(output)?;
    for i in 0..100000 {
        input.truncate(input_len);
        input.push_str(&i.to_string());
        if v2 {
            let mut ans = md5::compute(&input);
            for _ in 0..2016 {
                let input = format!("{:x}", ans);
                ans = md5::compute(input);
            }
            output.write_all(format!("{:x}\n", ans).as_bytes())?;
            if i % 100 == 0 {
                println!("{i}");
            }
        } else {
            output.write_all(format!("{:x}\n", md5::compute(&input)).as_bytes())?;
        }
    }
    output.flush()?;
    Ok(())
}
