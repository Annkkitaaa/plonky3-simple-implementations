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
        let local = main.row_slice(0);
        let next = main.row_slice(1);

        let local: &FibonacciRow<AB::Var> = (*local).borrow();
        let next: &FibonacciRow<AB::Var> = (*next).borrow();

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

pub fn generate_fibonacci_trace<F: PrimeField64>(num_steps: usize) -> RowMajorMatrix<F> {
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
    rows[0] = FibonacciRow::new(F::zero(), F::one());

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
