use std::fmt::{Display, Formatter};
#[derive(Clone)]
pub enum Operacao {
    Soma,
    Subtracao,
    Multiplicacao,
    Divisao
}
impl Display for Operacao {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let w = |s| format!("{}", s);
        let str = match self {
            Operacao::Soma =>           w("+"),
            Operacao::Divisao =>        w("÷"),
            Operacao::Subtracao =>      w("-"),
            Operacao::Multiplicacao =>  w("*"),
        };
        write!(f, "{}", str)
    }
}