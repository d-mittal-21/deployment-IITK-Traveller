use std::process;
use std::io::{Cursor, Write, BufRead};

pub fn operate(location:i32, state:&mut super::program_state::ProgramState, input: &mut Cursor<&[u8]>, output: &mut String){
    let tape = &mut state.tape;
    
    match location{
        0 => {},
        1 => process::exit(0),
        2 => {
            // let i: i32 = read!();
            let mut buf = String::new();
            (*input).read_line(&mut buf).unwrap();
            // println!("{}",buf);
            tape[state.mem1] = buf.trim().parse().unwrap();
        },
        3 => {
            // let i: i32 = read!();
            let mut buf = String::new();
            (*input).read_line(&mut buf).unwrap();
            // println!("{}",buf);
            tape[state.mem2] = buf.trim().parse().unwrap();
        }
        4 => {
            tape[state.mem3] = tape[state.mem1]+tape[state.mem2];
        }
        5 => {
            tape[state.mem3] = tape[state.mem1]*tape[state.mem2];
        }
        6 => {
            tape[state.mem3] = tape[state.mem1]-tape[state.mem2];
        }
        7 => {
            tape[state.mem3] = tape[state.mem1]/tape[state.mem2];
        }
        8 => {
            tape[state.mem1] = tape[state.mem3];
        }
        9 => {
            tape[state.mem3] = tape[state.mem1];
        }
        10 => {
            tape[state.mem2] = tape[state.mem3];
        }
        11 => {
            tape[state.mem3] = tape[state.mem2];
        }
        12 => {
            // println!("{}", tape[state.mem1]);
            output.push_str(&(tape[state.mem1].to_string()));
            output.push_str("\n");
            // output.push(65 as u8);
            // let cop = output.clone();
            // *output = cop;
            // println!("{}", String::from_utf8(cop).unwrap());

        }
        13 => {
            // println!("{}", tape[state.mem2]);
            output.push_str(&(tape[state.mem2].to_string()));
            output.push_str("\n");
            // output.push(65 as u8);
            // let cop = output.clone();
            // *output = cop;
            // println!("{}", String::from_utf8(cop).unwrap());
        }
        14 => {
            if tape[state.mem1] > tape[state.mem2] {
                state.location = 15;
            }
            else{
                state.location = 16;
            }
        }
        15 => {}
        16 => {}
        17 => {
            if tape[state.mem1] < tape[state.mem2]{
                state.location = 18;
            }
            else{
                state.location = 19;
            }
        }
        18 => {}
        19 => {}
        20 => {
            if tape[state.mem1] == tape[state.mem2] {
                state.location = 21;
            }
            else{
                state.location = 22;
            }
        }
        21 => {}
        22 => {}
        23 => {
            tape[state.mem1] += 1;
        }
        24 => {
            tape[state.mem2] += 1;
        }
        25 => {
            state.cond += 1;
        }
        26 => {
            tape[state.mem1] -= 1;
        }
        27 => {
            tape[state.mem2] -= 1;
        }
        28 => {
            state.cond -= 1;
        }
        29 => {
            tape[state.mem1] = 0;
        }
        30 => {
            tape[state.mem2] = 0;
        }
        31 => {
            tape[state.mem3] = 0;
        }
        32 => {
            state.cond = 0;
        }
        33 => {
            state.mem1 += 1;
        }
        34 => {
            state.mem2 += 1;
        }
        35 => {
            state.mem3 += 1;
        }
        36 => {
            state.mem1 -= 1;
        }
        37 => {
            state.mem2 -= 1;
        }
        38 =>{
            state.mem3 -= 1;
        }
        _ => {}
    }
}