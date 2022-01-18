import monty_pyrs
from timeit import default_timer as timer

start = timer()
result = monty_pyrs.play_threaded(1_000_000_000)
print(result)
end = timer()
print(end - start)
