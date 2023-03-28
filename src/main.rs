use std::io;

fn main() {
    println!("Угадайте число");
    println!("Пожалуйста, введите свою догадку.");

    // let - создание переменной
    // mut - изменяемая переменная
    let mut guess: String = String::new();

    // &mut guess - изменяемая ссылка
    io::stdin().read_line(&mut guess)
        // io::Result.expect - Ok/Err elements
        .expect("Не получилось прочитать строку");

    println!("Вы загадали: {}", guess);
}
