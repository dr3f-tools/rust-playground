use cli::parse_cli;
use math::math::add;

fn main() {
    let args = parse_cli();
    println!("{} + {} = {}", args.a, args.b, add(args.a, args.b));
}
