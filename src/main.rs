mod calculo;
mod operacao;
mod memoria_calculadora;
mod metodos_calculadora;

use crate::calculo::*;
use crate::memoria_calculadora::MemoriaCalculadora;
use crate::metodos_calculadora::MetodosCalculadora;

type QuatroCalculos = (Calculo<T>, Calculo<T>, Calculo<T>, Calculo<T>);
fn preparar_calculos() -> QuatroCalculos {
    let mut c = Calculo::new(5, 10);
            c.soma();
    let mut c1 = Calculo::new(-50, 10);
            c1.multiplicacao();
    let mut c2 = Calculo::new(20, -5);
            c2.divisao();
    let mut c3 = Calculo::new(20, 10);
            c3.subtracao();
    (c, c1, c2, c3)
}
fn main() {
    let (c, c1, c2, c3) = preparar_calculos();
    let mut memoria = MemoriaCalculadora::new();
            memoria.set_operacao(c);
            memoria.set_operacao(c1);
            memoria.set_operacao(c2);
            memoria.set_operacao(c3);
            memoria.dump();
}
