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

def python_ten_million_runs():
    iterations = 10_000_000
    (switched_pct, stayed_pct) = monty.run(iterations)
    print(f"Played {iterations} times, winning {switched_pct:.2%} of the time \
when switching and {stayed_pct:.2%} times when staying")

timed_run(python_ten_million_runs)
