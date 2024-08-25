use core::panic;
use std::fs::read_to_string;

use crate::{IntcodeComputer, Int, RunResult};

fn load_input(filename: &str) -> String {
    read_to_string(format!("test_inputs/{filename}")).unwrap()
}

fn assert_position_after_running(pos: Int, code: &str, expected: Int) {
    let mut computer = IntcodeComputer::from(code);
    assert_eq!(computer.run(), RunResult::Finished);
    assert_eq!(computer.read_at(pos), expected);
}

#[test]
fn test_day2() {
    // Examples
    assert_position_after_running(0, "1,9,10,3,2,3,11,0,99,30,40,50", 3500);
    assert_position_after_running(0, "1,0,0,0,99", 2);
    assert_position_after_running(0, "2,3,0,3,99", 2);
    assert_position_after_running(0, "2,4,4,5,99,0", 2);
    assert_position_after_running(0, "1,1,1,4,99,5,6,0,99", 30);

    // Part 1
    let code = load_input("d2.txt");
    assert_position_after_running(0, &code.replace("0,0", "12,2"), 3850704);

    // Part 2
    assert_position_after_running(0, &code.replace("0,0", "67,18"), 19690720);
}

#[test]
fn test_day5() {
    // Examples about parameter modes
    assert_position_after_running(4, "1002,4,3,4,33", 99);
    assert_position_after_running(4, "1101,100,-1,4,0", 99);

    // Examples about I/O
    let comp = IntcodeComputer::from("3,0,4,0,99");
    for i in -100_000..=100_000 {
        let mut c = comp.clone();
        c.input(i);
        assert_eq!(c.run(), RunResult::Output(i));
        assert_eq!(c.run(), RunResult::Finished);
    }

    // Part 1
    let code = load_input("d5.txt");
    let mut comp = IntcodeComputer::from(&code);
    comp.input(1);
    loop {
        if let RunResult::Output(val) = comp.run() {
            if val != 0 {
                assert_eq!(val, 14155342);
                break;
            }
        } else {
            panic!();
        }
    }
    assert_eq!(comp.run(), RunResult::Finished);

    // Part 2
    let mut comp = IntcodeComputer::from(&code);
    comp.input(5);
    assert_eq!(comp.run(), RunResult::Output(8684145));
    assert_eq!(comp.run(), RunResult::Finished);
}

#[test]
fn test_day9() {
    // Examples:

    // Quine
    let ex1 = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
    let mut comp = IntcodeComputer::from(ex1);
    for val in ex1.split(",") {
        assert_eq!(comp.run(), RunResult::Output(val.parse().unwrap()));
    }
    assert_eq!(comp.run(), RunResult::Finished);

    // Produces a 16-digit number
    let mut comp = IntcodeComputer::from("1102,34915192,34915192,7,4,7,99,0");
    assert_eq!(comp.run(), RunResult::Output(1_219_070_632_396_864));
    assert_eq!(comp.run(), RunResult::Finished);

    // Produces the large number in the middle
    let mut comp = IntcodeComputer::from("104,1125899906842624,99");
    assert_eq!(comp.run(), RunResult::Output(1125899906842624));
    assert_eq!(comp.run(), RunResult::Finished);

    // Part 1
    let code = load_input("d9.txt");
    let mut comp = IntcodeComputer::from(&code);
    comp.input(1);
    assert_eq!(comp.run(), RunResult::Output(3598076521));
    assert_eq!(comp.run(), RunResult::Finished);

    // Part 2
    let mut comp = IntcodeComputer::from(&code);
    comp.input(2);
    assert_eq!(comp.run(), RunResult::Output(90722));
    assert_eq!(comp.run(), RunResult::Finished);
}
