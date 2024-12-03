use std::f64::consts::PI;
enum Forma{
    Quadrado {lado: f64},
    Circulo {raio: f64},
    Retangulo {altura: f64, largura: f64},
    Triangulo {lado_a: f64, lado_b: f64, angulo_ab: f64}, //angulo em radianos
    Elipse {semieixo_maior: f64, semieixo_menor: f64},
    Cubo {lado: f64},
    Cilindro {altura: f64, raio: f64},
    Esfera {raio: f64},
}
fn main() {
    let circulo = Forma::Circulo {raio: 2.0};
    let quadrado = Forma::Quadrado {lado: 2.0};
    let retangulo = Forma::Retangulo {altura: 2.0, largura: 3.0};
    let triangulo = Forma::Triangulo {lado_a: 2.0, lado_b: 2.0, angulo_ab: PI/3.0};
    let elipse = Forma::Elipse {semieixo_maior: 8.0, semieixo_menor: 1.0};
    let cubo = Forma::Cubo {lado: 2.0};
    let cilindro = Forma::Cilindro {altura: 10.0, raio: 2.0};
    let esfera = Forma::Esfera {raio: 2.0};

    println!("---Perímetro---");
    println!("Circulo -> {:?}", circulo.perimetro());
    println!("Quadrado -> {:?}", quadrado.perimetro());
    println!("Retangulo -> {:?}", retangulo.perimetro());
    println!("Triangulo -> {:?}", triangulo.perimetro());
    println!("Elipse -> {:?}", elipse.perimetro());
    println!("Cubo -> {:?}", cubo.perimetro());
    println!("Cilindro -> {:?}", cilindro.perimetro());
    println!("Esfera -> {:?}", esfera.perimetro());

    println!("---Área---");
    println!("Circulo -> {}", circulo.area());
    println!("Quadrado -> {}", quadrado.area());
    println!("Retangulo -> {}", retangulo.area());
    println!("Triangulo -> {}", triangulo.area());
    println!("Elipse -> {}", elipse.area());
    println!("Cubo -> {}", cubo.area());
    println!("Cilindro -> {}", cilindro.area());
    println!("Esfera -> {}", esfera.area());

    println!("---Volume---");
    println!("Circulo -> {:?}", circulo.volume());
    println!("Quadrado -> {:?}", quadrado.volume());
    println!("Retangulo -> {:?}", retangulo.volume());
    println!("Triangulo -> {:?}", triangulo.volume());
    println!("Elipse -> {:?}", elipse.volume());
    println!("Cubo -> {:?}", cubo.volume());
    println!("Cilindro -> {:?}", cilindro.volume());
    println!("Esfera -> {:?}", esfera.volume());
}
impl Forma {
    fn perimetro(&self) -> Option<f64>{
        match self {
            Forma::Circulo {raio} => Some(2.0*raio*PI),
            Forma::Quadrado {lado} => Some(4.0*lado),
            Forma::Retangulo {altura, largura} => Some(2.0*(altura+largura)),
            Forma::Triangulo {lado_a, lado_b, angulo_ab} => {
                let lado_c = (lado_a.powf(2.0)+lado_b.powf(2.0)-2.0*lado_a*lado_b*angulo_ab.cos()).sqrt();
                Some(lado_a+lado_b+lado_c)
            },
            Forma::Elipse {semieixo_maior:a, semieixo_menor: b} => {
                Some(PI*(3.0*(a+b)-((3.0*a+b)*(a+3.0*b)).sqrt())) //aproximação
            }
            Forma::Cubo {..} | Forma::Cilindro {..} | Forma::Esfera {..} => None,
        }
    }
    fn area(&self) -> f64 {
        match self {
            Forma::Circulo { raio } => raio.powf(2.0)*PI,
            Forma::Quadrado {lado} => lado.powf(2.0),
            Forma::Retangulo {altura, largura} => altura*largura,
            Forma::Triangulo {lado_a, lado_b, angulo_ab} => lado_a*lado_b*angulo_ab.sin()/2.0,
            Forma::Elipse {semieixo_maior, semieixo_menor} => {PI*semieixo_maior*semieixo_menor},
            Forma::Cubo {lado} => {6.0*lado.powf(2.0)},
            Forma::Cilindro {altura, raio} => {2.0*PI*(raio.powf(2.0)+raio*altura)},
            Forma::Esfera {raio} => {4.0*PI*raio.powf(2.0)},
        }
    }
    fn volume(&self) -> Option<f64> {
        match self {
            Forma::Cubo {lado} => {Some(lado.powf(3.0))},
            Forma::Cilindro {altura, raio} => {Some(raio.powf(2.0)*PI*altura)},
            Forma::Esfera {raio} => {Some(4.0/3.0*PI*raio.powf(3.0))},
            _ => None,
        }
    }
}
