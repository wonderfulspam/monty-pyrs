import monty_pyrs
import monty
from timeit import default_timer as timer

def timed_run(func):
    print("")
    print(f"Executing {func.__name__}")
    print("--------------------------------")
    start = timer()
    func()
    end = timer()
    execution_time = end - start
    print(f"Took {execution_time:.4f}s to run {func.__name__}")

def rust_simple_one_billion_runs():
    """Call Rust and print the resulting string

    No args are passed over the wire and no custom
    datatypes are used.
    """
    string_result = monty_pyrs.play_one_billion_times()
    print(string_result)

def rust_complex_one_billion_runs():
    """Call Rust and do something with the returned value

    The no. of iterations is passed as an argument to a
    Rust function which returns a custom struct. A method
    on the struct is called to get the result which is
    then formatted.
    """
    iterations = 1_000_000_000
    results = monty_pyrs.play(iterations)
    (switched_pct, stayed_pct) = results.calc_win_rate()
    print(f"Played {iterations} times, winning {switched_pct:.2%} of the time \
when switching and {stayed_pct:.2%} times when staying")

timed_run(rust_simple_one_billion_runs)
timed_run(rust_complex_one_billion_runs)
