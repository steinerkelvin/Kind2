pub fn build_str_term(txt: &str) -> hvm::language::Term {
  use hvm::language::Term;
  let empty = Term::Ctr { name: "StrNil".to_string(), args: Vec::new() };
  let list = txt.chars().rfold(empty, |t, c| Term::Ctr {
    name: "StrCons".to_string(),
    args: vec![Box::new(Term::U32 { numb: c as u32 }), Box::new(t)],
  });
  list
}
