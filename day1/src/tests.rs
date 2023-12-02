pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use crate::two;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn testallnums()-> std::io::Result<()> {
        let content = "1123456";
        let file_path = &"6.txt";
        let mut file = File::create(file_path)?;
        file.write_all(content.as_bytes())?;
        let retval = two::runtwo(file_path);
        match retval {
        Ok(n) => {Ok(assert_eq!(n, 16))}
        Err(_e) => {Ok(assert!(false))}
        }
    }
    #[test]
    fn test_all_written()-> std::io::Result<()> {
        let content = "twoone";
        let file_path = &"5.txt";
        let mut file = File::create(file_path)?;
        file.write_all(content.as_bytes())?;
        let retval = two::runtwo(file_path);
        match retval {
        Ok(n) => {Ok(assert_eq!(n, 21))}
        Err(_e) => {Ok(assert!(false))}
        }
    }
    #[test]
    fn test_anoverlap()-> std::io::Result<()> {
        let content = "twone";
        let file_path = &"4.1.txt";
        let mut file = File::create(file_path)?;
        file.write_all(content.as_bytes())?;
        let retval = two::runtwo(file_path);
        match retval {
        Ok(n) => {Ok(assert_eq!(n, 21))}
        Err(_e) => {Ok(assert!(false))}
        }
    }
    #[test]
    fn test_overlap()-> std::io::Result<()> {
        let content = "twonethreee";
        let file_path = &"4.txt";
        let mut file = File::create(file_path)?;
        file.write_all(content.as_bytes())?;
        let retval = two::runtwo(file_path);
        match retval {
        Ok(n) => {Ok(assert_eq!(n, 23))}
        Err(_e) => {Ok(assert!(false))}
        }
    }
    #[test]
    fn test_double_overlap()-> std::io::Result<()>{
        let content = "twoneight";
        let file_path = &"3.txt";
        let mut file = File::create(file_path)?;
        file.write_all(content.as_bytes())?;
        let retval = two::runtwo(file_path);
        match retval {
        Ok(n) => {Ok(assert_eq!(n, 28))}
        Err(_e) => {Ok(assert!(false))}
        }
    }
    #[test]
    fn test_single_char()-> std::io::Result<()>{
        let content = "1";
        let file_path = &"2.txt";
        let mut file = File::create(file_path)?;
        file.write_all(content.as_bytes())?;
        let retval = two::runtwo(file_path);
        match retval {
        Ok(n) => {Ok(assert_eq!(n, 11))}
        Err(_e) => {Ok(assert!(false))}
        }
    }
    #[test]
    fn test_single_written()-> std::io::Result<()> {
        let content = "one";
        let file_path = &"1.txt";
        let mut file = File::create(file_path)?;
        file.write_all(content.as_bytes())?;
        let retval = two::runtwo(file_path);
        match retval {
        Ok(n) => {Ok(assert_eq!(n, 11))}
        Err(_e) => {Ok(assert!(false))}
        }
    }



    #[test]
    fn test_singleoverlap18()-> std::io::Result<()> {
        let content = "oneight";
        let file_path = &"7.txt";
        let mut file = File::create(file_path)?;
        file.write_all(content.as_bytes())?;
        let retval = two::runtwo(file_path);
        match retval {
        Ok(n) => {Ok(assert_eq!(n, 18))}
        Err(_e) => {Ok(assert!(false))}
        }
    }
    #[test]
    fn test_manyoverlap12()-> std::io::Result<()> {
        let content = "oneoenonetwoonw";
        let file_path = &"8.txt";
        let mut file = File::create(file_path)?;
        file.write_all(content.as_bytes())?;
        let retval = two::runtwo(file_path);
        match retval {
        Ok(n) => {Ok(assert_eq!(n, 12))}
        Err(_e) => {Ok(assert!(false))}
        }
    }
    #[test]
    fn test_manyoverlap11()-> std::io::Result<()> {
        let content = "oneoenonetwone";
        let file_path = &"9.txt";
        let mut file = File::create(file_path)?;
        file.write_all(content.as_bytes())?;
        let retval = two::runtwo(file_path);
        match retval {
        Ok(n) => {Ok(assert_eq!(n, 11))}
        Err(_e) => {Ok(assert!(false))}
        }
    }
    #[test]
    fn test_39()-> std::io::Result<()> {
        let content = "hey3maafnbla7nine";
        let file_path = &"10.txt";
        let mut file = File::create(file_path)?;
        file.write_all(content.as_bytes())?;
        let retval = two::runtwo(file_path);
        match retval {
        Ok(n) => {Ok(assert_eq!(n, 39))}
        Err(_e) => {Ok(assert!(false))}
        }
    }
    #[test]
    fn test_2line()-> std::io::Result<()> {
        let a = "hey3maafnbla7nine";
        let b = "segaesoneoenonetwone";
        let strings = [a, b];
        let file_path = &"11.txt";
        let mut file = File::create(file_path)?;
        for s in strings {
            writeln!(file, "{}", s)?;
        }
        let retval = two::runtwo(file_path);
        match retval {
        Ok(n) => {Ok(assert_eq!(n, 39 + 11))}
        Err(_e) => {Ok(assert!(false))}
        }
    }
    #[test]
    fn test_3lines()-> std::io::Result<()> {
        let a = "hey3maafnbla7nine";
        let b = "21";
        let c = "eighteight";
        let strings = [a, b, c];
        let file_path = &"12.txt";
        let mut file = File::create(file_path)?;
        for s in strings {
            writeln!(file, "{}", s)?;
        }
        let retval = two::runtwo(file_path);
        match retval {
        Ok(n) => {Ok(assert_eq!(n, 39 + 21 + 88))}
        Err(_e) => {Ok(assert!(false))}
        }
    }
    #[test]
    fn test_4lines()-> std::io::Result<()> {
        let a = "hey3maafnbla7nine";
        let b = "hey3maafnbla7nine";
        let c = "hey3maafnbla7nine";
        let d = "hey3maafnbla7nine";
        let strings = [a, b, c, d];
        let file_path = &"13.txt";
        let mut file = File::create(file_path)?;
        for s in strings {
            writeln!(file, "{}", s)?;
        }
        let retval = two::runtwo(file_path);
        match retval {
        Ok(n) => {Ok(assert_eq!(n, 39* 4))}
        Err(_e) => {Ok(assert!(false))}
        }
    }
}
