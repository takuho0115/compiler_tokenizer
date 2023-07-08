use std::{env, iter::{Peekable, Enumerate}, str::Chars, fmt::Error};
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
  pos: Option<usize>,
}

impl Token {
	fn new(kind: TokenKind, str: &char, pos: &usize)->Self{
		Token { kind: Some(kind), val: None, str: Some(*str), pos: Some(*pos) }
	}

	fn consume(&self, op:char)->bool{
		!(self.kind.unwrap() != TokenKind::TK_RESERVED || self.str.unwrap() != op)
	}

	fn expect(&self, op:char){
		if !self.consume(op) {
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
fn read_num(c:&char, iter:&mut Peekable<Enumerate<Chars>>)->usize{
	let mut join_str = String::new();
	join_str.push(*c);
	while !iter.peek().is_none() {
		let (_i, p) = iter.peek().unwrap();
		if p.to_digit(10).is_none(){
			break;
		}
		join_str.push(*p);
		iter.next();
	}
	join_str.parse::<usize>().unwrap()
}

fn at_error(s: &str, i: usize){
	println!("{}", s);
	print!("{}^ ", " ".repeat(i));
}

fn tokenize(p: &str)->Vec<Token>{
	let mut cur:Vec<Token> = Vec::new();
	let mut chars = p.chars().into_iter().enumerate().peekable();
	let mut current = chars.next();
	while !current.is_none() {
		let (i, c) = current.unwrap();
		if c.is_whitespace(){
			current = chars.next();
			continue;
		}

		if c == '+' || c == '-' {
			cur.push(Token::new(TokenKind::TK_RESERVED, &c, &i));
			current = chars.next();
			continue;
		}

		if !c.to_digit(10).is_none(){
			let mut tok = Token::new(TokenKind::TK_NUM, &c, &i);
			tok.val = Some(read_num(&c, &mut chars));
			cur.push(tok);
			current = chars.next();
			continue;
		}
		at_error(p, i);
		println!("トークナイズできません。");
		panic!("トークナイズできません。");
		
	}
	cur.push(Token::new(TokenKind::TK_EOF, &'\0', &chars.count()));
	cur
}

fn main(){
	let args: Vec<String> = env::args().collect();
	if args.len() != 2{
		panic!("引数の個数が正しくありません");
	}
	let token = tokenize(args[1].as_str());
	let mut token_iter = token.iter().peekable();

  // アセンブリの前半部分を出力
  println!(".intel_syntax noprefix");
  println!(".globl main");
  println!("main:");

  // 式の最初は数でなければならないので、それをチェックして
  // 最初のmov命令を出力
  println!("  mov rax, {}", token_iter.next().unwrap().expect_number());

	while !token_iter.peek().unwrap().at_eof() {
		if token_iter.peek().unwrap().consume('+') {
			token_iter.next();
			println!("  add rax, {}", token_iter.next().unwrap().expect_number());
			continue;
		}

		token_iter.next().unwrap().expect('-');
		println!("  sub rax, {}", token_iter.next().unwrap().expect_number());
	}

	println!("  ret");
}