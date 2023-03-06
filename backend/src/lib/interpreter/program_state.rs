pub struct ProgramState{
    pub mem1: usize,
    pub mem2: usize,
    pub mem3: usize,
    pub cond: i32,
    pub location: i32,
    pub tape: Vec<i32>
}


impl ProgramState{
    pub fn new() -> ProgramState{
        let tape = vec![0; 4096];

        let state = ProgramState{
            mem1: 0,
            mem2: 1,
            mem3: 2,
            cond: 0,
            location: 0,
            tape: tape
        };

        return state;
    }       
}