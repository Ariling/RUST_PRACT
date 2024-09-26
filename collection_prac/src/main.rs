use std::collections::HashMap;
fn main() {
    // 일단 배열, 튜플과 컬렌션의 차이점
    // 컬렉션들이 가리키고 있는 데이터들은 힙에 저장, 이는 데이터 양이 컴파일 타임에 결정되지 않아도 된다
    // 실행중에 유동적으로 크기 조절 가능 (나머지 애들은 그럴 수 없다)
    // 대표적인 컬렉션 Vector, String, hash map(map)
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    // 요소 읽는 방법 2가지(인덱싱 , get 메서드)
    let third : &i32 = &v[2];
    println!("The third element is {third}");

    let third : Option<&i32> = v.get(2);

    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    // 하지만 이런 경우 에러가 난다.
    // let mut b = vec![1,2,3,4,5];
    // let first = &b[0];
    // b.push(6);
    // println!("The first element is : {first}");
    // 벡터의 동작 방법 : 만약 메모리 공간이 부족하다면 다른 넉넉한 곳에 메모리 재할당 및 기존 요소를 새로운 곳에 복사
    // 이 경우, 기존 요소의 참조자는 해제된 메모리를 가리키게 되기 때문에 이러한 상황을 방지하고자 에러를 일으킴
    // 조건문에서 값을 변경하여 반영하고 싶을 때에는 역참조 연산자로 i 값을 얻어야 한다.
    for i in &mut v {
        *i += 50;
        println!("{i}");
    }
    // 백터 내에 다른 타입의 값들을 저장하려면 열거형을 정의하여 사용하면 된다.
    // 하지만 이런 경우 모든 타입 집합을 알아야하는데 이를 대체하기 위해 트레이트 객체가 있다고 한다.
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    // 문자열 메서드만 알아두기
    let mut s = String::from("foo");
    s.push_str("bar");
    let s2 = String::from("test");
    let s3 = s + &s2; // 이 경우 s는 이동되어 더 이상 s는 사용할 수 없게 된다.
    println!("s3 is {s3}");
    // format!이라는 매크로는 println!처럼 작동하지만 결과가 담긴 String을 반환
    // !!Rust에서는 문자열 인덱싱을 지원하지 않는다!! !! 슬라이싱도 바이트를 기준으로 한다 !!

    // ---------------------------------------------------------------------
    // 해시 맵 부분
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),20);

    for (key, value) in &scores {
        println!("{key} : {value}");
    }
    // 덮어쓰는 건 그냥 insert를 또 해주면 된다.
    // 키가 없을 때만 추가하는 경우 (entry활용)
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Pink")).or_insert(50);
    println!("{:?}", scores);
    // 서비스 거부 공격에 저항 기능을 제공할 수 있는 SipHash라고 불리는 해시 함수를 사용
}
