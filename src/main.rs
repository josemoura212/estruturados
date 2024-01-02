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

fn main() {
    enumeracao(Fruta::Morango);
    enumeracao(Fruta::Banana);
    enumeracao(Fruta::Maca);
    enumeracao(Fruta::Acai);

    let ponto2d = Coordenada::DoisD(5, 10);
    let ponto3d = Coordenada::TresD(5, 10, 15);

    match ponto2d {
        Coordenada::DoisD(x, y) => println!("Coordenada 2D: x = {}, y = {}", x, y),
        Coordenada::TresD(_, _, _) => println!("Coordenada 3D"),
    }
    match ponto3d {
        Coordenada::DoisD(_, _) => println!("Coordenada 2D"),
        Coordenada::TresD(x, y, z) => println!("Coordenada 3D: x = {}, y = {}, z = {}", x, y, z),
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
