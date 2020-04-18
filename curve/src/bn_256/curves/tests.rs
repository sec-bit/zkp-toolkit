#![allow(unused_imports)]
use core::ops::{AddAssign, MulAssign};
use math::{
    curves::{models::SWModelParameters, AffineCurve, PairingEngine, ProjectiveCurve},
    fields::{Field, FpParameters, PrimeField, SquareRootField},
    test_rng, CanonicalSerialize, One, Zero,
};
use rand::Rng;

use crate::{
    bn_256::{g1, g2, Bn_256, Fq, Fq12, Fq2, Fr, G1Affine, G1Projective, G2Affine, G2Projective},
    tests::{
        curves::{curve_tests, sw_curve_serialization_test},
        groups::group_test,
    },
};

#[test]
fn test_g1_projective_curve() {
    curve_tests::<G1Projective>();

    let byte_size = G1Affine::zero().serialized_size();
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

    let byte_size = G2Affine::zero().serialized_size();
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

    let mut sa: G1Projective = a;
    sa.mul_assign(s);
    let mut sb: G2Projective = b;
    sb.mul_assign(s);

    let ans1 = Bn_256::pairing(sa, b);
    let ans2 = Bn_256::pairing(a, sb);

    let ans3 = Bn_256::pairing(a, b).pow(s.into_repr());

    assert_eq!(ans1, ans2);
    assert_eq!(ans1, ans3);
    assert_eq!(ans2, ans3);

    assert_ne!(ans1, Fq12::one());
    assert_ne!(ans2, Fq12::one());
    assert_ne!(ans3, Fq12::one());

    assert_eq!(ans1.pow(Fr::characteristic()), Fq12::one());
    assert_eq!(ans2.pow(Fr::characteristic()), Fq12::one());
    assert_eq!(ans3.pow(Fr::characteristic()), Fq12::one());
}

#[test]
fn test_g1_generator_raw() {
    let mut x = Fq::zero();
    let mut i = 0;
    loop {
        // y^2 = x^3 + b
        let mut rhs = x;
        rhs.square_in_place();
        rhs.mul_assign(&x);
        rhs.add_assign(&g1::Parameters::COEFF_B);

        if let Some(y) = rhs.sqrt() {
            let p = G1Affine::new(x, if y < -y { y } else { -y }, false);
            assert!(p.is_in_correct_subgroup_assuming_on_curve());

            let g1 = p.scale_by_cofactor();
            assert_eq!(g1, p.into());

            if !g1.is_zero() {
                assert_eq!(i, 1);
                let g1 = G1Affine::from(g1);

                assert!(g1.is_in_correct_subgroup_assuming_on_curve());

                assert_eq!(g1, G1Affine::prime_subgroup_generator());
                break;
            }
        }

        i += 1;
        x.add_assign(&Fq::one());
    }
}
