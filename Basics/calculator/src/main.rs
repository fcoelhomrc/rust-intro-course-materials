/// Desafio da calculadora
/// Pode ser implementado com recurso à analise de apenas
/// uma string ou com cada elemento separado na sua propria string.
/// Podem ver exemplos de como vai ser utilizada a função nos testes disponíveis.
///
/// Devem apenas implementar uma das funções.
///
/// Podem comentar a função que não vão implementar para não haver problemas de compilação, incluindo os testes
/// para a mesma.
use ::std::io;

fn main() {
    // todo!("Implementar a leitura do stdin")

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: &str = input.trim();

    // Convert to list of strings
    let list: Vec<&str> = input.split_whitespace().collect();

    let result : i32 = calculator_str_list(&list);

    println!("{} = {}", input, result);

}

// fn calculator_str(string: &str) -> i32 {
//     todo!("Implementar a calculadora que de uma string calcule o resultado")
// }

fn calculator_str_list(string: &[&str]) -> i32 {
    // todo!("Implementar a calculadora que de uma string ou de uma lista de strings calcule o resultado")
    let mut result: i32 = 0;

    let a : i32 = string[0].parse().unwrap();  // parse allows to typecast from &str to i32
    let operation : &str = string[1];
    let b : i32 = string[2].parse().unwrap();

    if operation == "+" {
        result = a + b;
    } else if operation == "-" {
        result = a - b;
    } else if operation == "*" {
        result = a * b;
    } else if operation == "/" {
        result = a / b;
    } else {
        println!("Unknown operation: {}", operation);
    }

    result
}

#[cfg(test)]
pub mod calculator_test {

    // #[test]
    // fn test_calculator_str() {
    //     assert_eq!(super::calculator_str("1 + 1"), 2);
    //     assert_eq!(super::calculator_str("2 * 2"), 4);
    //     assert_eq!(super::calculator_str("2 / 2"), 1);
    //     assert_eq!(super::calculator_str("2 - 2"), 0);
    // }

    #[test]
    fn test_calculator_str_list() {
        assert_eq!(super::calculator_str_list(&vec!["2", "*", "3"]), 6);
        assert_eq!(super::calculator_str_list(&vec!["2", "+", "3"]), 5);
        assert_eq!(super::calculator_str_list(&vec!["3", "-", "2"]), 1);
        assert_eq!(super::calculator_str_list(&vec!["6", "/", "3"]), 2);
    }
}
