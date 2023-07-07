use std::env;
#[derive(PartialEq,Clone, Copy)]
enum TokenKind {
	TK_RESERVED,
	TK_NUM,
}

#[derive(Clone)]
struct Token{
	kind: Option<TokenKind>,
	val: Option<u32>,
	str: Option<char>,
}

impl Token {
	fn new(kind: TokenKind, str: &char)->Self{
		Token { kind: Some(kind), val: None, str: Some(*str) }
	}

	fn consume(&self, op:char)->bool{
		self.kind.unwrap() != TokenKind::TK_RESERVED || self.str.unwrap() != op
	}

	fn expect(&self, op:char){
		if self.kind.unwrap() != TokenKind::TK_RESERVED || self.str.unwrap() != op {
			panic!("'{}'ではありません", op);
		}
	}

	fn expect_number(&self)->u32{
		if self.kind.unwrap() != TokenKind::TK_NUM{
			panic!("数ではありません");
		}
		self.val.unwrap()
	}
}
fn tokenize(p: &str)->Vec<Token>{
	let mut cur:Vec<Token> = Vec::new();
	for c in p.chars(){
		if c.is_whitespace() {
			continue;
		}

		if c == '+' || c == '-' {
			cur.push(Token::new(TokenKind::TK_RESERVED, &c));
			continue;
		}

		if !c.to_digit(10).is_none(){
			let mut num = Token::new(TokenKind::TK_NUM, &c);
			num.val = c.to_digit(10);
			cur.push(num);
			continue;
		}

		panic!("トークナイズできません。");
	}
	cur
}

fn main(){
	let args: Vec<String> = env::args().collect();
	if args.len() != 2{
		panic!("引数の個数が正しくありません");
	}

	let token = tokenize(args[1].as_str());

  // アセンブリの前半部分を出力
  println!(".intel_syntax noprefix");
  println!(".globl main");
  println!("main:");

  // 式の最初は数でなければならないので、それをチェックして
  // 最初のmov命令を出力
  println!("  mov rax, {}", token[0].expect_number());
	for t in 1..token.len(){
		let cur = &token[t];
		if cur.consume('+'){
			println!("  add rax, {}", cur.expect_number());
			continue;
		}

		cur.expect('-');
		println!("  sub rax, {}", cur.expect_number());
	}

	println!("  ret");
}