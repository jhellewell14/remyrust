Very early stage of development!


This code runs 100 iterations using default priors for all of the parameters. We simulate 9 generations of infections over 25 days and return the distance from our observation vector (here the value 1 repeated 25 times). There is currently no return from running the code. 
```
remyrust:::r_entry(par_input = list(), iter = 100, 
        ngen = 9, max_time = 25, obs = rep(1L, 25))
```
