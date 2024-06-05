use criterion::{black_box, criterion_group, criterion_main, Criterion};

use curve25519_dalek::{RistrettoPoint, Scalar};
use homomorphic_commitments::{elgamal, pedersen};

pub fn benchmark_pedersen(c: &mut Criterion) {
    let public_key = pedersen::PublicKey::random(rand::rngs::OsRng);
    let commit_to = Scalar::random(&mut rand::rngs::OsRng);

    c.bench_function("pedersen-commit", |b| {
        b.iter(|| {
            let _ = pedersen::Commitment::create(
                black_box(commit_to),
                black_box(public_key),
                rand::rngs::OsRng,
            );
        })
    });

    let (valid_commitment, opening_info) = pedersen::Commitment::create(
        black_box(commit_to),
        black_box(public_key),
        rand::rngs::OsRng,
    );
    c.bench_function("pedersen-verify", |b| {
        b.iter(|| {
            let _ = valid_commitment.is_valid(public_key, opening_info);
        })
    });
}

pub fn benchmark_elgamal(c: &mut Criterion) {
    let public_key = elgamal::PublicKey::random(rand::rngs::OsRng);
    let commit_to = RistrettoPoint::random(&mut rand::rngs::OsRng);

    c.bench_function("elgamal-commit", |b| {
        b.iter(|| {
            let _ = elgamal::Commitment::create(
                black_box(commit_to),
                black_box(public_key),
                rand::rngs::OsRng,
            );
        })
    });

    let (valid_commitment, opening_info) = elgamal::Commitment::create(
        black_box(commit_to),
        black_box(public_key),
        rand::rngs::OsRng,
    );
    c.bench_function("elgamal-verify", |b| {
        b.iter(|| {
            let _ = valid_commitment.is_valid(public_key, opening_info);
        })
    });
}

criterion_group!(benches, benchmark_pedersen, benchmark_elgamal);
criterion_main!(benches);