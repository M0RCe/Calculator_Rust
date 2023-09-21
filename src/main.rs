/* 
   Calculator Program
   Copyright (c) 2023 M0RCe
*/
macro_rules! read_digit {
    () => {
        {
            let mut str = String::new();
            std::io::stdin().read_line(&mut str).expect("Не удалось прочесть строку");
            let input: i32 = str.trim().parse().expect("Ожидался целочисленный тип");
            input
        }
    };
}
macro_rules! read_digits {
    () => {
        {
            let mut str = String::new();
            let mut arr: Vec<i32> = Vec::new();
            std::io::stdin().read_line(&mut str).expect("Не удалось прочесть строку");
            let split_str = str.trim().split_whitespace();
            for (_, val) in split_str.enumerate() {
                let int_val: i32 = val.parse().expect("Ожидался целочисленный тип");
                arr.push(int_val);
            }
            arr
        }
    };
}
macro_rules! read_calc_op {
    () => {
        {
            let mut str = String::new();
            let mut arr: Vec<String> = Vec::new();
            std::io::stdin().read_line(&mut str).expect("Не удалось прочесть строку");
            for val in str.trim().split_whitespace() {
                arr.push(val.to_string());
            }
            arr
        }
    };
}
macro_rules! read_digit_exit {
    () => {
        {
            let mut str = String::new();
            std::io::stdin().read_line(&mut str).expect("Не удалось прочесть строку");
            str
        }
    };
}
macro_rules! read_digits_float {
    () => {
        {
            let mut str = String::new();
            let mut arr: Vec<f32> = Vec::new();
            std::io::stdin().read_line(&mut str).expect("Не удалось прочесть строку");
            let split_str = str.trim().split_whitespace();
            for (_, val) in split_str.enumerate() {
                let int_val: f32 = val.parse().expect("Ожидался тип с плавающей точкой");
                arr.push(int_val);
            }
            arr
        }
    };
}

fn enter_array() -> Vec<f32> {
    let arrlen = read_digit!();
    println!("\nРазмер массива: {arrlen}");
    let arr: Vec<f32> = loop {
        let safe_arr: Vec<f32> = read_digits_float!();    
        if safe_arr.len() as i32 == arrlen && safe_arr.len() > 1 { break safe_arr; }
        println!("Ошибка с кол-вом элементов массива. Имеем: {}. Нужно: {arrlen}", safe_arr.len());
        if safe_arr.len() < 2 { panic!("Размер массива обязан иметь минимум 2 элемента"); }
    };
    
    for i in 0..arr.len() {
        println!("arr[{i}] = {}", arr[i]);
    }
    arr
}

fn choose_elem(arr: &Vec<f32>) -> Vec<f32> {
    let mut newarr: Vec<f32> = Vec::new();
    let favindex: Vec<i32> = loop {
        let ind: Vec<i32> = read_digits!();
        for i in &ind {
            if i >= &(arr.len() as i32) && i < &0 { 
                println!("Ошибка с индексами");
                continue;
            }
        }
        if ind.len() > 1 && ind.len() <= arr.len() { break ind; }
        println!("\nОшибка с индексами");
    };
    
    println!("Выбраны элементы: ");
    for i in 0..favindex.len() {
        println!("arr[{}] = {}", favindex[i], arr[favindex[i] as usize]);
        newarr.push(arr[favindex[i] as usize]);
    }
    newarr
}

fn choose_op(arrlen: usize) -> Vec<String> {
    let mut opers = std::collections::HashMap::new();
    opers.insert("сложение", '+');
    opers.insert("вычитание", '-');
    opers.insert("умножение", '*');
    opers.insert("деление", '/');

    let favoper = loop {
        let mut safe_op: Vec<String> = read_calc_op!();
        let mut safe_op_err: bool = false;       // false значит ошибок нет
        for val in &mut safe_op {
            if opers.contains_key::<str>(&val) {
                for (k, v) in opers.iter() {
                    if k == val {*val = v.to_string();}
                }
                continue; 
            }
            safe_op_err = true;
        }
        if safe_op_err == false && safe_op.len() == (arrlen - 1) { break safe_op; }
        println!("Ошибка с операциями. Имеем: {}. Нужно: {}", safe_op.len(), arrlen - 1);
    };
    favoper
}
// 5*3-9/3*2-5 -> 15-6-5 -> 4
#[allow(dead_code)]
fn calc_fin(arr: &mut Vec<f32>, op: &mut Vec<String>) -> f32 {
    let mut answer: f32 = 0.0;
    let mut question = String::new();
    let mut op_seq: Vec<usize> = Vec::new();
    
    for i in 0..arr.len() {
        question.push_str(&arr[i].to_string());
        if arr.len() - 1 == i { break; }
        question.push_str(&op[i].to_string());
    }
    
    println!("Выражение: {question}");
    
    for i in 0..op.len() {
        if op[i] == "*" || op[i] == "/" { op_seq.push(i) }
    }
    for val in op_seq.iter() {
        let digit1: f32 = arr[*val as usize];
        let digit2: f32 = arr[val + 1];
        let new_digit: f32;
        if digit2 == 0.0 && op[*val] == "/" { panic!("Деление на ноль"); }
        match &op[*val] as &str {
            "*" => { new_digit = digit1 * digit2 },
            "/" => { new_digit = digit1 / digit2 },
            _ => panic!("Неверная запись знака"),
        }

        arr[*val as usize] = new_digit;
        arr[val + 1] = new_digit;
        
        answer = arr[val + 1];
    }
    if !op_seq.is_empty() {
        op_seq.reverse();
        for val in &op_seq {
            arr.remove(*val as usize);
            op.remove(*val as usize);
        }
        op_seq.clear();
    }
    for i in 0..op.len() {
        if op[i] == "+" || op[i] == "-" { op_seq.push(i) }
    }
    for val in op_seq.iter() {
        let digit1: f32 = arr[*val as usize];
        let digit2: f32 = arr[val + 1];
        let new_digit: f32;
        match &op[*val] as &str {
            "+" => { new_digit = digit1 + digit2 },
            "-" => { new_digit = digit1 - digit2 },
            _ => panic!("Неверная запись знака"),
        }

        arr[*val as usize] = new_digit;
        arr[val + 1] = new_digit;
        
        answer = arr[val + 1];
    }
    answer
}

#[test]
fn test() {
    let mut arr: Vec<f32> = [10.0, 5.0, 3.0].to_vec();
    let mut op: Vec<String> = Vec::new();
    op.push("/".to_string());
    op.push("+".to_string());
    assert_eq!(calc_fin(&mut arr, &mut op), 5.0);
}

#[test]
#[should_panic(expected = "Неверная запись знака")]
fn test2() {
    let mut arr: Vec<f32> = [10.0, 0.0].to_vec();
    let mut op: Vec<String> = Vec::new();
    op.push("деление".to_string());
    calc_fin(&mut arr, &mut op);
}

#[test]
#[should_panic(expected = "Деление на ноль")]
fn test3() {
    let mut arr: Vec<f32> = [10.0, 0.0].to_vec();
    let mut op: Vec<String> = Vec::new();
    op.push("/".to_string());
    calc_fin(&mut arr, &mut op);
}

#[test]
fn test4() {
    let mut arr: Vec<f32> = [500.0, 10.0, 66.0, 32.0].to_vec();
    let mut op: Vec<String> = Vec::new();
    op.push("+".to_string());
    op.push("-".to_string());
	 op.push("*".to_string());
    assert_ne!(calc_fin(&mut arr, &mut op), 14208.0);
	// lol https://stackoverflow.com/questions/73350166/how-do-i-convert-and-calculate-a-string-expression-into-arithmetic-expression-wi
}

fn main() {
    loop {
        print!("Ввод размера: ");
        let mut arr: Vec<f32> = enter_array();
        print!("Ввод индексов: ");
        arr = choose_elem(&arr);
        print!("Ввод операций (сложение, вычитание, умножение, деление): ");
        let mut favoper = choose_op(arr.len());
        let answer = calc_fin(&mut arr, &mut favoper);
        println!("Ответ: {answer}");
        println!("1 - выйти, иначе - выбрать другие элементы и операции");
        if read_digit_exit!().trim() == "1" { break; }
    }
}
