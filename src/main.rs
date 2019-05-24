use std::io;

fn main() {
    println!("(^_^) Please put number. e.g. 3.14159");

    // Standard input.
    // Be sure to add "info" before the output message.
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("info Failed: stdin.read_line.");

    // Excludes trailing newlines.
    line = line.trim().parse().expect("info Failed: stdin parse.");
    // println!("> {}", line);

    let mut veca: Vec<char> = Vec::new();
    let mut vec0: Vec<char> = Vec::new();
    let mut vec1: Vec<char> = Vec::new();
    let mut vec2: Vec<char> = Vec::new();
    let mut vec3: Vec<char> = Vec::new();
    let mut vec4: Vec<char> = Vec::new();
    let mut vec5: Vec<char> = Vec::new();
    let mut vec6: Vec<char> = Vec::new();
    let mut vec7: Vec<char> = Vec::new();
    let mut vec8: Vec<char> = Vec::new();
    let mut vec9: Vec<char> = Vec::new();

    for i in 0..line.chars().count() {
        let ch = line.chars().nth(i).unwrap_or_else(|| panic!("n th fail."));
        // println!("> {}", ch);
        match ch {
            '0' => {
                veca.push(' ');
                vec0.push(ch);
                vec1.push(' ');
                vec2.push(' ');
                vec3.push(' ');
                vec4.push(' ');
                vec5.push(' ');
                vec6.push(' ');
                vec7.push(' ');
                vec8.push(' ');
                vec9.push(' ');
            }
            '1' => {
                veca.push(' ');
                vec0.push(' ');
                vec1.push(ch);
                vec2.push(' ');
                vec3.push(' ');
                vec4.push(' ');
                vec5.push(' ');
                vec6.push(' ');
                vec7.push(' ');
                vec8.push(' ');
                vec9.push(' ');
            }
            '2' => {
                veca.push(' ');
                vec0.push(' ');
                vec1.push(' ');
                vec2.push(ch);
                vec3.push(' ');
                vec4.push(' ');
                vec5.push(' ');
                vec6.push(' ');
                vec7.push(' ');
                vec8.push(' ');
                vec9.push(' ');
            }
            '3' => {
                veca.push(' ');
                vec0.push(' ');
                vec1.push(' ');
                vec2.push(' ');
                vec3.push(ch);
                vec4.push(' ');
                vec5.push(' ');
                vec6.push(' ');
                vec7.push(' ');
                vec8.push(' ');
                vec9.push(' ');
            }
            '4' => {
                veca.push(' ');
                vec0.push(' ');
                vec1.push(' ');
                vec2.push(' ');
                vec3.push(' ');
                vec4.push(ch);
                vec5.push(' ');
                vec6.push(' ');
                vec7.push(' ');
                vec8.push(' ');
                vec9.push(' ');
            }
            '5' => {
                veca.push(' ');
                vec0.push(' ');
                vec1.push(' ');
                vec2.push(' ');
                vec3.push(' ');
                vec4.push(' ');
                vec5.push(ch);
                vec6.push(' ');
                vec7.push(' ');
                vec8.push(' ');
                vec9.push(' ');
            }
            '6' => {
                veca.push(' ');
                vec0.push(' ');
                vec1.push(' ');
                vec2.push(' ');
                vec3.push(' ');
                vec4.push(' ');
                vec5.push(' ');
                vec6.push(ch);
                vec7.push(' ');
                vec8.push(' ');
                vec9.push(' ');
            }
            '7' => {
                veca.push(' ');
                vec0.push(' ');
                vec1.push(' ');
                vec2.push(' ');
                vec3.push(' ');
                vec4.push(' ');
                vec5.push(' ');
                vec6.push(' ');
                vec7.push(ch);
                vec8.push(' ');
                vec9.push(' ');
            }
            '8' => {
                veca.push(' ');
                vec0.push(' ');
                vec1.push(' ');
                vec2.push(' ');
                vec3.push(' ');
                vec4.push(' ');
                vec5.push(' ');
                vec6.push(' ');
                vec7.push(' ');
                vec8.push(ch);
                vec9.push(' ');
            }
            '9' => {
                veca.push(' ');
                vec0.push(' ');
                vec1.push(' ');
                vec2.push(' ');
                vec3.push(' ');
                vec4.push(' ');
                vec5.push(' ');
                vec6.push(' ');
                vec7.push(' ');
                vec8.push(' ');
                vec9.push(ch);
            }
            _ => {
                veca.push(ch);
                vec0.push(' ');
                vec1.push(' ');
                vec2.push(' ');
                vec3.push(' ');
                vec4.push(' ');
                vec5.push(' ');
                vec6.push(' ');
                vec7.push(' ');
                vec8.push(' ');
                vec9.push(' ');
            }
        }
    }

    {
        let s: String = veca.iter().collect();
        println!("{}", s);
    }
    {
        let s: String = vec0.iter().collect();
        println!("{}", s);
    }
    {
        let s: String = vec1.iter().collect();
        println!("{}", s);
    }
    {
        let s: String = vec2.iter().collect();
        println!("{}", s);
    }
    {
        let s: String = vec3.iter().collect();
        println!("{}", s);
    }
    {
        let s: String = vec4.iter().collect();
        println!("{}", s);
    }
    {
        let s: String = vec5.iter().collect();
        println!("{}", s);
    }
    {
        let s: String = vec6.iter().collect();
        println!("{}", s);
    }
    {
        let s: String = vec7.iter().collect();
        println!("{}", s);
    }
    {
        let s: String = vec8.iter().collect();
        println!("{}", s);
    }
    {
        let s: String = vec9.iter().collect();
        println!("{}", s);
    }
}
