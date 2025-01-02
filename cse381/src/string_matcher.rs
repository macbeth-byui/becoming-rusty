use std::collections::HashMap;

/* Use an FSM to determine how many matches a text contains.
 */
pub fn match_pattern(text : &str, fsm : &[HashMap<char, usize>]) -> Result<Vec<usize>, String> {
    // The accepting state from the build_fsm function will always be the last state
    let accepting = fsm.len() - 1;

    // Start at the first state
    let mut state = 0;
    
    let mut results = Vec::<usize>::new();

    // Visit each letter of the text.
    for (index, letter) in text.chars().enumerate() {
        // Get the next state based on the letter
        state = match fsm[state].get(&letter) {
            Some(next) => *next,
            None => return Err("Invalid letter in text".to_string())
        };
        // Save index if we are in an accepting state now
        if state == accepting {
            results.push(index);
        }
    }

    Ok(results)
}

/* Create an FSM for the pattern and valid input list of characeters.
 * The FSM is represented by a list of maps where each map represents a state and the keys
 * in each map contain all the possible inputs that can be received in that state.
 */
pub fn build_fsm(pattern : &str, inputs : &str) -> Vec<HashMap<char, usize>> {
    let mut fsm = Vec::<HashMap<char, usize>>::new();
    // Each state of the FSM represents pattern[..k]
    for k in 0..=pattern.len() {
        let mut map = HashMap::<char, usize>::new();
        // Consider receiving each input character in the current state
        for a in inputs.chars() {
            // pka represents the current full input.  i identifies the state that 
            // matches pka.  
            let pka = format!("{}{}", &pattern[..k], a);
            let mut i = std::cmp::min(k+1, pattern.len());

            // Matching means that pka ends with whatever state i matches.
            while !pka.ends_with(&pattern[..i]) {
                i -= 1;
            }
            map.insert(a, i);
        }
        fsm.push(map);
    }

    fsm
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_build_fsm1() {
        let fsm = build_fsm("AAC", "ACGT");
        assert_eq!(fsm, vec![
            vec![('A', 1), ('C', 0), ('G', 0), ('T', 0)].into_iter().collect(),
            vec![('A', 2), ('C', 0), ('G', 0), ('T', 0)].into_iter().collect(),
            vec![('A', 2), ('C', 3), ('G', 0), ('T', 0)].into_iter().collect(),
            vec![('A', 1), ('C', 0), ('G', 0), ('T', 0)].into_iter().collect(),
        ]);
    }

    #[test]
    fn test2_build_fsm1() {
        let fsm = build_fsm("CBCBA", "ABC");
        assert_eq!(fsm, vec![
            vec![('A', 0), ('B', 0), ('C', 1)].into_iter().collect(),
            vec![('A', 0), ('B', 2), ('C', 1)].into_iter().collect(),
            vec![('A', 0), ('B', 0), ('C', 3)].into_iter().collect(),
            vec![('A', 0), ('B', 4), ('C', 1)].into_iter().collect(),        
            vec![('A', 5), ('B', 0), ('C', 3)].into_iter().collect(),        
            vec![('A', 0), ('B', 0), ('C', 1)].into_iter().collect(),        
        ]);
    }

    #[test]
    fn test3_match1() {
        let fsm = build_fsm("AAC", "ACGT");
        let results = match_pattern("GTAACAGTAAACG", &fsm);
        assert!(results.is_ok());
        assert_eq!(results.unwrap(), vec![4, 11]);
    }

    #[test]
    fn test4_match2() {
        let fsm = build_fsm("AA", "ACGT");
        let results = match_pattern("GTAACAGTAAACG", &fsm);
        assert!(results.is_ok());
        assert_eq!(results.unwrap(), vec![3, 9, 10]);
    }

    #[test]
    fn test5_match3() {
        let fsm = build_fsm("CBC", "ABC");
        let results = match_pattern("ABCBCABCBCBC", &fsm);
        assert!(results.is_ok());
        assert_eq!(results.unwrap(), vec![4, 9, 11]);
    }

    #[test]
    fn test6_no_matches() {
        let fsm = build_fsm("AACT", "ACGT");
        let results = match_pattern("GTAACAGTAAACG", &fsm);
        assert!(results.is_ok());
        assert_eq!(results.unwrap(), vec![]);
    }
}