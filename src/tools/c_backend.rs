//! Standalone scalar C code generator; does not invoke Micro-C parsing or codegen.
use alloc::{boxed::Box, format, string::String, vec::Vec};
use hashbrown::HashMap;

#[derive(Clone)]
enum E { N(i64), V(String), Call(String, Vec<E>), Bin(Box<E>, String, Box<E>), Neg(Box<E>), Not(Box<E>), BitNot(Box<E>), Ternary(Box<E>,Box<E>,Box<E>) }
#[derive(Clone)]
enum S { Let(String,E), Set(String,E), Expr(E), Ret(Option<E>), If(E,Vec<S>,Vec<S>), Loop(Vec<S>), Break, Continue }
struct F { name:String, args:Vec<String>, body:Vec<S> }
struct P { t:Vec<String>, i:usize }
impl P {
 fn peek(&self)->Option<&str>{self.t.get(self.i).map(String::as_str)}
 fn pop(&mut self)->String{let x=self.t.get(self.i).cloned().unwrap_or_default();self.i+=1;x}
 fn eat(&mut self,s:&str)->bool{if self.peek()==Some(s){self.i+=1;true}else{false}}
 fn need(&mut self,s:&str)->Result<(), &'static str>{if self.eat(s){Ok(())}else{Err("invalid normalized C syntax")}}
 fn names(&mut self,end:&str)->Result<Vec<String>, &'static str>{let mut v=Vec::new();while self.peek()!=Some(end){let n=self.pop();if n.is_empty(){return Err("unterminated C parameter list")}v.push(n);if !self.eat(",")&&self.peek()!=Some(end){return Err("invalid C parameter list")}}Ok(v)}
 fn block(&mut self)->Result<Vec<S>, &'static str>{self.need("{")?;let mut v=Vec::new();while self.peek()!=Some("}"){if self.peek().is_none(){return Err("unterminated C body")}v.push(self.stmt()?)}self.need("}")?;Ok(v)}
 fn block_or_stmt(&mut self)->Result<Vec<S>, &'static str>{if self.peek()==Some("{"){self.block()}else{Ok(alloc::vec![self.stmt()?])}}
 fn stmt(&mut self)->Result<S, &'static str>{
  if self.eat("let"){let n=self.pop();self.need("=")?;let e=self.expr(0)?;self.need(";")?;return Ok(S::Let(n,e))}
  if self.eat("return"){let e=if self.peek()==Some(";"){None}else{Some(self.expr(0)?)};self.need(";")?;return Ok(S::Ret(e))}
  if self.eat("break"){self.need(";")?;return Ok(S::Break)}
  if self.eat("continue"){self.need(";")?;return Ok(S::Continue)}
  if self.eat("if"){self.need("(")?;let e=self.expr(0)?;self.need(")")?;let a=self.block_or_stmt()?;let b=if self.eat("else"){self.block_or_stmt()?}else{Vec::new()};return Ok(S::If(e,a,b))}
  if self.eat("loop"){return Ok(S::Loop(self.block()?))}
  if self.i+1<self.t.len()&&matches!(self.t[self.i+1].as_str(),"="|"+="|"-="|"*="|"/="|"%="|"&="|"|="|"^="|"<<="|">>="){let n=self.pop();let op=self.pop();let rhs=self.expr(0)?;self.need(";")?;let e=if op=="="{rhs}else{E::Bin(Box::new(E::V(n.clone())),String::from(&op[..op.len()-1]),Box::new(rhs))};return Ok(S::Set(n,e))}
  let e=self.expr(0)?;self.need(";")?;Ok(S::Expr(e))
 }
 fn expr(&mut self,min:u8)->Result<E,&'static str>{
  let t=self.pop();
  let mut a=if t=="("{if super::c_compiler::is_type(self.peek().unwrap_or("")){while self.peek().map(super::c_compiler::is_type).unwrap_or(false)||self.peek()==Some("*"){self.i+=1}self.need(")")?;self.expr(7)?}else{let e=self.expr(0)?;self.need(")")?;e}}
  else if t=="-"{E::Neg(Box::new(self.expr(7)?))}
  else if t=="!"{E::Not(Box::new(self.expr(7)?))}
  else if t=="~"{E::BitNot(Box::new(self.expr(7)?))}
  else if t=="+"{self.expr(7)?}
  else if let Some(n)=number(&t){E::N(n)}
  else if self.eat("("){let mut args=Vec::new();while self.peek()!=Some(")"){args.push(self.expr(0)?);if !self.eat(",")&&self.peek()!=Some(")"){return Err("invalid call args")}}self.need(")")?;E::Call(t,args)}
  else if ident(&t){E::V(t)}else{return Err("unsupported C expression")};
 loop{let Some(op)=self.peek()else{break};let p=prec(op);if p==0||p<min{break}if op=="?"{self.pop();let yes=self.expr(0)?;self.need(":")?;let no=self.expr(p)?;a=E::Ternary(Box::new(a),Box::new(yes),Box::new(no));continue}let op=self.pop();let b=self.expr(p+1)?;a=E::Bin(Box::new(a),op,Box::new(b))}Ok(a)
 }
 fn program(mut self)->Result<(Vec<F>,Vec<String>),&'static str>{
  let mut fs=Vec::new();let mut ex=Vec::new();while self.peek().is_some(){let external=self.eat("extern");if !external{self.eat("export");}self.need("fn")?;let name=self.pop();self.need("(")?;let args=self.names(")")?;self.need(")")?;if external{self.need(";")?;ex.push(name)}else{fs.push(F{name,args,body:self.block()?})}}Ok((fs,ex))
 }
}
fn number(s:&str)->Option<i64>{let s=s.trim_end_matches(['u','U','l','L']);if let Some(h)=s.strip_prefix("0x").or_else(||s.strip_prefix("0X")){i64::from_str_radix(h,16).ok()}else if let Some(b)=s.strip_prefix("0b").or_else(||s.strip_prefix("0B")){i64::from_str_radix(b,2).ok()}else if s.len()>1&&s.starts_with('0'){i64::from_str_radix(&s[1..],8).ok()}else{s.parse().ok()}}
fn ident(s:&str)->bool{let mut c=s.chars();matches!(c.next(),Some(x)if x.is_ascii_alphabetic()||x=='_')&&c.all(|x|x.is_ascii_alphanumeric()||x=='_')}
fn prec(s:&str)->u8{match s{"?"=>2,"||"=>1,"&&"=>2,"|"=>3,"^"=>4,"&"=>5,"=="|"!="=>6,"<"|"<="|">"|">="=>7,"<<"|">>"=>8,"+"|"-"=>9,"*"|"/"|"%"=>10,_=>0}}
fn collect(s:&[S],v:&mut Vec<String>){for x in s{match x{S::Let(n,_)=>if !v.contains(n){v.push(n.clone())},S::If(_,a,b)=>{collect(a,v);collect(b,v)},S::Loop(a)=>collect(a,v),_=>()}}}
struct G{a:String,slots:HashMap<String,i32>,label:u32,loops:Vec<(String,String)>}
impl G{
 fn lab(&mut self)->String{let s=format!("CL{}",self.label);self.label+=1;s}
 fn e(&mut self,e:&E)->Result<(), &'static str>{match e{
  E::N(n)=>self.a.push_str(&format!("    mov rax, {}\n",n)),
  E::V(n)=>{let o=*self.slots.get(n).ok_or("unknown local")?;self.a.push_str(&format!("    mov rax, [rbp-{}]\n",o))},
  E::Neg(x)=>{self.e(x)?;self.a.push_str("    mov rcx, 0\n    sub rcx, rax\n    mov rax, rcx\n")},
  E::Not(x)=>{self.e(x)?;self.a.push_str("    cmp rax, 0\n    sete al\n    movzx rax, al\n")},
  E::BitNot(x)=>{self.e(x)?;self.a.push_str("    not rax\n")},
  E::Ternary(c,y,n)=>{let els=self.lab();let end=self.lab();self.e(c)?;self.a.push_str(&format!("    cmp rax, 0\n    je {}\n",els));self.e(y)?;self.a.push_str(&format!("    jmp {}\n{}:\n",end,els));self.e(n)?;self.a.push_str(&format!("{}:\n",end))},
  E::Bin(l,op,r) if op=="&&"||op=="||"=>{let short=self.lab();let end=self.lab();self.e(l)?;self.a.push_str("    cmp rax, 0\n");self.a.push_str(&format!("    {} {}\n",if op=="&&"{"je"}else{"jne"},short));self.e(r)?;self.a.push_str("    cmp rax, 0\n    setne al\n    movzx rax, al\n");self.a.push_str(&format!("    jmp {}\n{}:\n    mov rax, {}\n{}:\n",end,short,if op=="&&"{0}else{1},end))},
  E::Bin(l,op,r)=>{self.e(l)?;self.a.push_str("    push rax\n");self.e(r)?;self.a.push_str("    mov rcx, rax\n    pop rax\n");match op.as_str(){"+"=>self.a.push_str("    add rax, rcx\n"),"-"=>self.a.push_str("    sub rax, rcx\n"),"*"=>self.a.push_str("    imul rax, rcx\n"),"/"=>self.a.push_str("    cqo\n    idiv rcx\n"),"%"=>self.a.push_str("    cqo\n    idiv rcx\n    mov rax, rdx\n"),"&"=>self.a.push_str("    and rax, rcx\n"),"|"=>self.a.push_str("    or rax, rcx\n"),"^"=>self.a.push_str("    xor rax, rcx\n"),"<<"=>self.a.push_str("    shl rax, cl\n"),">>"=>self.a.push_str("    sar rax, cl\n"),"=="|"!="|"<"|"<="|">"|">="=>{self.a.push_str("    cmp rax, rcx\n");let c=match op.as_str(){"=="=>"sete","!="=>"setne","<"=>"setl","<="=>"setle",">"=>"setg",_=>"setge"};self.a.push_str(&format!("    {} al\n    movzx rax, al\n",c))},_=>return Err("unsupported C operator")}},
  E::Call(n,args)=>{if args.len()>4{return Err("Win64 calls support up to four arguments")}for x in args{self.e(x)?;self.a.push_str("    push rax\n")}let regs=["rcx","rdx","r8","r9"];for i in (0..args.len()).rev(){self.a.push_str(&format!("    pop {}\n",regs[i]))}self.a.push_str(&format!("    call {}\n",n))}
 }Ok(())}
 fn ss(&mut self,s:&[S])->Result<(),&'static str>{for x in s{match x{
  S::Let(n,e)|S::Set(n,e)=>{self.e(e)?;let o=*self.slots.get(n).ok_or("unknown C local")?;self.a.push_str(&format!("    mov [rbp-{}], rax\n",o))},
  S::Expr(e)=>self.e(e)?,
  S::Ret(e)=>{if let Some(e)=e{self.e(e)?}self.a.push_str("    mov rsp, rbp\n    pop rbp\n    ret\n")},
  S::If(c,y,n)=>{let els=self.lab();let end=self.lab();self.e(c)?;self.a.push_str(&format!("    cmp rax, 0\n    je {}\n",els));self.ss(y)?;self.a.push_str(&format!("    jmp {}\n{}:\n",end,els));self.ss(n)?;self.a.push_str(&format!("{}:\n",end))},
  S::Loop(b)=>{let st=self.lab();let en=self.lab();self.loops.push((st.clone(),en.clone()));self.a.push_str(&format!("{}:\n",st));self.ss(b)?;self.a.push_str(&format!("    jmp {}\n{}:\n",st,en));self.loops.pop();},
  S::Break=>{let e=self.loops.last().ok_or("break outside loop")?.1.clone();self.a.push_str(&format!("    jmp {}\n",e))},
  S::Continue=>{let st=self.loops.last().ok_or("continue outside loop")?.0.clone();self.a.push_str(&format!("    jmp {}\n",st))}
 }}Ok(())}
}
pub fn compile(source:&str)->Result<String,&'static str>{
 let t=super::c_compiler::tokenize(source)?;let(fs,ex)= (P{t,i:0}).program()?;if fs.is_empty(){return Err("no C functions found")}
 let mut asm=String::from("; standalone HPVMx C backend\nBITS 64\nORG 0x100000\n");for n in ex{asm.push_str(&format!("extern {}\n",n))}
 let mut next_label=0;
 for f in fs{let mut ns=f.args.clone();collect(&f.body,&mut ns);let mut slots=HashMap::new();for(i,n)in ns.iter().enumerate(){slots.insert(n.clone(),(i as i32+1)*8);}let frame=((ns.len() as i32*8+15)/16)*16+32;asm.push_str(&format!("{}:\n    push rbp\n    mov rbp, rsp\n    sub rsp, {}\n",f.name,frame));let regs=["rcx","rdx","r8","r9"];for(i,n)in f.args.iter().enumerate(){if i>=4{return Err("Win64 functions support up to four arguments")}asm.push_str(&format!("    mov [rbp-{}], {}\n",slots[n],regs[i]));}let mut g=G{a:asm,slots,label:next_label,loops:Vec::new()};g.ss(&f.body)?;next_label=g.label;g.a.push_str("    mov rsp, rbp\n    pop rbp\n    ret\n\n");asm=g.a;}Ok(asm)
}
