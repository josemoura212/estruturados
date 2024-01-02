enum Fruta {
    Maca,
    Banana,
    Morango,
    Acai,
}

enum Coordenada {
    DoisD(i32, i32),
    TresD(i32, i32, i32),
}

struct Pessoa {
    nome: String,
    idade: u8,
    altura: f32,
}

fn main() {
    enumeracao(Fruta::Morango);
    enumeracao(Fruta::Banana);
    enumeracao(Fruta::Maca);
    enumeracao(Fruta::Acai);

    enumeracao_com_dados();

    estrutura();
}

fn estrutura() {
    let glaucio = Pessoa {
        nome: String::from("Glaucio"),
        idade: 20,
        altura: 1.75,
    };

    println!("\nNome: {}", glaucio.nome);
    println!("Idade: {}", glaucio.idade);
    println!("Altura: {}\n", glaucio.altura);
}

fn enumeracao_com_dados() {
    let ponto2d = Coordenada::DoisD(5, 10);
    let ponto3d = Coordenada::TresD(5, 10, 15);

    match ponto2d {
        Coordenada::DoisD(x, y) => println!("\nCoordenada 2D: x = {}, y = {}", x, y),
        Coordenada::TresD(_, _, _) => println!("Coordenada 3D"),
    }
    match ponto3d {
        Coordenada::DoisD(_, _) => println!("Coordenada 2D"),
        Coordenada::TresD(x, y, z) => println!("\nCoordenada 3D: x = {}, y = {}, z = {}", x, y, z),
    }
}

fn enumeracao(fruta: Fruta) {
    match fruta {
        Fruta::Maca => println!("É uma maçã"),
        Fruta::Banana => println!("É uma banana"),
        Fruta::Morango => println!("É um morango"),
        Fruta::Acai => println!("É um açaí"),
    }
}
