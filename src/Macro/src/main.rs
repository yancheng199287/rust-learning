#[macro_export]
macro_rules! call_js_function {
    ($a:expr,$source:expr,$name:expr, $( $x:expr ),* ) => {
        {

                // qqqqhhhhqejjhjh
                println!("{}", concat!(   $($x),*)    );


               // "qqqq"   "hhhh"  "qejjhjh"
                println!("{:?}   {:?}  {:?}  ",   $($x),*);
        }
    };
}

fn main() {
    call_js_function!("Hello, world!","sss","ppp","qqqq","hhhh","qejjhjh");
}
