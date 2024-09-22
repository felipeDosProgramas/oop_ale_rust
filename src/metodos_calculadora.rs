use crate::calculo::{Calculo, ResultadoOuErro, T};
use crate::operacao::Operacao;

pub trait MetodosCalculadora {
    fn soma(&mut self);
    fn subtracao(&mut self);
    fn multiplicacao(&mut self);
    fn divisao(&mut self);
}

impl MetodosCalculadora for Calculo<T> {
    fn soma(&mut self) {
        self.operacao = Some(Operacao::Soma);
        self.resultado = Some(Ok(
            self.v1 + self.v2
        ));
    }

    fn subtracao(&mut self) {
        self.operacao = Some(Operacao::Subtracao);
        self.resultado = Some(Ok(
            self.v1 - self.v2
        ));
    }

    fn multiplicacao(&mut self) {
        self.operacao = Some(Operacao::Multiplicacao);
        self.resultado = Some(Ok(
            self.v1 * self.v2
        ));
    }

    fn divisao(&mut self) {
        self.operacao = Some(Operacao::Divisao);
        let r: ResultadoOuErro = if self.v2 == 0 {
            Ok(
                self.v1 / self.v2
            )
        } else {
            Err("divisão por zero é impossível".to_string())
        };
        self.resultado = Some(r);
    }
}
