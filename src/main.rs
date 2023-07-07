use std::{env, iter::Peekable};
#[derive(PartialEq,Clone, Copy)]
enum TokenKind {
	TK_RESERVED,
	TK_NUM,
	TK_EOF,
}

#[derive(Clone)]
struct Token{
	kind: Option<TokenKind>,
	val: Option<usize>,
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

	fn expect_number(&self)->usize{
		if self.kind.unwrap() != TokenKind::TK_NUM{
			panic!("数ではありません");
		}
		self.val.unwrap()
	}

	fn at_eof(&self)->bool{
		self.kind == Some(TokenKind::TK_EOF)
	}
}
fn read_num(c:&char, iter:&mut Peekable<impl Iterator<Item = char>>)->usize{
	let mut join_str = String::new();
	join_str.push(*c);
	while !iter.peek().unwrap().to_digit(10).is_none() {
		join_str.push(*iter.peek().unwrap());
		iter.next();
	}
	join_str.parse::<usize>().unwrap()
}
fn tokenize(p: &str)->Vec<&Token>{
	let mut cur:Vec<&Token> = Vec::new();
	let mut chars = p.chars().into_iter().peekable();
	let mut c = chars.next();
	while !c.is_none() {
		if c.unwrap().is_whitespace(){
			continue;
		}

		if c.unwrap() == '+' || c.unwrap() == '-' {
			cur.push(&Token::new(TokenKind::TK_RESERVED, &c.unwrap()));
			continue;
		}

		if !c.unwrap().to_digit(10).is_none(){
			let tok = &Token::new(TokenKind::TK_NUM, &c.unwrap());
			tok.val = Some(read_num(&c.unwrap(), &mut chars));
			cur.push(tok);
			continue;
		}

		panic!("トークナイズできません。");
	}
	cur.push(&Token::new(TokenKind::TK_EOF, &'\0'));
	cur
}

fn main(){
	let args: Vec<String> = env::args().collect();
	if args.len() != 2{
		panic!("引数の個数が正しくありません");
	}

	let token = tokenize(args[1].as_str());
	let mut i = 0;

  // アセンブリの前半部分を出力
  println!(".intel_syntax noprefix");
  println!(".globl main");
  println!("main:");

  // 式の最初は数でなければならないので、それをチェックして
  // 最初のmov命令を出力
  println!("  mov rax, {}", token[i].expect_number());

	while 
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