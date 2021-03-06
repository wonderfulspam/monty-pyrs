//! # Monty-rs - A blazing fast, mostly stupid Monty Hall problem simulator
//!
//!
//! See the [Wikipedia article](https://en.wikipedia.org/wiki/Monty_Hall_problem)
//! for an explanation of the Monty Hall problem and its origins.
//!
//! This project aims to be the fastest Monty Hall simulator in existence.
//! To that end, some corners are cut:
//!
//! - Random number generation is fast rather than properly random
//! - The first option is always chosen as the initial guess
//! - When an incorrect option is removed following the initial choice, the
//! simulation does not randomly pick an option to remove, it simply removes
//! the first option. Half the time (ie. when the initial choice is not
//! correct and there is only one possible option that can be removed) this
//! makes no difference, and regardless, it doesn't really matter. What we
//! care about is whether switching is more successful than not switching.
//!
//! This is likely also the silliest Monty Hall problem simulator in existence.
//! This is a non-goal.

use derive_more::AddAssign; // Adds += overload for Results struct
use pyo3::prelude::*; // Macros for exposing Rust code to Python
use rand_core::{RngCore, SeedableRng}; // Traits for generating random numbers and seeding
use rand_xorshift::XorShiftRng; // The fastest possible (?) random number generator
use tinyvec::{array_vec, ArrayVec}; // The smallest possible (?) data structure that implements removal

/// Inner struct for [Results](struct.Results.html) that tracks wins and losses for
/// a given strategy
#[derive(Default, AddAssign)]
struct ResultSet {
    wins: u64,
    losses: u64,
}

/// Tracks results for the two possible strategies: switching and staying.
#[derive(Default, AddAssign)]
#[pyclass]
pub struct Results {
    switched: ResultSet,
    stayed: ResultSet,
}

#[pymethods]
impl Results {
    /// Calculate win rates for the two strategies as percentages.
    ///
    /// ```rust
    /// use monty_pyrs::{Results, play_threaded};
    /// use assert_approx_eq::assert_approx_eq;
    ///
    /// let results: Results = play_threaded(1_000_000);
    /// let (switched_pct, stayed_pct) = results.calc_win_rate();
    /// // Ensure we are within 0.5 of target percentage
    /// assert_approx_eq!(switched_pct, 0.6667, 0.005);
    /// assert_approx_eq!(stayed_pct, 0.3333, 0.005);
    /// ```
    pub fn calc_win_rate(&self) -> (f64, f64) {
        (
            self.switched.wins as f64 / (self.switched.wins + self.switched.losses) as f64,
            self.stayed.wins as f64 / (self.stayed.wins + self.stayed.losses) as f64,
        )
    }
}

/// Holds the RNG for generating a random correct door
/// as well as the impl for playing games.
pub struct MontyHall<R> {
    rng: R,
}

impl<R> MontyHall<R>
where
    R: RngCore,
{
    /// Allows you to BYO random generator.
    /// ```rust
    /// use monty_pyrs::MontyHall;
    /// use rand_xorshift::XorShiftRng;
    /// use rand_core::SeedableRng;
    ///
    /// let rng = XorShiftRng::seed_from_u64(1337);
    /// let mut monty = MontyHall::new_with_rng(rng);
    /// let success = monty.play_single(true);
    /// ```
    pub fn new_with_rng(rng: R) -> Self {
        Self { rng }
    }

    /// Play a single simulation of the Monty Hall problem.
    /// ```rust
    /// use monty_pyrs::MontyHall;
    ///
    /// let mut monty = MontyHall::default();
    /// let success = monty.play_single(true);
    /// ```
    pub fn play_single(&mut self, switch_doors: bool) -> bool {
        let mut doors: ArrayVec<[i8; 3]> = array_vec![0, 1, 2];
        let correct_door = (self.rng.next_u32() % 3) as i8;
        let mut choice: i8 = 0; // https://xkcd.com/221/, sort of

        // Find the first non-correct, non-chosen door and remove it
        doors
            .iter()
            .position(|&x| x != correct_door && x != choice)
            .map(|e| doors.remove(e));

        if switch_doors {
            // Unwrapping is safe; we know there will always be at least one viable option left
            choice = *doors.iter().find(|&&x| x != choice).unwrap();
        }

        choice == correct_door
    }

    /// Play a number of simulations.
    ///
    /// Half of the simulations use the switching strategy, the other half do not.
    ///
    /// ```rust
    /// use monty_pyrs::{MontyHall, Results};
    ///
    /// let mut monty = MontyHall::default();
    /// let results: Results = monty.play_multiple(1_000_000);
    /// ```
    pub fn play_multiple(&mut self, iterations: u64) -> Results {
        let half = iterations / 2;
        let mut results = Results::default();
        for _ in 0..half {
            let switch = true;
            let won = self.play_single(switch);
            if won {
                results.switched.wins += 1;
            } else {
                results.switched.losses += 1;
            }
        }
        for _ in 0..half {
            let switch = false;
            let won = self.play_single(switch);
            if won {
                results.stayed.wins += 1;
            } else {
                results.stayed.losses += 1;
            }
        }
        results
    }
}

impl Default for MontyHall<XorShiftRng> {
    fn default() -> Self {
        Self::new_with_rng(XorShiftRng::seed_from_u64(0))
    }
}

/// A wrapper around [MontyHall::play_multiple] that splits the work by
/// the amount of logical CPUs available.
///
/// ```rust
/// use monty_pyrs::{Results, play_threaded};
/// let results: Results = play_threaded(1_000_000);
/// ```
pub fn play_threaded(iterations: u64) -> Results {
    let threads = num_cpus::get();

    let iterations_per_thread = iterations / threads as u64;
    let mut handles = Vec::with_capacity(threads);
    for _ in 0..threads {
        let iters = iterations_per_thread;
        let mut monty = MontyHall::default();
        handles.push(std::thread::spawn(move || monty.play_multiple(iters)));
    }
    let mut results = Results::default();
    for handle in handles {
        results += handle.join().unwrap();
    }
    results
}

/// Play one billion iterations of the Monty Hall simulation,
/// returning a formatted string with the result.
#[pyfunction]
fn play_one_billion_times() -> PyResult<String> {
    let iterations = 1_000_000_000;
    let results = play_threaded(iterations);
    let (switched_pct, stayed_pct) = results.calc_win_rate();
    Ok(format!(
        "Played {iterations} times, winning {switched_pct:.2}% of the time when switching and {stayed_pct:.2}% times when staying",
        iterations = iterations,
        switched_pct = switched_pct * 100.,
        stayed_pct = stayed_pct * 100.
    ))
}

#[pyfunction]
/// Play a number of iterations of the Monty Hall simulation,
/// returning the [Results]
fn play(iterations: u64) -> PyResult<Results> {
    Ok(play_threaded(iterations))
}

#[pymodule]
fn monty_pyrs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(play_one_billion_times, m)?)?;
    m.add_function(wrap_pyfunction!(play, m)?)?;
    m.add_class::<Results>()?;
    Ok(())
}
