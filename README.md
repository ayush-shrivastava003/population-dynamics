The simulator has been rewritten in Python. I will continue to use this until I can ensure the simulator can consistently work, and tweak settings to increase the longevity and stability of both populations.

Pre-existing code that was written in Rust is now in the `rs/` folder (it has been modified since the last commit).

New code can be found in the `py/` folder. `sim_no_square.py` is the recommended simulator and is the one I'm using for testing It essentially gets rid of the squares and just makes the board one big square. I have also added visualization using `pandas` and `matplotlib`. Graphs will be generated in the `plots/` folder.