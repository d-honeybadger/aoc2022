pub(crate) mod util {
    use std::fs;

    // TODO: figure out why it's complaining that this function is dead code even though it'd definitely getting used
    pub(crate) fn read_input(args: &Vec<String>) -> (Vec<String>, String) {
        if args.len() != 3 {
            eprintln!("incorrect number of arguments (must be 2)");
            std::process::exit(1)
        }
        let filename = &args[1];
    
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let lines: Vec<String> = contents.lines().map(|s|s.into()).collect();
    
        let part: String = args[2].to_string();
    
        return (lines, part)
    }
}
