// The following code is from (scipr-lab's zexe)[https://github.com/scipr-lab/zexe] and thanks for their work

use crate::mnt4_753::{Fq, FQ_ONE};
use math::{
    biginteger::BigInteger768 as BigInteger,
    field_new,
    fields::fp2::{Fp2, Fp2Parameters},
};

pub type Fq2 = Fp2<Fq2Parameters>;

pub struct Fq2Parameters;

impl Fp2Parameters for Fq2Parameters {
    type Fp = Fq;

    // non_residue = 13
    #[rustfmt::skip]
    const NONRESIDUE: Fq = field_new!(Fq, BigInteger([
            11881297496860141143,
            13588356353764843511,
            9969398190777826186,
            17325157081734070311,
            16341533986183788031,
            8322434028726676858,
            13631157743146294957,
            8365783422740577875,
            3010239015809771096,
            11776256826687733591,
            7214251687253691272,
            268626707558702
    ]));

    // qnr = (8, 1)
    const QUADRATIC_NONRESIDUE: (Self::Fp, Self::Fp) = (
        field_new!(
            Fq,
            BigInteger([
                587330122779359758,
                14352661462510473462,
                17802452401246596498,
                18018663494943049411,
                17948754733747257098,
                10253180574146027531,
                6683223122694781837,
                13573468617269213174,
                5059368039312883748,
                950479668716233863,
                9936591501985804621,
                88719447132658
            ])
        ),
        FQ_ONE,
    );

    // Coefficients:
    // [1, 41898490967918953402344214791240637128170709919953949071783502921025352812571106773058893763790338921418070971888253786114353726529584385201591605722013126468931404347949840543007986327743462853720628051692141265303114721689600]
    // see https://github.com/o1-labs/snarky/blob/2cf5ef3a14989e57c17518832b3c52590068fc48/src/camlsnark_c/libsnark-caml/depends/libff/libff/algebra/curves/mnt753/mnt4753/mnt4753_init.cpp
    const FROBENIUS_COEFF_FP2_C1: [Self::Fp; 2] = [
        FQ_ONE,
        field_new!(
            Fq,
            BigInteger([
                14260497802974073023,
                5895249896161266456,
                14682908860938702530,
                17222385991615618722,
                14621060510943733448,
                10594887362868996148,
                7477357615964975684,
                12570239403004322603,
                2180620924574446161,
                12129628062772479841,
                8853285699251153944,
                362282887012814
            ])
        ),
    ];
}
