mod bignum_u512;
mod ed25519;
mod u256_mod;

#[cfg(test)]
mod test_utils;

fn main() {
    println!("The coordinates of the generator point are: \nx: {} \ny: {}\n", ed25519::G.get_x(), ed25519::G.get_y());

    let benchmark_start_time = std::time::Instant::now();

    let point_mul_result = ed25519::PRIME_GROUP_ORDER * &ed25519::G;
    assert_eq!(point_mul_result, ed25519::CurvePoint::identity());

    println!("Point multiplication took {}ms", benchmark_start_time.elapsed().as_millis());
}
