use crate::calculo::{Calculo, T};

pub struct MemoriaCalculadora{
    operacoes: Vec<Calculo<T>>
}
impl MemoriaCalculadora {
    pub fn new() -> Self {
        MemoriaCalculadora {
            operacoes: Vec::new()
        }
    }
    pub fn set_operacao(&mut self, calculo: Calculo<T>) -> usize {
        self.operacoes.push(calculo);
        self.operacoes.len() - 1
    }
    pub fn verificar_integridade_calculo(o: &Calculo<T>) -> bool {
        if o.operacao.is_none() {
            println!("operação não selecionada");
            false
        }
        else if o.resultado.clone().is_none() {
            println!("resultado não calculado");
            false
        }
        else if o.resultado.clone().unwrap().is_err() {
            println!("{}", o.clone().resultado.unwrap().err().unwrap());
            false
        }
        else {
            true
        }
    }
    fn imprimir_operacao(o: &Calculo<T>) {
        if !Self::verificar_integridade_calculo(&o) {
            return;
        }
        println!(
            "{} {} {} = {}",
            o.v1, o.operacao.clone().unwrap().to_string(),
            o.v2, o.resultado.clone().unwrap().unwrap()
        );
    }
    pub fn dump(&self) {
        self.operacoes.iter().for_each(Self::imprimir_operacao);
    }
}