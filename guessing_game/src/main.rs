// 입출력 관련 라이브러리
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number !");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop{
        println!("Please input your guess.");
        // 새로운 빈 String 인스턴스를 바인딩한 가변 변수 생성
        let mut guess = String::new();
        // 입력받기
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        // 자료형 일치를 해줘야 한다 trim은 줄바꿈 등을 제거해준다.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed : {guess}");

        match guess.cmp(&secret_number){
        Ordering::Less => println!("Small!"),
        Ordering::Greater => println!("Big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        },
        }
    }

    // {}로만 하면 그 뒤에 ,로 해당하는 값을 넣어주기
    // let x = 5;
    // let y = 10;
    // println!("x = {x} and y + 2 = {}", y + 2);
}
