fn main(){
    let var1 = "test1";
let json = r#"{"type": "type1", "type2": var1}"#;
println!("{}", json) // => {"type2": "type1", "type2": var1}


}