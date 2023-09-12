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

fn main() {
    print!("Ввод размера: ");
    let arrlen = read_digit!();
    println!("\nРазмер массива: {arrlen}");
    let arr: Vec<i32> = loop {
        let safe_arr: Vec<i32> = read_digits!();    
        if safe_arr.len() as i32 == arrlen { break safe_arr; }
        println!("Ошибка с кол-вом элементов массива. Имеем: {}. Нужно: {arrlen}", safe_arr.len());
    };
    
    for i in 0..arr.len() {
        println!("arr[{i}] = {}", arr[i]);
    }
    
    let favindex: Vec<i32> = loop {
        print!("Ввод индексов: ");
        let ind: Vec<i32> = read_digits!();
        for i in &ind {
            if i >= &arrlen && i < &0 { 
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
    }
    
    let mut opers = std::collections::HashMap::new();
    opers.insert("сложение", '+');
    opers.insert("вычитание", '-');
    opers.insert("умножение", '*');
    opers.insert("деление", '/');
    println!("Ввод операций (сложение, вычитание, умножение, деление): ");

    let favoper = loop {
        let mut safe_op: Vec<String> = read_calc_op!();
        let mut safe_op_err: bool = false;       // false значит ошибок нет
        for val in &mut safe_op {
            if opers.contains_key::<str>(&val) {
                for (k, v) in opers.iter() {
                    if k == val { *val = v.to_string(); }
                }
                continue; 
            }
            safe_op_err = true;
        }
        if safe_op_err == false && safe_op.len() == (favindex.len() - 1) { break safe_op; }
        println!("Ошибка с операциями. Имеем: {}. Нужно: {}", safe_op.len(), favindex.len()-1);
    };
   println!("{:?}", favoper);
}
