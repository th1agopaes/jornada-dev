//! MC2 · Rust do zero — Módulo 5 · Aula 2: `String` de novo e `HashMap<K, V>`
//! Objetivo: montar e limpar textos, ler letras sem índice e guardar pares chave → valor.
//! Rodar:    `cargo run --bin m5_string_hashmap`

// `HashMap` não vem no prelude (o kit que já chega importado: Vec, String, Option...)
use std::collections::HashMap;

// --- 1. String: juntar e limpar ---------------------------------

fn montar_textos() {
    // `push_str` junta um &str no fim; `push` junta um char só (Módulo 2)
    let mut aviso = String::from("Frieren");
    aviso.push_str(" entrou na masmorra");
    aviso.push('!');
    println!("{aviso}"); // -> Frieren entrou na masmorra!

    // `+` também junta, mas MOVE o texto da esquerda e pede &str na direita
    let heroi = String::from("Himmel");
    let sacerdote = String::from("Heiter");
    let dupla = heroi + " e " + &sacerdote; // `&sacerdote` (&String) vira &str sozinho
    // println!("{heroi}"); // E0382: `heroi` foi movido para dentro de `dupla`

    // `format!` só empresta: nada é movido e se lê melhor (o preferido)
    let grupo = format!("{dupla}, com Frieren e Eisen");
    println!("{grupo}"); // -> Himmel e Heiter, com Frieren e Eisen
    println!("{sacerdote} continua valendo"); // -> Heiter continua valendo

    // Métodos que não alteram: devolvem uma fatia (`trim`) ou uma String nova
    let digitado = "    ATACAR Slime    ";
    println!("[{}]", digitado.trim()); // -> [ATACAR Slime]
    println!("{}", digitado.trim().to_lowercase()); // -> atacar slime
    println!("{}", digitado.trim().starts_with("ATACAR")); // -> true
}

// ---2. String: sem índice, com chars() ---------------------------------

fn ler_letras() {
    let feitico = String::from("Zoltraak");

    // ❌ E0277: texto não aceita índice; em UTF-8, uma letra ocupa de 1 a 4 bytes
    // let primeira = feitico[0];

    // `chars()` é um iterador de char; `next()` puxa o próximo e devolve Option<char>
    println!("{:?}", feitico.chars().next()); // -> Some('Z')
    println!("{:?}", feitico.chars().nth(3)); // -> Some('t')
    println!("{:?}", feitico.chars().nth(20)); // -> None

    // `len()` conta bytes; `chars().count()` conta chars (Módulo 2)
    let magia = "Ação";
    println!("{} bytes, {} chars", magia.len(), magia.chars().count()); // -> 6 bytes, 4 chars
    // &magia[0..2] -> panic: end byte index 2 is not a char boundary: it is inside 'ç'

    for letra in magia.chars() {
        print!("{letra} ");
    }
    println!(); // -> A ç ã o
}

// --- 3. HashMap: criar, inserir e ler ---------------------------------

fn criar_inventario() -> HashMap<String, u32> {
    let mut inventario = HashMap::new();
    inventario.insert(String::from("Poção"), 3);
    inventario.insert(String::from("Éter"), 1);

    // Chave repetida não duplica: `ìnsert` troca o valor e devolve o antigo em Option
    let antigo = inventario.insert(String::from("Poção"), 5);
    println!("{antigo:?}"); // -> Some(3)

    // `get` devolve Option<&u32>; a chave é String, mas a busca aceita &str
    println!("{:?}", inventario.get("Poção")); // -> Some(5)
    // `copied()` tira o & de dentro (Option<&u32> -> Option<u32>) para usar o `unwrap_or`
    println!("{}", inventario.get("Elixir").copied().unwrap_or(0)); // -> 0
    println!("{}", inventario.contains_key("Éter")); // -> true

    // A chave String é movida para dentro do mapa
    let item = String::from("Grimório");
    inventario.insert(item, 1);
    // println!("{item}"); // E0382: `item` agora pertence ao inventário

    inventario
}

// --- 4. HashMap: alterar, remover e percorre ------------------------

fn atualizar(inventario: &mut HashMap<String, u32>) {
    // `values_mut()` entrega &mut de cada valor: bônus de evento, +1 em tudo
    for qtd in inventario.values_mut() {
        *qtd += 1;
    }

    // `remove` tira o par e devolve o valor em Option
    println!("{:?}", inventario.remove("Éter")); // -> Some(2)
    println!("{:?}", inventario.remove("Éter")); // -> None
    println!("{} tipos de item", inventario.len()); // -> 2 tipos de item
}

fn mostrar(inventario: &HashMap<String, u32>) {
    // Cada volta entrega (&String, &u32). A ORDEM É ALEATÓRIA e muda a cada execução
    for (nome, qtd) in inventario {
        println!("{nome}: {qtd}");
    }
}

// --- 5. entry: contar o loot ------------------------

fn contar_loot(drops: &[&str]) -> HashMap<String, u32> {
    let mut contagem = HashMap::new();
    // `for &item` desmonta o &&str e entrega o &str (como o `for &n` do Módulo 2)
    for &item in drops {
        // `entry` acha a vaga da chave; `or_insert(0)` põe 0 se ela estiver vazia e
        // devolve &mut u32 para o valor que está DENTRO do mapa; o `*` soma nele
        *contagem.entry(item.to_string()).or_insert(0) += 1;
    }
    contagem
}

// --- 6. Ranking: do mapa para um Vec ordenado ------------------------

// Uma referência de entrada -> o &str da saída vive enquanto o placar vive (lifetime elision)
fn ranking(placar: &HashMap<String, u32>) -> Vec<(u32, &str)> {
    let mut lista = Vec::new();
    for (nome, pontos) in placar {
        lista.push((*pontos, nome.as_str()));
    }
    // Tuplas comparam campo a campo: primeiro os pontos; no empate, o nome
    lista.sort();
    lista.reverse(); // do maior para o menor
    lista
}

// --- 7. Juntando tudo ------------------------

fn main() {
    montar_textos();
    ler_letras();

    let mut inventario = criar_inventario();
    atualizar(&mut inventario);
    mostrar(&inventario); // -> Poção: 6 e Grimório: 2, em qualquer ordem

    let loot = contar_loot(&["Gosma", "Moeda", "Gosma", "Osso", "Moeda", "Gosma"]);
    println!("{:?}", ranking(&loot)); // -> [(3, "Gosma"), (2, "Moeda"), (1, "Osso")]

    // `HashMap::from` cria a partir de um array de pares (chave, valor)
    let placar = HashMap::from([
        (String::from("Frieren"), 980),
        (String::from("Fern"), 1250),
        (String::from("Stark"), 720),
        (String::from("Himmel"), 1100),
    ]);
    let top = ranking(&placar);
    for (i, (pontos, nome)) in top.iter().enumerate() {
        println!("{}º {nome}: {pontos}", i + 1); // -> 1º Fern: 1250 ... 4º Stark: 720
    }
}
