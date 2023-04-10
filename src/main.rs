//NOTE - So for making a command line tool the first thing I need is, for a way to take user input, upon a little reasearch I found that I can use the std library to do that.

fn main(){
    let arguments = std::env::args().collect::<Vec<String>>();
    println!("{:?}", arguments);
}
