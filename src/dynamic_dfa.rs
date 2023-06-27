use serde_json::{Value};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;

fn error_dotcomma(
    buffer: &Vec<TokenVector>,
    pos: &mut i32, )-> String{
    return format!("Error in line: {}\n   Missing ';' after {} token",buffer[(*pos-1) as usize].ln, buffer[(*pos-1) as usize].word)
}

fn error_expression(
    buffer: &Vec<TokenVector>,
    pos: &mut i32, )-> String{
    return format!("Error in line: {}\n   Missing expression after {} token",buffer[(*pos-1) as usize].ln, buffer[(*pos-1) as usize].word)
}

fn error_id(
    buffer: &Vec<TokenVector>,
    pos: &mut i32, )-> String{
    return format!("Error in line: {}\n   Missing id after {} token",buffer[(*pos-1) as usize].ln, buffer[(*pos-1) as usize].word)
}

fn error_type(
    buffer: &Vec<TokenVector>,
    pos: &mut i32, )-> String{
    return format!("Error in line: {}\n   Missing type before {} token",buffer[*pos as usize].ln, buffer[*pos as usize].word)
}

fn error_close_par(
    buffer: &Vec<TokenVector>,
    pos: &mut i32, )-> String{
    return format!("Error in line: {}\n   Missing ')' after {} token",buffer[(*pos-1) as usize].ln, buffer[(*pos-1) as usize].word)
}

fn error_close_key(
    buffer: &Vec<TokenVector>,
    pos: &mut i32, )-> String{
    return format!("Error in line: {}\n   Missing '}}' after {} token",buffer[(*pos-1) as usize].ln, buffer[(*pos-1) as usize].word)
}

fn error_main(
    buffer: &Vec<TokenVector>,
    pos: &mut i32, )-> String{
    return format!("Error in line: {}\n   Missing or wrong 'main' statement",buffer[(*pos-1) as usize].ln)}

fn error_if(
    buffer: &Vec<TokenVector>,
    pos: &mut i32, )-> String{
    return format!("Error in line: {}\n   Missing or wrong 'if' statement",buffer[(*pos-1) as usize].ln)}

fn error_exp(
    buffer: &Vec<TokenVector>,
    pos: &mut i32, )-> String{
    return format!("Error in line: {}\n   Missing expression on 'if' statement",buffer[(*pos-1) as usize].ln)}

fn error_else(
    buffer: &Vec<TokenVector>,
    pos: &mut i32, )-> String{
    return format!("Error in line: {}\n   Missing or wrong 'else' statement",buffer[(*pos-1) as usize].ln)}

// struct TokenInTable {
//     type_token: String,
//     line: usize,
//     pos: usize,
//     value: String,
// }
struct TokenVector{
    val: String,
    word: String,
    ln: usize,
    ps: usize,
}
    
fn get_states(json: &Value) -> Vec<String> {
        let input_states: &Vec<Value> = json["states"].as_array().unwrap();
    
        let mut states: Vec<String> = Vec::new();
    
        for state in input_states {
            states.push(state.as_str().unwrap().to_string());
        }
    
        states
}

fn get_tokens(json: &Value) -> Vec<String> {
    let input_tokens: &Vec<Value> = json["tokens"].as_array().unwrap();

    let mut tokens: Vec<String> = Vec::new();

    for token in input_tokens {
        tokens.push(token.as_str().unwrap().to_string());
    }

    tokens
}

fn get_reserved_words(json: &Value) -> HashMap<String, Vec<String>> {
    let mut reserved_words: &Vec<Value> = json["reserved"]["type"].as_array().unwrap();

    let mut reserved: HashMap<String, Vec<String>> = HashMap::new();
    let mut words: Vec<String> = Vec::new();

    for word in reserved_words {
        words.push(word.as_str().unwrap().to_string());
    }
    reserved.insert("type".to_string(), words);
    reserved_words= json["reserved"]["words"].as_array().unwrap();
    words = Vec::new();
    for word in reserved_words {
        words.push(word.as_str().unwrap().to_string());
    }
    reserved.insert("words".to_string(), words);
    reserved
}

fn get_transitions(
    json: &Value,
    states: &Vec<String>,
    tokens: &Vec<String>,
) -> HashMap<String, HashMap<String, String>> {
    let mut dfa: HashMap<String, HashMap<String, String>> = HashMap::new();

    for state in states {
        let mut transition_for_state: HashMap<String, String> = HashMap::new();

        for token in tokens {
            let transition = json["transitions"][state.as_str()][token.as_str()]
                .as_str()
                .unwrap()
                .to_string();
            transition_for_state.insert(token.to_string(), transition);
        }

        dfa.insert(state.to_string(), transition_for_state);
    }

    dfa
}

fn get_json() -> Value {
    // Read input_dfa.json
    let file = File::open("src/input_dfa.json").unwrap();
    let reader = BufReader::new(file);
    // Parse the json file
    let json: Value = serde_json::from_reader(reader).unwrap();

    json
}

fn get_accepted_states(json: &Value) -> HashSet<String> {
    let input_accepted_states: &Vec<Value> = json["accepted_states"].as_array().unwrap();

    let mut accepted_states: HashSet<String> = HashSet::new();

    for state in input_accepted_states {
        accepted_states.insert(state.as_str().unwrap().to_string());
    }

    accepted_states
}

fn analyze_input(
    input_token_array: &Vec<String>,
    reserved_words: &HashMap<String, Vec<String>>,
    dfa: &HashMap<String, HashMap<String, String>>,
    initial_state: &String,
    accepted_states: &HashSet<String>
) ->    Vec<TokenVector> {

    let mut current_state = initial_state.to_string();
    let mut buffer = String::new();
    let mut data = Vec::<TokenVector>::new();


    for (line, lines) in input_token_array.iter().enumerate() {
        println!("{}", lines);
        for (position, chars) in lines.chars().enumerate() {
            let next_state = &dfa[&current_state][&chars.to_string()];
                if (!accepted_states.contains(next_state) ||  !accepted_states.contains(&current_state)) && !buffer.is_empty(){ 
                    if !current_state.eq("error"){
                        if !buffer.contains(" ") && !buffer.contains("\n"){
                            let mut token_struct = TokenVector{
                                val: "".to_string(),
                                word: buffer.to_string(),
                                ln: line+1,
                                ps: position,
                            };
                            if !reserved_words["type"].contains(&buffer) && !reserved_words["words"].contains(&buffer){
                                token_struct.val = current_state.to_string();
                            } else{
                                token_struct.val = buffer.to_string();
                            }

                            data.push(token_struct);
                        }
                    }else{
                        println!("Error:{}:{}    in token: {}", line, position, buffer);
                    }
                    buffer = String::new();
                }
                buffer.push_str(chars.to_string().as_str());
                current_state = next_state.to_string();
        }
        if !current_state.eq("error"){
            let token_struct= TokenVector{
                val: current_state.to_string(),
                word: buffer.to_string(),
                ln: line+1,
                ps: input_token_array[line].len(),
            };
            data.push(token_struct);
        }
        buffer = String::new();
        current_state = initial_state.to_string();
    }
    data.push(TokenVector{
        val: "$".to_string(),
        word: "$".to_string(),
        ln: 0,
        ps: 0,
    });
    data

}

pub fn validate(exp: &Vec<String>) -> Vec<String> {
    let json = get_json();
    let states = get_states(&json);
    let tokens = get_tokens(&json);
    let reserved_words = get_reserved_words(&json);
    //println!("Reserved words: {:?}", reserved_words);
    let accepted_states: HashSet<String> = get_accepted_states(&json);
    let dfa = get_transitions(&json, &states, &tokens);
    let initial_state = json["initial_state"].as_str().unwrap().to_string();
    let array_input_tokens = analyze_input(exp, &reserved_words, &dfa, &initial_state, &accepted_states);
    // for tokens in &array_input_tokens{
    //     println!("token: {}        token_val: {}    token_line: {}    token_pos: {}", tokens.word, tokens.val, tokens.ln, tokens.ps);
    // }
    //println!("{:?}", token_table);
    grammar_check(&array_input_tokens)
}


fn grammar_check(buffer: &Vec<TokenVector>) -> Vec<String>{
    let mut token: String = "".to_string();
    let mut pos = 0;
    let mut vec_error:Vec<String> = Vec::new(); //Vector de errores

    program(&mut pos,&buffer,&mut token, &mut vec_error);
    vec_error
}

fn program(
    pos : &mut i32,
    buffer:&Vec<TokenVector>,
    token: &mut String,
    vec_error: &mut Vec<String>)-> i32{
        if dec_list(pos,buffer,token,vec_error) == 1{
            if inicio(pos,buffer,token,vec_error) == 1{
                return 1;
            }
        }
        1
    }

fn inicio(
    pos : &mut i32,
    buffer:&Vec<TokenVector>,
    token: &mut String,
    vec_error: &mut Vec<String>)-> i32{
        let mut actual_pos = *pos;
        if read(pos,buffer,token) == 14{ // main
            actual_pos = *pos;
            if read(pos,buffer,token) == 5{ // (
                actual_pos = *pos;
                if read(pos,buffer,token) == 6{  //  )
                    actual_pos = *pos;
                    if read(pos,buffer,token) == 15{  //  {
                        if sentencia(pos,buffer,token,vec_error) == 1{ 
                            actual_pos = *pos;
                            if buffer[buffer.len()-2].val == '}'.to_string(){  //  }
                                return 1;
                            }else{*pos = actual_pos; vec_error.push(error_close_key(buffer, pos));return 1;}
                        }
                    }else{*pos = actual_pos;}
                }else{*pos = actual_pos;}
            }else{*pos = actual_pos;}
        }else{*pos = actual_pos;}
        vec_error.push(error_main(buffer, pos));
    0
    }

fn sentencia(
    pos : &mut i32,
    buffer:&Vec<TokenVector>,
    token: &mut String,
    vec_error: &mut Vec<String>)-> i32{
        //println!("buffer: {} line: {}", buffer[*pos as usize].word, buffer[*pos as usize].ln);
        if asig_list(pos,buffer,token,vec_error) == 1{
            if sentencia(pos, buffer, token, vec_error) == 1{
                return 1;
            }
        }
        if condicion(pos,buffer,token,vec_error) == 1{
            if sentencia(pos, buffer, token, vec_error) == 1{
                return 1;
            }
        }
        return 1
    }

fn condicion(
    pos : &mut i32,
    buffer:&Vec<TokenVector>,
    token: &mut String,
    vec_error: &mut Vec<String>)-> i32{
        let mut actual_pos = *pos;
        if read(pos,buffer,token) == 17{  // if
            actual_pos = *pos;
            if read(pos,buffer,token) == 5{  // (
                if expresion(pos,buffer,token,vec_error) == 1{
                    actual_pos = *pos;
                    if read(pos,buffer,token) == 6{  // )
                        actual_pos = *pos;
                        if read(pos,buffer,token) == 15{  // {
                            //println!("buffer: {} line: {}", buffer[*pos as usize].word, buffer[*pos as usize].ln);
                            if sentencia(pos,buffer,token,vec_error) == 1{
                                actual_pos = *pos;
                                if read(pos,buffer,token) == 16{  //  }
                                    actual_pos = *pos;
                                    if sino(pos,buffer,token, vec_error) == 1{  //  else
                                        return 1;
                                    }else{*pos = actual_pos;return 1;}
                                }else{*pos = actual_pos;vec_error.push(error_if(buffer, pos))}
                            }else{return 1;}
                        } else {*pos = actual_pos;vec_error.push(error_if(buffer, pos));}
                    } else {*pos = actual_pos; vec_error.push(error_if(buffer, pos));}
                }else{vec_error.push(error_exp(buffer, pos));}
            } else {*pos = actual_pos; vec_error.push(error_if(buffer, pos));}
        } else {*pos = actual_pos;}
        return 0
    }

fn sino(
    pos : &mut i32,
    buffer:&Vec<TokenVector>,
    token: &mut String,
    vec_error: &mut Vec<String>) -> i32{
        let mut actual_pos = *pos;
        if read(pos,buffer,token) == 18{  // else
            actual_pos = *pos;
            if read(pos,buffer,token) == 15{  // {
                if sentencia(pos,buffer,token,vec_error) == 1{
                    actual_pos = *pos;
                    if read(pos,buffer,token) == 16{  //  }
                        return 1;
                    }else{*pos = actual_pos;vec_error.push(error_else(buffer, pos));}
                }else{return 1;}
            }else{*pos = actual_pos;vec_error.push(error_else(buffer, pos));}
        }else{*pos = actual_pos;}
        return 0
    }

fn expresion(
    pos : &mut i32,
    buffer:&Vec<TokenVector>,
    token: &mut String,
    vec_error: &mut Vec<String>)-> i32{
        if e(pos,buffer,token,vec_error) == 1{
            if relacion(pos,buffer,token) == 1{
                if e(pos,buffer,token,vec_error) == 1{
                    return 1;
                }
            }
        }
        return 0
    }

fn relacion(
    pos : &mut i32,
    buffer:&Vec<TokenVector>,
    token: &mut String) -> i32{
        let actual_pos = *pos;
        if read(pos,buffer,token) == 13{  //  =
            if read(pos,buffer,token) == 13{  //  =
                return 1;
            }
        }
        *pos = actual_pos;
        if read(pos,buffer,token) == 21{  //  !
            if read(pos,buffer,token) == 13{  //  =
                return 1;
            }
        }
        *pos = actual_pos;
        return 0
    }

fn asig_list(
    pos : &mut i32,
    buffer:&Vec<TokenVector>,
    token: &mut String,
    vec_error: &mut Vec<String>)-> i32{
        if asignacion(pos,buffer,token,vec_error) == 1{
            if asig_list(pos,buffer,token,vec_error) == 1{
                return 1;
            }
            return 1;
        }
        return 0
    }

fn asignacion(
    pos : &mut i32,
    buffer:&Vec<TokenVector>,
    token: &mut String,
    vec_error: &mut Vec<String>)-> i32{
        if varasig_list(pos,buffer,token,vec_error) == 1{
            let actual_pos = *pos;
            if read(pos,buffer,token) == 12{  // ;
                return 1;
            } else{*pos = actual_pos; vec_error.push(error_dotcomma(buffer, pos)); return 1;}  //CAMBIADO PARA LEER ERRORES
        }
        return 0
    }

fn varasig_list(
    pos : &mut i32,
    buffer:&Vec<TokenVector>,
    token: &mut String,
    vec_error: &mut Vec<String>)-> i32{
        if var_asig_ini(pos,buffer,token,vec_error) == 1{
            let actual_pos = *pos;
            if read(pos,buffer,token) == 11{  // ,
                if varasig_list(pos,buffer,token,vec_error) == 1{
                    return 1;
                }
            }else{*pos = actual_pos; return 1;}
        }
        return 0
    }

fn var_asig_ini(
    pos : &mut i32,
    buffer:&Vec<TokenVector>,
    token: &mut String,
    vec_error: &mut Vec<String>)-> i32{
        let actual_pos = *pos;
        if read(pos,buffer,token) == 8 { // id
            if dec_asig(pos,buffer,token,vec_error) == 1{
                return 1;
            }
        } else{*pos = actual_pos;}    // CAMBIADO PARA EJECUTARSE AUNQUE FALTE ID
        return 0
    }

fn dec_asig(
    pos : &mut i32,
    buffer:&Vec<TokenVector>,
    token: &mut String,
    vec_error: &mut Vec<String>)-> i32{
        let actual_pos = *pos;
        if read(pos,buffer,token) == 13{ // =
            if e(pos,buffer,token,vec_error) == 1{
                return 1;
            } else {vec_error.push(error_expression(buffer,pos));return 1;} //CAMBIADO PARA EJECUTARSE AUNQUE FALTE EXPRESION
        }else {*pos = actual_pos;}
        return 1
    }

fn dec_list(
    pos : &mut i32,
    buffer:&Vec<TokenVector>,
    token: &mut String,
    vec_error: &mut Vec<String>)-> i32{
        if dec(pos,buffer,token,vec_error) == 1{
            if dec_list(pos,buffer,token,vec_error) == 1{
                return 1;
            }
        }
        return 1
    }

fn dec(
    pos : &mut i32,
    buffer:&Vec<TokenVector>,
    token: &mut String,
    vec_error: &mut Vec<String>)->   i32{
        if type_var(pos,buffer,token,vec_error) == 1{
            if vardec_list(pos,buffer,token,vec_error) == 1{
                let actual_pos = *pos;
                if read(pos,buffer,token) == 12{  //;
                    return 1;
                }else{*pos = actual_pos; vec_error.push(error_dotcomma(buffer,pos)); return 1;}  // CAMBIADO PARA EJECUTARSE AUNQUE FALTE ;
            }
        }
        return 0
    }

fn type_var(
    pos : &mut i32,
    buffer:&Vec<TokenVector>,
    token: &mut String,
    vec_error: &mut Vec<String>)-> i32{
    let actual_pos = *pos;
    if read(pos,buffer,token) == 7{
        return 1;
    }
    *pos = actual_pos;
    if read(pos,buffer,token) == 9{
        return 1;
    }
    *pos = actual_pos;
    if read(pos,buffer,token) == 10{
        return 1;
    }
    *pos = actual_pos;
    if buffer[*pos as usize].val == "id"{
        vec_error.push(error_type(buffer,pos));
        return 1;
    }
    return 0
    }

fn vardec_list(
    pos : &mut i32,
    buffer:&Vec<TokenVector>,
    token: &mut String,
    vec_error: &mut Vec<String>)-> i32{
        if var_dec_ini(pos,buffer,token,vec_error) == 1{
            let actual_pos = *pos;
            if read(pos,buffer,token) == 11{  // ,
                if vardec_list(pos,buffer,token,vec_error) == 1{
                    return 1;
                }
            }else{*pos = actual_pos; return 1;}
        }
        return 0
    }

fn var_dec_ini(
    pos: &mut i32,
    buffer:&Vec<TokenVector>,
    token: &mut String,
    vec_error: &mut Vec<String>)-> i32{
        let actual_pos = *pos;
        if read(pos,buffer,token) == 8 || (buffer[*pos as usize].val == '='.to_string()) {  // id
            if asig(pos,buffer,token,vec_error) == 1{
                return 1;
            }
        }else{*pos = actual_pos; vec_error.push(error_id(buffer,pos));return 1;}    // CAMBIADO PARA EJECUTARSE AUNQUE FALTE ID
        return 0
    }

fn asig(
    pos: &mut i32,
    buffer:&Vec<TokenVector>,
    token: &mut String,
    vec_error: &mut Vec<String>)-> i32{
        let actual_pos = *pos;
        if read(pos,buffer,token) == 13{  // =
            if e(pos,buffer,token,vec_error) == 1{
                return 1;
            } else {vec_error.push(error_expression(buffer,pos));return 1;} //CAMBIADO PARA EJECUTARSE AUNQUE FALTE EXPRESION
        } else{*pos = actual_pos;}
        return 1
    }

fn funcion(
    pos : &mut i32,
    buffer:&Vec<TokenVector>,
    token: &mut String,
    vec_error: &mut Vec<String>)-> i32{
        if f_pow(pos,buffer,token,vec_error) == 1{
            return 1;
        }
        if f_sqrt(pos,buffer,token,vec_error) == 1{
            return 1;
        }
        return 0
    }

fn f_pow(
    pos: &mut i32,
    buffer:&Vec<TokenVector>,
    token: &mut String,
    vec_error: &mut Vec<String>)-> i32{
        let mut actual_pos = *pos;
        
        if read(pos,buffer,token) == 19{ // pow
            actual_pos = *pos;
            if read(pos,buffer,token) == 5{ // (
                if e(pos,buffer,token,vec_error) == 1{
                    actual_pos = *pos;
                    if read(pos,buffer,token) == 11{ // ,
                        if e(pos,buffer,token,vec_error) == 1{
                            actual_pos = *pos;
                            if read(pos,buffer,token) == 6{ // )
                                return 1;
                            } else{ *pos = actual_pos; return 0;}
                        }else {vec_error.push(format!("Error in line: {}\n   Missing expression before {}",buffer[*pos as usize].ln,buffer[*pos as usize].word));return 0;}
                    } else{ *pos = actual_pos; vec_error.push(format!("Error in line: {}\n   Missing ',' before {}",buffer[*pos as usize].ln,buffer[*pos as usize].word));return 0;}
                } else {vec_error.push(format!("Error in line: {}\n   Missing expression before {}",buffer[*pos as usize].ln,buffer[*pos as usize].word));return 0;}
            } else{ *pos = actual_pos; vec_error.push(format!("Error in line: {}\n   Missing '(' before {}",buffer[*pos as usize].ln,buffer[*pos as usize].word)); return 0;}
        }
        *pos = actual_pos;
        return 0
    }

fn f_sqrt(
    pos: &mut i32,
    buffer:&Vec<TokenVector>,
    token: &mut String,
    vec_error: &mut Vec<String>)-> i32{
        let mut actual_pos = *pos;
        if read(pos,buffer,token) == 20{ // sqrt
            actual_pos = *pos;
            if read(pos,buffer,token) == 5{ // (
                if e(pos,buffer,token,vec_error) == 1{
                    actual_pos = *pos;
                    if read(pos,buffer,token) == 6{ // )
                        return 1;
                    } else{ *pos = actual_pos; return 0;}
                }else{vec_error.push(format!("Error in line: {}\n   Missing expression before {}",buffer[*pos as usize].ln,buffer[*pos as usize].word));}
            } else{*pos = actual_pos;vec_error.push(format!("Error in line: {}\n   Missing '(' before {}",buffer[*pos as usize].ln,buffer[*pos as usize].word));}
        }else{*pos = actual_pos}
        return 0
}

fn ex(
    pos : &mut i32,
    buffer:&Vec<TokenVector>,
    token: &mut String,
    vec_error: &mut Vec<String>)-> i32{
    let actual_pos = *pos;
    if read(pos,buffer,token) == 1{
        if t(pos,buffer,token,vec_error) == 1{
            return ex(pos,buffer,token,vec_error)
        } else {return 0}
    }
    *pos = actual_pos;
    if read(pos,buffer,token) == 2{
        if t(pos,buffer,token,vec_error) == 1{
            return ex(pos,buffer,token,vec_error)
        } else {return 0}
    }
    *pos = actual_pos;
    return 1
}

fn tx(
    pos : &mut i32,
    buffer:&Vec<TokenVector>,
    token: &mut String,
    vec_error: &mut Vec<String>)-> i32{
    let actual_pos = *pos;
    if read(pos,buffer,token) == 3{
        if f(pos,buffer,token,vec_error) == 1{
            return tx(pos,buffer,token,vec_error)
        } else {return 0}
    }
    *pos = actual_pos;
    if read(pos,buffer,token) == 4{
        if f(pos,buffer,token,vec_error) == 1{
            return tx(pos,buffer,token,vec_error)
        } else {return 0}
    }
    *pos = actual_pos;
    return 1
}

fn f(
    pos : &mut i32,
    buffer:&Vec<TokenVector>,
    token: &mut String,
    vec_error: &mut Vec<String>)-> i32 {
    let mut actual_pos = *pos;
    if read(pos,buffer,token) == 5 { // (
      if e(pos,buffer,token,vec_error) == 1 {
        actual_pos = *pos;
        if read(pos,buffer,token) == 6 { // )
          return 1
        } else{*pos = actual_pos; vec_error.push(error_close_par(buffer, pos)); return 1;}  //CAMBIADO PARA QUE NO DE ERROR POR FALTA DE PARENTESIS
      }
    }
    *pos = actual_pos;
    if read(pos,buffer,token) == 7 { // int
      return 1
    }
    *pos = actual_pos;
    if read(pos,buffer,token) == 8{ // float
      return 1
    }
    *pos = actual_pos;
    if read(pos,buffer,token) == 9{ // id
      return 1
    }
    *pos = actual_pos;
    if read(pos,buffer,token) == 10{ // char
      return 1
    }
    *pos = actual_pos;
    if funcion(pos,buffer,token,vec_error) == 1{
        return 1
    }
    //vec_error.push(format!("Error in line: {}  position: {}\n   Expected expression before {} token",buffer[*pos as usize].ln,buffer[*pos as usize].ps,buffer[*pos as usize].word));
    return 0
  }

  fn t(
    pos : &mut i32,
    buffer:&Vec<TokenVector>,
    token: &mut String,
    vec_error: &mut Vec<String>)->i32{
        if f(pos,buffer,token,vec_error) == 1 {
            return tx(pos,buffer,token,vec_error) 
          }
    return 0
    }

fn e(
    pos : &mut i32,
    buffer:&Vec<TokenVector>,
    token: &mut String,
    vec_error: &mut Vec<String>)->i32{
        if t(pos,buffer,token,vec_error) == 1 {
            if ex(pos,buffer,token,vec_error) == 1 {
              return 1
            }
          }
        return 0
    }

fn read(    
    pos : &mut i32,
    buffer:&Vec<TokenVector>,
    token: &mut String)-> i32{
    //println!("word: {}", buffer[*pos as usize].val);
    if buffer[*pos as usize].val == "+"{
        *pos = *pos + 1;
        *token = "+".to_string();
        return 1
    }else if buffer[*pos as usize].val == "-"{
        *pos = *pos + 1;
        *token = "-".to_string();
        return 2
    }else if buffer[*pos as usize].val == "/"{
        *pos = *pos + 1;
        *token = "/".to_string();
        return 3
    }else  if buffer[*pos as usize].val == "*"{
        *pos = *pos + 1;
        *token = "*".to_string();
        return 4
    } else if buffer[*pos as usize].val == "("{
        *pos = *pos + 1;
        *token = "(".to_string();
        return 5
    } else if buffer[*pos as usize].val == ")"{
        *pos = *pos + 1;
        *token = ")".to_string();
        return 6
    } else if buffer[*pos as usize].val == "int"{
        *pos = *pos + 1;
        *token = "int".to_string();
        return 7
    } else if buffer[*pos as usize].val == "id"{
        *pos = *pos + 1;
        *token = "id".to_string();
        return 8
    } else if buffer[*pos as usize].val == "float"{
        *pos = *pos + 1;
        *token = "float".to_string();
        return 9
    } else if buffer[*pos as usize].val == "char"{
        *pos = *pos + 1;
        *token = "char".to_string();
        return 10
    } else if buffer[*pos as usize].val == ","{
        *pos = *pos + 1;
        *token = ",".to_string();
        return 11
    } else if buffer[*pos as usize].val == ";"{
        *pos = *pos + 1;
        *token = ";".to_string();
        return 12
    } else if buffer[*pos as usize].val == "="{
        *pos = *pos + 1;
        *token = "=".to_string();
        return 13
    } else if buffer[*pos as usize].val == "main"{
        *pos = *pos + 1;
        *token = "main".to_string();
        return 14 
    } else if buffer[*pos as usize].val == "{"{
        *pos = *pos + 1;
        *token = "{".to_string();
        return 15
    } else if buffer[*pos as usize].val == "}"{
        *pos = *pos + 1;
        *token = "}".to_string();
        return 16
    } else if buffer[*pos as usize].val == "if"{
        *pos = *pos +1;
        *token = "if".to_string();
        return 17
    } else if buffer[*pos as usize].val == "else"{
        *pos = *pos +1;
        *token = "else".to_string();
        return 18
    } else if buffer[*pos as usize].val == "pow"{
        *pos = *pos +1;
        *token = "pow".to_string();
        return 19
    } else if buffer[*pos as usize].val == "sqrt"{
        *pos = *pos +1;
        *token = "sqrt".to_string();
        return 20
    } else if buffer[*pos as usize].val == "!"{
        *pos = *pos +1;
        *token = "!".to_string();
        return 21
    }

    *token = "".to_string();
    return 0
}  
