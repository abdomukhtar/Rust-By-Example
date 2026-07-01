#[derive(Debug)]
enum Message {
    // 1- Variant بدون بيانات
    Empty,

    // 2- Tuple Variant
    Text(String),

    // 3- Struct Variant
    Number { value: i32, positive: bool },
}

fn main() {
    // -------------------------
    // Tuple Variant
    // -------------------------
    let msg1 = Message::Text(String::from("Ali"));

    match msg1 {
        Message::Text(text) => {
            println!("Text: {}", text);
        }

        Message::Number { value, positive } => {
            println!("Number: {} ({})", value, positive);
        }

        Message::Empty => {
            println!("Empty");
        }
    }

    // -------------------------
    // Struct Variant
    // -------------------------
    let msg2 = Message::Number {
        value: 100,
        positive: true,
    };

    match msg2 {
        Message::Text(text) => {
            println!("Text: {}", text);
        }

        Message::Number { value, positive } => {
            println!("Number: {}", value);
            println!("Positive: {}", positive);
        }

        Message::Empty => {
            println!("Empty");
        }
    }

    // -------------------------
    // Empty Variant
    // -------------------------
    let msg3 = Message::Empty;

    match msg3 {
        Message::Text(text) => {
            println!("Text: {}", text);
        }

        Message::Number { value, positive } => {
            println!("Number: {} ({})", value, positive);
        }

        Message::Empty => {
            println!("Empty");
        }
    }
}

//=========================================================================
//------------------------------------------------------------------------|
//=========================================================================
enum Signal {
    // هنا نكتب نوع البيانات الذي سيحمله الـ Variant
    Variant(String),

    Variant1,
    Variant2,
}

fn main() {
    // هنا ننشئ متغيرًا ونضع بداخله قيمة من نوع Signal
    let anything = Signal::Variant(String::from("Go"));

    match anything {
        // هنا نكتب متغيرًا جديدًا سيحمل القيمة الموجودة داخل Variant
        Signal::Variant(variable) => println!("{}", variable),

        Signal::Variant1 => println!("It's a 2nd"),

        Signal::Variant2 => println!("It's a 3rd"),
    }
}
