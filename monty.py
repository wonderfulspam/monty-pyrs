from random import randint

def generate_game(n: int):
    game = []

    for _ in range(n):
        doors = [False] * 3
        winner = randint(0, 2) # Choose winning door by random
        doors[winner] = True
        game.append(doors)

    return game


# This is a helper function that takes a list of 3 doors, looks at the second and third door, and then opens the one with goat (i.e. wrong door). This simulates a host with knowledge of what is behind the doors.

def reveal_goat(doors):
    # Get from doors 2 and 3 the one which contains goat.  
    for i in range(1, 3):
        if doors[i] == False:
            return i


# **Simulate random choice**
# 
# Simulate situation where the player randomly chooses whether to keep his initial choice or switch his choice.

def simulate_random_choice(game: list):
    wins = 0
    attempts = 0

    history = []

    for doors in game:
        attempts += 1

        # Host reveals a door with goat.
        goat = reveal_goat(doors)

        # Player randomly chooses whether to keep initial choice or switch.
        new_choice = randint(0, 1)
        final_choice = 0 if new_choice == 0 else 2 if goat == 1 else 1

        if (doors[final_choice] == True):
            wins += 1

        history.append(wins / attempts)

    return wins, history


# **Simulate initial choice**
# 
# Simulate situation where the player *only* keeps his initial choice and never switches.

def simulate_keep_choice(game: list):
    wins = 0
    attempts = 0

    for doors in game:
        attempts += 1
        
        # User does not switch game.
        if (doors[0] == True):
            wins += 1

    return wins


# **Simulate switch choice**
# 
# Simulate situation where the player switches his choice everytime.

def simulate_switch_choice(game: list):
    wins = 0
    attempts = 0

    for doors in game:
        attempts += 1
        
        # Host reveals a door with goat.
        goat = reveal_goat(doors)

        # Player switches his doors (here he chooses the non-opened doors).
        new_choice = 1 if goat == 2 else 2

        if (doors[new_choice] == True):
            wins += 1

    return wins

def run(iterations: int):
    game = generate_game(iterations // 2)
    switched_wins = simulate_switch_choice(game)
    stayed_wins = simulate_keep_choice(game)
    return switched_wins / iterations * 2, stayed_wins / iterations * 2
