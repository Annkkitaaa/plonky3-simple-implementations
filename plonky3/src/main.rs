use core::borrow::Borrow;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_baby_bear::{BabyBear, Poseidon2BabyBear};
use p3_challenger::DuplexChallenger;
use p3_commit::ExtensionMmcs;
use p3_dft::Radix2DitParallel;
use p3_field::extension::BinomialExtensionField;
use p3_field::{Field, PrimeField64};
use p3_fri::{TwoAdicFriPcs, create_test_fri_params};
use p3_matrix::Matrix;
use p3_matrix::dense::RowMajorMatrix;
use p3_merkle_tree::MerkleTreeMmcs;
use p3_symmetric::{PaddingFreeSponge, TruncatedPermutation};
use p3_uni_stark::{StarkConfig, prove, verify};
use rand::SeedableRng;
use rand::rngs::SmallRng;

const NUM_ARITHMETIC_COLS: usize = 4;

#[derive(Debug, Clone)]
pub struct ArithmeticAir;

impl<F> BaseAir<F> for ArithmeticAir {
    fn width(&self) -> usize {
        NUM_ARITHMETIC_COLS
    }
}

impl<AB: AirBuilder> Air<AB> for ArithmeticAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0).expect("Matrix is empty?");
        let local: &ArithmeticRow<AB::Var> = (*local).borrow();
        
        let constraint = local.a.clone() + local.c.clone() * local.d.clone() - local.e.clone();
        builder.assert_zero(constraint);
    }
}

#[derive(Debug, Clone)]
pub struct ArithmeticRow<F> {
    pub a: F, pub c: F, pub d: F, pub e: F,
}

impl<F> ArithmeticRow<F> {
    const fn new(a: F, c: F, d: F, e: F) -> Self {
        Self { a, c, d, e }
    }
}

impl<F> Borrow<ArithmeticRow<F>> for [F] {
    fn borrow(&self) -> &ArithmeticRow<F> {
        debug_assert_eq!(self.len(), NUM_ARITHMETIC_COLS);
        let (prefix, rows, suffix) = unsafe { self.align_to::<ArithmeticRow<F>>() };
        debug_assert!(prefix.is_empty(), "Alignment should match");
        debug_assert!(suffix.is_empty(), "Alignment should match");
        debug_assert_eq!(rows.len(), 1);
        &rows[0]
    }
}

pub fn generate_arithmetic_trace<F: PrimeField64>() -> RowMajorMatrix<F> {
    let n = 1;
    let mut trace = RowMajorMatrix::new(F::zero_vec(n * NUM_ARITHMETIC_COLS), NUM_ARITHMETIC_COLS);
    
    let (prefix, rows, suffix) = unsafe { trace.values.align_to_mut::<ArithmeticRow<F>>() };
    assert!(prefix.is_empty(), "Alignment should match");
    assert!(suffix.is_empty(), "Alignment should match");
    assert_eq!(rows.len(), n);
    
    rows[0] = ArithmeticRow::new(
        F::from_u64(3), F::from_u64(4), F::from_u64(5), F::from_u64(23)
    );
    
    trace
}

type Val = BabyBear;
type Perm = Poseidon2BabyBear<16>;
type MyHash = PaddingFreeSponge<Perm, 16, 8, 8>;
type MyCompress = TruncatedPermutation<Perm, 2, 8, 16>;
type ValMmcs = MerkleTreeMmcs<<Val as Field>::Packing, <Val as Field>::Packing, MyHash, MyCompress, 8>;
type Challenge = BinomialExtensionField<Val, 4>;
type ChallengeMmcs = ExtensionMmcs<Val, Challenge, ValMmcs>;
type Challenger = DuplexChallenger<Val, Perm, 16, 8>;
type Dft = Radix2DitParallel<Val>;
type Pcs = TwoAdicFriPcs<Val, Dft, ValMmcs, ChallengeMmcs>;
type MyConfig = StarkConfig<Pcs, Challenge, Challenger>;

fn create_config() -> MyConfig {
    let mut rng = SmallRng::seed_from_u64(1);
    let perm = Perm::new_from_rng_128(&mut rng);
    let hash = MyHash::new(perm.clone());
    let compress = MyCompress::new(perm.clone());
    let val_mmcs = ValMmcs::new(hash, compress);
    let challenge_mmcs = ChallengeMmcs::new(val_mmcs.clone());
    let dft = Dft::default();
    let fri_params = create_test_fri_params(challenge_mmcs, 1);
    let pcs = Pcs::new(dft, val_mmcs, fri_params);
    let challenger = Challenger::new(perm);
    MyConfig::new(pcs, challenger)
}

fn main() {
    println!(" Plonky3 Arithmetic Proof System");
    println!("   Proving: a + c * d = e");
    println!("   Values: 3 + 4 * 5 = 23");
    println!();
    
    let air = ArithmeticAir;
    let trace = generate_arithmetic_trace::<Val>();
    let config = create_config();
    
    println!(" Generated execution trace:");
    println!("   Single row: [a=3, c=4, d=5, e=23]");
    println!("   Constraint: a + c * d - e = 0");
    println!("   Check: 3 + 4 * 5 - 23 = 0 ✓");
    println!();
    
    println!(" Generating STARK proof...");
    let proof = prove(&config, &air, trace, &vec![])
        .expect("Failed to generate proof");
    
    println!(" Proof generated successfully!");
    println!();
    
    println!(" Verifying proof...");
    verify(&config, &air, &proof, &vec![])
        .expect("Proof verification failed");
    
    println!(" Proof verified successfully!");
    println!();
    println!(" Summary:");
    println!("   - Created STARK proof for: a + c*d = e");
    println!("   - Values: 3 + 4*5 = 23");
    println!("   - Proof verification completed ✓");
}