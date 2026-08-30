#[allow(unused)]
pub trait Exp2 {
    fn exp2(self) -> Self;
}

impl Exp2 for f32 {
    fn exp2(self) -> Self {
        self.exp2()
    }
}

impl Exp2 for f64 {
    fn exp2(self) -> Self {
        self.exp2()
    }
}

impl<const N: u32, const ES: u32, const RS: u32, Int: fast_posit::Int> Exp2
    for fast_posit::Posit<N, ES, Int, RS>
{
    fn exp2(self) -> Self {
        posit_exp2(self)
    }
}

use fast_posit::{Int, Posit, RoundFrom};

// clanker generated this, since I wasn't able to find any proper generic implementations
#[inline(always)]
fn posit_exp2<const N: u32, const ES: u32, I: Int, const RS: u32>(
    x: Posit<N, ES, I, RS>,
) -> Posit<N, ES, I, RS> {
    type P<const N: u32, const ES: u32, I, const RS: u32> = Posit<N, ES, I, RS>;

    const LN2: f64 = core::f64::consts::LN_2;

    let zero = P::<N, ES, I, RS>::ZERO;
    let one = P::<N, ES, I, RS>::ONE;
    let nar = P::<N, ES, I, RS>::NAR;

    if x == nar {
        return nar;
    }

    if x == zero {
        return one;
    }

    /*
     * A posit's maximum magnitude is a power of two. For an unbounded
     * posit this is:
     *
     *     2^((N - 2) * 2^ES)
     *
     * Thus exp2(x) is guaranteed to overflow/underflow outside this range.
     *
     * Calculate the bound without converting x to an integer.
     */
    let max_exp: u128 = (N as u128 - 2) << ES;

    /*
     * `max_exp` normally fits in i128 for the posit sizes practical here.
     * If ES is enormous, just let the ordinary posit arithmetic handle it.
     */
    if max_exp <= i128::MAX as u128 {
        let bound = P::<N, ES, I, RS>::round_from(max_exp as i128);

        if x > bound {
            return P::<N, ES, I, RS>::MAX;
        }

        if x < -bound {
            return zero;
        }
    }

    /*
     * x = n + f, where f is in [-0.5, 0.5].
     *
     * Using nearest_int rather than floor gives us a much smaller
     * approximation interval.
     */
    let n_p = x.nearest_int();
    let f = x - n_p;

    /*
     * The integer part is now small enough that i128 is safe for all
     * results that can still be represented by the target posit.
     *
     * If conversion saturates at i128::MAX/MIN, the result would already
     * be outside the useful range handled above.
     */
    let n: i128 = RoundFrom::round_from(n_p);

    /*
     * 2^f = exp(f ln 2)
     *
     * For f ∈ [-1/2, 1/2], |f ln 2| <= 0.3466.
     *
     * We deliberately keep the transcendental constant in f64 here:
     * this is the fast approximation version. The polynomial itself is
     * evaluated using Posit arithmetic.
     *
     * For p64 this is a reasonable compromise between speed and accuracy.
     */
    let ln2 = P::<N, ES, I, RS>::round_from(LN2);
    let y = f * ln2;

    /*
     * exp(y), degree 8, Horner form.
     *
     * exp(y) =
     * 1 + y +
     * y²/2! + y³/3! + ... + y⁸/8!
     */
    let c8 = P::<N, ES, I, RS>::round_from(1.0 / 40320.0);
    let c7 = P::<N, ES, I, RS>::round_from(1.0 / 5040.0);
    let c6 = P::<N, ES, I, RS>::round_from(1.0 / 720.0);
    let c5 = P::<N, ES, I, RS>::round_from(1.0 / 120.0);
    let c4 = P::<N, ES, I, RS>::round_from(1.0 / 24.0);
    let c3 = P::<N, ES, I, RS>::round_from(1.0 / 6.0);
    let c2 = P::<N, ES, I, RS>::round_from(0.5);

    let mut p = c8;
    p = c7 + y * p;
    p = c6 + y * p;
    p = c5 + y * p;
    p = c4 + y * p;
    p = c3 + y * p;
    p = c2 + y * p;
    p = one + y * p;

    let mut result = one + y * p;

    /*
     * Multiply by 2^n using exponentiation by squaring.
     *
     * n is an ordinary integer here, not a Posit, so the loop is small.
     * For p64/ES=2 the largest relevant |n| is only 248.
     */
    if n > 0 {
        let mut e = n as u128;
        let mut base = one + one;

        while e != 0 {
            if e & 1 != 0 {
                result *= base;
            }

            e >>= 1;

            if e != 0 {
                base = base * base;
            }
        }
    } else if n < 0 {
        let mut e = (-n) as u128;
        let mut base = one + one;

        while e != 0 {
            if e & 1 != 0 {
                result /= base;
            }

            e >>= 1;

            if e != 0 {
                base = base * base;
            }
        }
    }

    result
}
