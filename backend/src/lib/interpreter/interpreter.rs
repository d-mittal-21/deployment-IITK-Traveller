use super::lexer;
use super::traveller;

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