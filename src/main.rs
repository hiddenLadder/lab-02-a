use std::{env::args, process};

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 3 {
        eprintln!("figure 1: too few arguments");
        process::exit(1);
    } else if args.len() > 3 {
        eprintln!("figure 1: too many arguments");
        process::exit(1);
    }

    let x = &args[1];
    let y = &args[2];

    match (x.parse::<i32>(), y.parse::<i32>()) {
        (Ok(x), Ok(y)) => {
            if out_of_range(x, y) {
                eprintln!("figure 1: out of range");
                process::exit(1);
            }

            let h = x.pow(2) + y.pow(2);

            // первая четверть
            if (x > 10 || y > 10) && h < 400 {
                println!("inside");
                process::exit(0);
            } else if (x == 10 || y == 10) || h == 400 {
                println!("border");
                process::exit(0);
            } else if x > 0 && y > 0 && h > 400 {
                dbg!(1);
                println!("outside");
                process::exit(0);
            }

            // вторая четверть
            if (h > 100) && ((x > -20 && x < 0) && (y < 20 && y > 0)) {
                println!("inside");
                process::exit(0);
            } else if (x == -20 || y == 20) || h == 100 {
                println!("border");
                process::exit(0);
            } else if x < -20 || y > 20 {
                println!("outside");
                process::exit(0);
            }

            // третья четверть
            if (x < -10 || y < -10) && x > -20 && y > -20 {
                println!("inside");
                process::exit(0);
            } else if (x == -10 || y == -10) || (x == -20 || y == -20) {
                println!("border");
                process::exit(0);
            } else if x < -20 || y < -20 {
                println!("outside");
                process::exit(0);
            }

            // четвертая четверть
            if h > 100 && h < 400 {
                println!("inside");
                process::exit(0);
            } else if h == 100 || h == 400 {
                println!("border");
                process::exit(0);
            } else if x > 0 && y < 0 && h > 400 {
                println!("outside");
                process::exit(0);
            }

            if ((x > -10 && x <= 0) && (y < 10 && y >= 0) && h < 100)
                || ((x < 10 && x >= 0) && (y > -10 && y <= 0) && h < 100)
                || ((x < 10 && x >= 0) && (y < 10 && y >= 0))
                || ((x > -10 && x <= 0) && (y > -10 && y <= 0))
            {
                println!("outside");
                process::exit(0);
            }
        }
        // figure-1: bad format -- eсли в аргументах содержится что-то отличное от пары целых чисел
        _ => {
            eprintln!("figure 1: bad format");
            process::exit(1);
        }
    };
}

fn out_of_range(x: i32, y: i32) -> bool {
    // -100 <= x <= 100 && -100 <= y <= 100
    return !(-100 <= x && x <= 100 && -100 <= y && y <= 100);
}

// тесты
#[cfg(test)]
mod tests {
    use assert_cmd::Command;

    use super::*;
    #[test]
    fn test_out_of_range() {
        assert!(out_of_range(-101, -101));
        assert!(out_of_range(101, 101));
        assert!(!out_of_range(0, 0));
    }
    #[test]
    fn test_figure1() {
        let mut binding = Command::new("cargo");
        let cmd = binding.arg("run").arg("--");
        let assert = cmd.arg("5").arg("15").assert();
        assert.stdout(b"inside\n" as &[u8]);
    }
    #[test]
    fn test_figure2() {
        let mut binding = Command::new("cargo");
        let cmd = binding.arg("run").arg("--");
        let assert = cmd.arg("-5").arg("15").assert();
        assert.stdout(b"inside\n" as &[u8]);
    }

    #[test]
    fn test_figure3() {
        let mut binding = Command::new("cargo");
        let cmd = binding.arg("run").arg("--");
        let assert = cmd.arg("20").arg("20").assert();
        assert.stdout(b"outside\n" as &[u8]);
    }
    #[test]
    fn test_figure4() {
        let mut binding = Command::new("cargo");
        let cmd = binding.arg("run").arg("--");
        let assert = cmd.arg("-20").arg("-20").assert();
        assert.stdout(b"border\n" as &[u8]);
    }

    #[test]
    fn test_figure5() {
        let mut binding = Command::new("cargo");
        let cmd = binding.arg("run").arg("--");
        let assert = cmd.arg("-5").arg("-5").assert();
        assert.stdout(b"outside\n" as &[u8]);
    }

    #[test]
    fn test_figure6() {
        let mut binding = Command::new("cargo");
        let cmd = binding.arg("run").arg("--");
        let assert = cmd.arg("5").arg("-5").assert();
        assert.stdout(b"outside\n" as &[u8]);
    }

    #[test]
    fn test_figure7() {
        let mut binding = Command::new("cargo");
        let cmd = binding.arg("run").arg("--");
        let assert = cmd.arg("-5").arg("5").assert();
        assert.stdout(b"outside\n" as &[u8]);
    }

    #[test]
    fn test_figure8() {
        let mut binding = Command::new("cargo");
        let cmd = binding.arg("run").arg("--");
        let assert = cmd.arg("0").arg("0").assert();
        assert.stdout(b"outside\n" as &[u8]);
    }

    #[test]
    fn test_figure9() {
        let mut binding = Command::new("cargo");
        let cmd = binding.arg("run").arg("--");
        let assert = cmd.arg("-5").arg("0").assert();
        assert.stdout(b"outside\n" as &[u8]);
    }

    #[test]
    fn test_figure10() {
        let mut binding = Command::new("cargo");
        let cmd = binding.arg("run").arg("--");
        let assert = cmd.arg("0").arg("-5").assert();
        assert.stdout(b"outside\n" as &[u8]);
    }

    #[test]
    fn test_figure11() {
        let mut binding = Command::new("cargo");
        let cmd = binding.arg("run").arg("--");
        let assert = cmd.arg("-5").arg("15").assert();
        assert.stdout(b"inside\n" as &[u8]);
    }

    #[test]
    fn test_figure12() {
        let mut binding = Command::new("cargo");
        let cmd = binding.arg("run").arg("--");
        let assert = cmd.arg("-5").arg("-20").assert();
        assert.stdout(b"border\n" as &[u8]);
    }

    #[test]
    fn test_figure13() {
        let mut binding = Command::new("cargo");
        let cmd = binding.arg("run").arg("--");
        let assert = cmd.arg("5").arg("-15").assert();
        assert.stdout(b"inside\n" as &[u8]);
    }

    #[test]
    fn test_figure14() {
        let mut binding = Command::new("cargo");
        let cmd = binding.arg("run").arg("--");
        let assert = cmd.arg("-5").arg("-15").assert();
        assert.stdout(b"inside\n" as &[u8]);
    }
}
