use fltk::{app::*, button::Button, window::Window, WidgetBase, WidgetExt, WindowExt, GroupExt, output::Output, InputExt };

#[derive(Copy,Clone)]
enum UnaryOps {
    None,
    Equals,
    Clear,
    Sign,
}
#[derive(Copy, Clone, PartialEq)]
enum Ops {
    None,
    Minus,
    Plus,
    Divide,
    Multiply,
}
#[derive(Copy,Clone)]
enum Message {
    UOp(UnaryOps),
    Number(i32),
    Op(Ops),
    Decimal,
}
fn main() {
    // Init app
    let win_w = 385;
    let win_h = 550;
    let app = App::default().with_scheme(Scheme::Gleam);
    let mut wind = Window::default()
        .with_size(win_w,win_h)
        .center_screen()
        .with_label("calculator");

    let mut out = Output::new(20,20, 345, 50, "");
    out.set_text_size(30);
    out.set_value("0");
    let mut v1:f64 = 0.0;
    let mut op_buf = Ops::None;
    let mut v2:f64 = 0.0;

    // Buttons 
    // 0-9 , +,-,*,/ , = , C , . , +/- (prefix)
    // pos, size, label
    let mut but_0 = Button::new(20,480,75,60,"0");
    let mut but_1 = Button::new(20,390,75,60,"1");
    let mut but_2 = Button::new(110,390,75,60,"2");
    let mut but_3 = Button::new(200,390,75,60,"3");
    let mut but_4 = Button::new(20,300,75,60,"4");
    let mut but_5 = Button::new(110,300,75,60,"5");
    let mut but_6 = Button::new(200,300,75,60,"6");
    let mut but_7 = Button::new(20,210,75,60,"7");
    let mut but_8 = Button::new(110,210,75,60,"8");
    let mut but_9 = Button::new(200,210,75,60,"9");



    let but_plus = Button::new(290,480,75,60,"+");
    let but_minus = Button::new(290,390,75,60,"-");
    let but_divide = Button::new(290,300,75,60,"/");
    let but_multiply = Button::new(290,210,75,60,"*");
   
    let mut but_decimal = Button::new(110,480,75,60, ".");
    let but_equals = Button::new(200,480,75,60,"=");
    let but_clear = Button::new(20,170,75,30,"C");
    let but_sign = Button::new(110,170,75,30,"+/-");

    // 
    let but_op_vec = vec![
         but_plus,  but_minus,  but_divide,  but_multiply,
    ];

    let but_unary_op_vec = vec![
        but_equals, but_clear, but_sign,
    ];

    let but_num_vec = vec![
         &mut but_0, &mut but_1, &mut but_2, &mut but_3, &mut but_4, &mut but_5, &mut but_6, &mut but_7, &mut but_8, &mut but_9,
    ];
    // make enum and vectors for buttons or sometig to organize
    wind.end();
    wind.show();

    let (s,r) = channel::<Message>();

    for but in but_num_vec {
        
        but.emit(s, Message::Number(
            but.label().parse().unwrap()
        ));
    }

    for mut but in but_op_vec {
        let op = match but.label().as_str() {
            "-" => Ops::Minus,
            "+" => Ops::Plus,
            "/" => Ops::Divide,
            "*" => Ops::Multiply,
            _ => Ops::None,
        };
        but.emit(s, Message::Op(op));
    }

    for mut but in but_unary_op_vec {
        let op = match but.label().as_str() {
            "=" => UnaryOps::Equals,
            "C" => UnaryOps::Clear,
            "+/-" => UnaryOps::Sign,
            _ => UnaryOps::None,
        };
        but.emit(s,Message::UOp(op));
    }

    but_decimal.emit(s, Message::Decimal);
    // Main loop
    
    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Message::Number(num) => {
                    let mut tmp = out.value();
                    if tmp == "0" {
                        tmp.clear();
                    }
                    if tmp == "-0" {
                        tmp = String::from("-");
                    }
                    tmp.push_str(&num.to_string());
                    
                    out.set_value(&tmp);
                    if op_buf == Ops::None {
                        v1 = out.value().parse::<f64>().unwrap();
                    } else {
                        v2 = out.value().parse::<f64>().unwrap();
                    }
                },
                Message::Decimal => {
                    let mut tmp = out.value();
                    if !tmp.contains(".") {
                        tmp.push_str(".");
                    }
                    out.set_value(&tmp);
                },
                Message::Op(op) => {
                    op_buf = op;
                    out.set_value("0");
                }
                Message::UOp(uop) => {
                    match uop {
                        UnaryOps::Equals => {
                            let res = match op_buf {
                                Ops::Plus => v1 + v2,
                                Ops::Minus => v1 - v2,
                                Ops::Divide => v1 / v2,
                                Ops::Multiply => v1 * v2,
                                _ => v1,
                            };
                            op_buf = Ops::None;
                            out.set_value(&res.to_string());
                            v1 = res;
                        },
                        UnaryOps::Clear => {
                            v1 = 0.0;
                            v2 = 0.0;
                            op_buf = Ops::None;
                            out.set_value("0");
                        },
                        UnaryOps::Sign => {
                            let mut flipped = out.value().parse::<f64>().unwrap();
                            flipped = flipped * -1.0;
                            out.set_value(&flipped.to_string());
                            if op_buf == Ops::None {
                                v1 = v1 * -1.0;
                            } else {
                                v2 = v2 * -1.0;
                            }
                        },
                        _ => (),
                    }
                }

            }
        }
    }

    app.run().unwrap();
}
