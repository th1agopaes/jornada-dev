//! MC2 · Rust do zero — Módulo 6 · Aula 1: módulos, `pub` e `use`
//! Objetivo: separar o código em módulos, escolher o que fica público e encurtar caminhos.
//! Rodar:    `cargo run --bin m6_modulos`

// --- 1. `mod`: uma sala com porta própria ---------------------------------

mod loja {
    // Tudo dentro de um módulo nasce privado: só ele (e os módulos de dentro dele) enxerga
    fn preco_base(item: &str) -> u32 {
        match item {
            "poção" => 50,
            "elixir" => 300,
            _ => 10,
        }
    }

    // `pub` abre o item para quem está fora do módulo
    pub fn preco(item: &str, qtd: u32) -> u32 {
        preco_base(item) * qtd // mesmo módulo: a função privada é visível
    }
}

fn comprar() {
    // Caminho (path) com `::`, o mesmo de `HashMap::new()`: módulo::item
    println!("3 poções: {} de ouro", loja::preco("poção", 3)); // -> 3 poções: 150 de ouro
}

// --- 2. Struct pública, campo privado ---------------------------------

mod guilda {
    const NIVEL_MAX: u32 = 99;

    // `pub struct` libera o tipo; cada campo decide sozinho se é público
    pub struct Aventureiro {
        pub nome: String,
        nivel: u32, // privado: só muda pelos métodos, então nunca passa de NIVEL_MAX
    }

    impl Aventureiro {
        // Com um campo privado, quem está fora não consegue montar `Aventureiro { ... }`
        // à mão: o `new` vira a única porta de entrada
        pub fn new(nome: &str) -> Self {
            Aventureiro {
                nome: nome.to_string(),
                nivel: 1,
            }
        }

        pub fn nivel(&self) -> u32 {
            self.nivel
        }

        pub fn subir_nivel(&mut self) {
            self.nivel = (self.nivel + 1).min(NIVEL_MAX);
        }
    }

    // Num `pub enum`, todas as variantes ficam públicas juntas
    pub enum Classe {
        Maga,
        Guerreiro,
    }

    impl Classe {
        pub fn titulo(&self) -> &'static str {
            match self {
                Classe::Maga => "maga",
                Classe::Guerreiro => "guerreiro",
            }
        }
    }
}

fn recrutar() {
    // Sem `use`, o caminho fica comprido: módulo::Tipo::função (a seção 4 encurta)
    let mut fern = guilda::Aventureiro::new("Fern");
    fern.subir_nivel();
    fern.nome.push_str(", a aprendiz"); // campo `pub`: lê e escreve de fora
    let titulo = guilda::Classe::Maga.titulo();
    println!("{} ({titulo}), nível {}", fern.nome, fern.nivel());
    // -> Fern, a aprendiz (maga), nível 2
}

// --- 3. Caminhos: `crate::`, `super::` e `self::` ---------------------------------

const OURO_POR_SALA: u32 = 100;

mod masmorra {
    // Para chegar a `entrar` de fora, o caminho inteiro precisa etar aberto: `sala` e `entrar`
    pub mod sala {
        pub fn entrar(numero: u32) -> String {
            // `super::` sobe um nível (para `masmorra`), como o `..` do terminal
            let perigo = super::perigo(numero);
            // `crate::` parte da raiz do crate (este arquivo), como o `/` do terminal
            let ouro = crate::OURO_POR_SALA * numero;
            format!("sala {numero}: perigo {perigo}, {ouro} de ouro")
        }
    }

    // Privada, mas `sala` mora dentro de `masmorra`: quem está dentro enxerga o de fora
    fn perigo(numero: u32) -> &'static str {
        if numero > 2 { "alto" } else { "baixo" }
    }

    pub fn chefe() -> String {
        // `self::` = o módulo atual, como o `./` do terminal (aqui dava para omitir)
        self::sala::entrar(3)
    }
}

fn caminhos() {
    // Caminho absoluto: começa em `crate`
    println!("{}", crate::masmorra::sala::entrar(1)); // -> sala 1: perigo baixo, 100 de ouro
    // Caminho relativo: começa no módulo atual (aqui, a raiz)
    println!("{}", masmorra::chefe()); // -> sala 3: perigo alto, 300 de ouro
}

// --- 4. `use`: atalhos para caminhos longos ---------------------------------

fn atalhos() {
    // Atalho criado dentro de uma função vale até a `}` dela; no topo do arquivo, vale
    // para o módulo inteiro (o `use std::collections::HashMap;` do Módulo 5)
    use crate::guilda::{Aventureiro, Classe}; // tipo entra pelo próprio nome
    use crate::masmorra::sala; // função entra pelo módulo pai: `sala::entrar``mostra a origem
    use crate::masmorra::sala::entrar as visitar; // `as` dá outro nome ao atalho

    let himmel = Aventureiro::new("Himmel");
    let classe = Classe::Guerreiro;
    println!("{} ({})", himmel.nome, classe.titulo()); // -> Himmel (guerreiro)
    println!("{}", sala::entrar(2)); // -> sala 2: perigo baixo, 200 de ouro
    println!("{}", visitar(4)); // -> sala 4: perigo alto, 400 de ouro
}

fn main() {
    comprar();
    recrutar();
    caminhos();
    atalhos();
}
