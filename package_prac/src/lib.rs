mod front_of_house {
    // 여긴 비공개 영역이라 에러가 날 것임, 공개적 영역으로 바꾸려면 상위를 포함하여 하위ㅏ지 pub 키워드 추가하기
    mod hosting {
        fn add_to_waitlist(){}
    }
}

// 별개의 파일로 모듈 분리하는 방법


pub fn eat_at_restaurant(){
    //절대 경로
    crate::front_of_house::hosing::add_to_waitlist();

    //상대 경로
    front_of_house::hosing::add_to_waitlist();

    // super로 시작하는 상대 경로를 사용해 함수를 호출할 수도 있다.
}

mod back_of_house {
    pub struct Breakfast {
        pub toast : String,
        seasonal_fruit : String,
    }

    impl Breakfast {
        pub fn summer(toast : &str) -> Breakfast {
            Breakfast {
                toast : String::from(toast),
                seasonal_fruit : String::from("peaches"),
            }
        }
    }
}

pub gn eat_at_restaurant( ){
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    //얘는 컴파일 되지 않는다 -> private이기 때문에 수정이나 조회가 허용되지 않는다.
    // meal.seasonal_fruit = String::from("blueberries");
}