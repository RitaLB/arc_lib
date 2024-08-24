Algorithm 2:  find arcs v2 

Input: OL = Truth table output lines ; 
       n_inputs = number of truth table inputs ; 
       nbit_output = index of the current output under evaluation

Output: arcs[ ] 
    
1. arcs <- [ ]
2. n_lines_truth_table <- 2**n_inputs
// foreach input vector (truth table row) 
3. for input_vector <- 0 to (n_lines_truth_table) do
4.  |  arcs.push([ ])
5.  |  for bit <- 0 to n_inputs do 
    |  | // creates a vector with only one bit modified
6.  |  |    result <- num ^ (2** bit)
    |  |   	 
    |  | // Checks if the new input vector ∈ arcs
7.  |  |    if result > num then 
    |  |     | // Checks if output changes
8.  |  |     |       if OL[num][nbit_output] =! OL[result][nbit_output] then
9.  |  |     |       |	 arcs[num].push(result)
    |  |     |       |_
    |  |     |_
    |  |_
    |_
10. return arcs

