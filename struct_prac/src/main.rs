#[derive(Debug)]
struct Rectangle {
    width : u32,
    height : u32,
}

// 메서드 정의하기
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other : &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    // 구조체는 여러 값을 묶고 이름을 지어서 의미 있는 묶음을 정의
    // 메서드의 정의 방법을 주로 봐보자
    // 기본 인스턴스 생성 및 변경은 다른 언어들이랑 비슷한 느낌이 있다.
    // 구조체 데이터의 소유권
    /*라이프타임을 활용하는 것이 좋다! 참조자를 이용하는 것도 좋은 방법이지만
    라이프타임을 사용하면 구조체가 존재하는 동안에 구조체 내의 참조자가 가리키는 데이터의 유효함을 보장받기 때문
    이건 추후에 다룰 예정이다!*/
    // 활용 예시
    let rect1 = Rectangle{
        width : 30,
        height : 50,
    };
    let rect2 = Rectangle{
        width : 10,
        height : 40,
    };
    let rect3 = Rectangle{
        width : 30,
        height : 60,
    };
    println!(
        "The area of the rectangle is {} square pixels.",rect1.area());
    // 만약 구조체 정보를 데려오고 싶다면 {} 대신 {:?}를 사용해보기 -> 다만 이걸 이용할 땐 첫번째 줄을 활용하기
    println!("rect1 is {:?}", rect1);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
// 소유권을 가져오지 않고 참조를 이용하기
fn area(rectangle : &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
