#[derive(Debug)] // 상태를 바로 확인
enum IpAddrKind {
    // 각 배리언트에 직접 데이터를 붙일 수 있다.
    // 다른 타입과 다른 양의 연관된 데이터를 가질 수 있다.
    V4,
    V6,
}

enum Option<T> {
    None, 
    Some(T)
}

fn plus_one(x : Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

struct IpAddr {
    kind : IpAddrKind,
    address : String,
}

fn value_in_cents(coin : Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    // 열거형은 TS하면서 많이 하던 것이므로 실습만 하면서 감을 익히기
    // Option 타입은 값이 있거나 없을 수 있는 타입을 만들 수 있다. -> 만약 널이라면 널인 경우를 처리해야 한다.
    // 열거형 값에 데이터가 있는 경우, 처리해야 하는 경우의 수에 따라 match나 if let을 사용하여 해당 값을 추출하여 사용 가능
    // match -> 제어 흐름 연산자
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("Hello, world!");
    // if let을 이용하면 match와 동일하게 작동
    let mut count = 0
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    if let Coin::Quarter(state) = coin {
        println!("State quater from {:?}!", state);
    }else{
        count += 1;
    }
}
