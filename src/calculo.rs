use crate::operacao::Operacao;

pub type T = i32;
pub type ResultadoOuErro = Result<T, String>;
pub type NadaOuResultadoOuErro = Option<ResultadoOuErro>;

#[derive(Clone)]
pub struct Calculo<T> {
    pub v1: T,
    pub v2: T,
    pub operacao: Option<Operacao>,
    pub resultado: NadaOuResultadoOuErro
}
impl Calculo<T> {
    pub fn new(v1: T, v2: T) -> Self {
        Self {
            v1, v2,
            operacao: None,
            resultado: None
        }
    }
}