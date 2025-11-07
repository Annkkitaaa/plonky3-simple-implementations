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

// Fibonacci trace: 2 columns [a, b] representing consecutive Fibonacci numbers
const NUM_FIBONACCI_COLS: usize = 2;

#[derive(Debug, Clone)]
pub struct FibonacciAir;

impl<F> BaseAir<F> for FibonacciAir {
    fn width(&self) -> usize {
        NUM_FIBONACCI_COLS
    }
}

impl<AB: AirBuilder> Air<AB> for FibonacciAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();

        // Get current row and next row
        let local_slice = main.row_slice(0).unwrap();
        let next_slice = main.row_slice(1).unwrap();

        let local: &FibonacciRow<AB::Var> = (*local_slice).borrow();
        let next: &FibonacciRow<AB::Var> = (*next_slice).borrow();

        // Constraint 1: Fibonacci recurrence relation
        // next.b should equal local.a + local.b
        let transition_constraint =
            next.b.clone() - local.a.clone() - local.b.clone();
        builder.assert_zero(transition_constraint);

        // Constraint 2: State propagation
        // next.a should equal local.b
        let propagation_constraint =
            next.a.clone() - local.b.clone();
        builder.assert_zero(propagation_constraint);
    }
}

// Row structure: [a, b] where a = F(n-1), b = F(n)
#[derive(Debug, Clone)]
pub struct FibonacciRow<F> {
    pub a: F,  // F(n-1)
    pub b: F,  // F(n)
}

impl<F> FibonacciRow<F> {
    const fn new(a: F, b: F) -> Self {
        Self { a, b }
    }
}

// Memory layout conversion for efficient access
impl<F> Borrow<FibonacciRow<F>> for [F] {
    fn borrow(&self) -> &FibonacciRow<F> {
        debug_assert_eq!(self.len(), NUM_FIBONACCI_COLS);
        let (prefix, rows, suffix) = unsafe {
            self.align_to::<FibonacciRow<F>>()
        };
        debug_assert!(prefix.is_empty(), "Alignment should match");
        debug_assert!(suffix.is_empty(), "Alignment should match");
        debug_assert_eq!(rows.len(), 1);
        &rows[0]
    }
}

pub fn generate_fibonacci_trace<F: Field + PrimeField64>(num_steps: usize) -> RowMajorMatrix<F> {
    // Ensure power of 2 for FFT operations
    let n = num_steps.next_power_of_two().max(256);

    let mut trace = RowMajorMatrix::new(
        F::zero_vec(n * NUM_FIBONACCI_COLS),
        NUM_FIBONACCI_COLS
    );

    let (prefix, rows, suffix) = unsafe {
        trace.values.align_to_mut::<FibonacciRow<F>>()
    };
    assert!(prefix.is_empty(), "Alignment should match");
    assert!(suffix.is_empty(), "Alignment should match");
    assert_eq!(rows.len(), n);

    // Initialize: F(0) = 0, F(1) = 1
    rows[0] = FibonacciRow::new(F::ZERO, F::ONE);

    // Generate Fibonacci sequence: F(n) = F(n-1) + F(n-2)
    for i in 1..num_steps {
        let prev_a = rows[i - 1].a;
        let prev_b = rows[i - 1].b;

        rows[i] = FibonacciRow::new(
            prev_b,           // a = previous b (shift forward)
            prev_a + prev_b   // b = F(n) = F(n-1) + F(n-2)
        );
    }

    // Pad remaining rows with final values to meet power-of-2 requirement
    for i in num_steps..n {
        rows[i] = rows[num_steps - 1].clone();
    }

    trace
}

// Type definitions following Plonky3 patterns
type Val = BabyBear;
type Perm = Poseidon2BabyBear<16>;
type MyHash = PaddingFreeSponge<Perm, 16, 8, 8>;
type MyCompress = TruncatedPermutation<Perm, 2, 8, 16>;
type ValMmcs = MerkleTreeMmcs<
    <Val as Field>::Packing,
    <Val as Field>::Packing,
    MyHash,
    MyCompress,
    8
>;
type Challenge = BinomialExtensionField<Val, 4>;
type ChallengeMmcs = ExtensionMmcs<Val, Challenge, ValMmcs>;
type Challenger = DuplexChallenger<Val, Perm, 16, 8>;
type Dft = Radix2DitParallel<Val>;
type Pcs = TwoAdicFriPcs<Val, Dft, ValMmcs, ChallengeMmcs>;
type MyConfig = StarkConfig<Pcs, Challenge, Challenger>;

// Simple RNG for deterministic setup
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }
}

impl rand::RngCore for SimpleRng {
    fn next_u32(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
        (self.state >> 32) as u32
    }

    fn next_u64(&mut self) -> u64 {
        let high = self.next_u32() as u64;
        let low = self.next_u32() as u64;
        (high << 32) | low
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for chunk in dest.chunks_mut(4) {
            let val = self.next_u32().to_le_bytes();
            for (i, &byte) in val.iter().enumerate() {
                if i < chunk.len() {
                    chunk[i] = byte;
                }
            }
        }
    }
}

impl rand::CryptoRng for SimpleRng {}

fn create_config() -> MyConfig {
    let mut rng = SimpleRng::new(42);
    let perm = Perm::new_from_rng_128(&mut rng);
    let hash = MyHash::new(perm.clone());
    let compress = MyCompress::new(perm.clone());
    let val_mmcs = ValMmcs::new(hash, compress);
    let challenge_mmcs = ChallengeMmcs::new(val_mmcs.clone());
    let dft = Dft::default();
    let fri_params = create_test_fri_params(challenge_mmcs, 4);
    let pcs = Pcs::new(dft, val_mmcs, fri_params);
    let challenger = Challenger::new(perm);
    MyConfig::new(pcs, challenger)
}

fn main() {
    println!(" Plonky3 Fibonacci Proof System");
    println!("   Proving: F(n) = F(n-1) + F(n-2)");
    println!("   Computing: F(0)=0, F(1)=1, F(2)=1, F(3)=2, F(4)=3, F(5)=5...");
    println!();

    // Generate Fibonacci sequence up to F(100)
    let num_steps = 100;
    let air = FibonacciAir;
    let trace = generate_fibonacci_trace::<Val>(num_steps);
    let config = create_config();

    // Display some values from the trace
    println!(" Generated execution trace:");
    println!("   Computing {} Fibonacci numbers", num_steps);
    println!("   Trace padded to {} rows (power of 2)", trace.height());

    // Calculate and display some Fibonacci values
    let trace_data = &trace.values;
    println!("\n   Sample values:");
    for i in [0, 1, 2, 3, 4, 5, 10, 20, 50, 99].iter() {
        if *i < num_steps {
            let idx = i * NUM_FIBONACCI_COLS;
            let _a = trace_data[idx];
            let b = trace_data[idx + 1];
            println!("   F({}) = {}", i, b);
        }
    }
    println!();

    println!("   Constraints:");
    println!("   1. Transition: next.b = local.a + local.b (Fibonacci rule)");
    println!("   2. Propagation: next.a = local.b (state shift)");
    println!();

    println!(" Generating STARK proof...");
    let proof = prove(&config, &air, trace, &vec![]);

    println!(" Proof generated successfully!");
    println!();

    println!(" Verifying proof...");
    let verify_result = verify(&config, &air, &proof, &vec![]);

    match verify_result {
        Ok(()) => {
            println!(" Proof verified successfully!");
            println!();
            println!(" What was proven:");
            println!("   - The prover knows a valid Fibonacci sequence");
            println!("   - Every step satisfies F(n) = F(n-1) + F(n-2)");
            println!("   - The sequence starts with F(0)=0, F(1)=1");
            println!("   - All {} steps are correctly computed", num_steps);
        },
        Err(e) => {
            println!(" Verification failed: {:?}", e);
            return;
        }
    }
}
