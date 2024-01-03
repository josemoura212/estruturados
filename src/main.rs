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

struct Retangulo {
    altura: u32,
    largura: u32,
}

trait FormaGeometrica {
    fn calcular_area(&self) -> u64;

    fn new(largura: u32, altura: u32) -> Self;
}

impl FormaGeometrica for Retangulo {
    fn calcular_area(&self) -> u64 {
        (self.altura * self.largura).into()
    }

    fn new(largura: u32, altura: u32) -> Self {
        Self { altura, largura }
    }
}

impl Pessoa {
    fn new(nome: String, idade: u8, altura: f32) -> Self {
        Self {
            nome,
            idade,
            altura,
        }
    }

    fn print(&self) {
        println!("\nNome: {}", self.nome);
        println!("Idade: {}", self.idade);
        println!("Altura: {}\n", self.altura);
    }
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

    let jose = Pessoa::new(String::from("José"), 30, 1.80);

    jose.print();

    println!("\nNome: {}", glaucio.nome);
    println!("Idade: {}", glaucio.idade);
    println!("Altura: {}\n", glaucio.altura);

    let retangulo1 = Retangulo::new(10, 20);

    let retangulo2 = Retangulo {
        altura: 33,
        largura: 5,
    };

    let area1 = retangulo1.calcular_area();

    let area2 = retangulo2.calcular_area();

    println!("Área do retângulo 1: {}\n", area1);
    println!("Área do retângulo 2: {}\n", area2);
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
