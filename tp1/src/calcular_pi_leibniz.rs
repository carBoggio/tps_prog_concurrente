pub fn calcular_pi_leibniz(i: u64) -> f64 {
    let mut sumatoria: f64 = 0.0;

    for n in 0u64..=i {
        // El signo alterna: positivo para n par, negativo para n impar
        let signo = if n % 2 == 0 { 1.0 } else { -1.0 };
        
        // El denominador sigue la forma 2n + 1 (usamos u64 para evitar overflow)
        let denominador = (2 * n + 1) as f64;
        
        sumatoria += signo / denominador;
    }

    // Al final multiplicamos por 4 para despejar Pi
    sumatoria * 4.0
}