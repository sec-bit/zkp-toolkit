use math::{
    test_rng, AffineCurve, CanonicalSerialize, Field, One, PairingEngine, PrimeField,
    ProjectiveCurve, UniformRand, Zero,
};
use rand::Rng;

use crate::mnt4_753::*;

use crate::tests::{curves::*, groups::*};

#[test]
fn test_g1_projective_curve() {
    curve_tests::<G1Projective>();

    let byte_size = <G1Affine as Zero>::zero().serialized_size();
    sw_curve_serialization_test::<g1::Parameters>(byte_size);
}

#[test]
fn test_g1_projective_group() {
    let mut rng = test_rng();
    let a: G1Projective = rng.gen();
    let b: G1Projective = rng.gen();
    group_test(a, b);
}

#[test]
fn test_g1_generator() {
    let generator = G1Affine::prime_subgroup_generator();
    assert!(generator.is_on_curve());
    assert!(generator.is_in_correct_subgroup_assuming_on_curve());
}

#[test]
fn test_g2_projective_curve() {
    curve_tests::<G2Projective>();

    let byte_size = <G2Affine as Zero>::zero().serialized_size();
    sw_curve_serialization_test::<g2::Parameters>(byte_size);
}

#[test]
fn test_g2_projective_group() {
    let mut rng = test_rng();
    let a: G2Projective = rng.gen();
    let b: G2Projective = rng.gen();
    group_test(a, b);
}

#[test]
fn test_g2_generator() {
    let generator = G2Affine::prime_subgroup_generator();
    assert!(generator.is_on_curve());
    assert!(generator.is_in_correct_subgroup_assuming_on_curve());
}

#[test]
fn test_bilinearity() {
    let mut rng = test_rng();
    let a: G1Projective = rng.gen();
    let b: G2Projective = rng.gen();
    let s: Fr = rng.gen();

    let sa = a.mul(s);
    let sb = b.mul(s);

    let ans1 = MNT4_753::pairing(sa, b);
    let ans2 = MNT4_753::pairing(a, sb);
    let ans3 = MNT4_753::pairing(a, b).pow(s.into_repr());

    assert_eq!(ans1, ans2);
    assert_eq!(ans2, ans3);

    assert_ne!(ans1, Fq4::one());
    assert_ne!(ans2, Fq4::one());
    assert_ne!(ans3, Fq4::one());

    assert_eq!(ans1.pow(Fr::characteristic()), Fq4::one());
    assert_eq!(ans2.pow(Fr::characteristic()), Fq4::one());
    assert_eq!(ans3.pow(Fr::characteristic()), Fq4::one());
}

#[test]
fn test_product_of_pairings() {
    let rng = &mut test_rng();

    let a = G1Projective::rand(rng).into_affine();
    let b = G2Projective::rand(rng).into_affine();
    let c = G1Projective::rand(rng).into_affine();
    let d = G2Projective::rand(rng).into_affine();
    let ans1 = MNT4_753::pairing(a, b) * &MNT4_753::pairing(c, d);
    let ans2 = MNT4_753::product_of_pairings(&[(a.into(), b.into()), (c.into(), d.into())]);
    assert_eq!(ans1, ans2);
}
