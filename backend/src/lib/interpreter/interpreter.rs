use super::lexer;
use super::traveller;

// pub fn interpret(code: &str, input: &str) -> String{
//     let input = std::io::Cursor::new(input.as_bytes());
//     let mut output = String::new();
//     // output.push(65 as u8);
    
//     let code = String::from(code);
    
//     let parsed_code = match parser::parse_code(&code){
//         Ok(v) => v,
//         Err(i) => {
//             panic!("Syntax Error on line {}", i);
//         }
//     };
    
//     let graph = graph::generate_graph(parsed_code); 
//     let mut state = program_state::ProgramState::new();
//     // let serialized_graph = serde_json::to_string(&state).unwrap();
    
//     // println!("Hi");
//     graph::traverse(&graph, &mut state, input, &mut output);
//     // println!("{:?}", output);


//     output
// }

pub fn interpret(code: &str, input: &str) -> Result<String, String>{
    let input = std::io::Cursor::new(input.as_bytes());
    let mut output = String::new();

    let (tokens, lines) = lexer::store_input(code)?; //check this
    let locations = lexer::create_map();
    let (graph, increment_graph) = lexer::build_graph(&tokens, &locations, lines)?;

    let mut mem: Vec<Vec<i32>> = vec![vec![0; 2048]];
    let mut mem_flag: Vec<Vec<i8>> = vec![vec![0; 2048]];

    let mut traveller = traveller::TravelStat::new(0, 0, 1, 0, 2, 0, 0, 0, 0);

    traveller.travel(
        &mut mem,
        &mut mem_flag,
        &locations,
        &graph,
        &increment_graph,
        input,
        &mut output
    )?;

    Ok(output)
}