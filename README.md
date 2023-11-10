# bucket-queue
A bucket queue implementation in the rust programming language. 

# Bucket Queues
Bucket queues are a specialized priority queue data structure that work well for monotonous, integer data with some maximum increment. 
Their most common application are for Dijkstra's algorithm for shortest paths, but they can be applied to any problem where:
- keys are integers
- keys are monotonous, meaning after the smallest key was extracted, no key that's smaller than the previous minimum can be added
- there is a constant (small) range of values that can be added to the queue

This is true for Dijkstra on Graphs with integer edge weights with a maximum edge weight of $C$, because:
- edge weights are integers
- if a value is pushed to the queue, it will have the value of the previous minimum plus some edge weight
- that edge weight will be smaller than or equal to $C$

# Space Complexity
Because of the simple nature of bucket queues, the whole data structure always takes O(C) memory.
None of the operations change the amount of memory allocated.

# Time Complexity
For a maximum increment of $C$, the operations have the following (worst case) runtime complexities:
Operation | Complexity
---|---
`insert` | $O(1)$
`remove` | $O(1)$
`decrease_key` | $O(1)$
`get_min` | $O(C)$
`pop_min` | $O(C)$

Also note that these time complexities are all "real" upper bounds for worst-case time complexity, not amortized.
This is better than anything a general purpose priority queue can achieve.
Using a bucket queue, Dijkstra's algorithm can be implemented in $O(|E| + |V|C)$ time.

## Dijkstra Complexity
Here a comparison of some common priority queue data structures and the time complexity dijkstra has using them:

Priority Queue Type | Dijkstra Complexity
---|---
Binary Heap | $O((\|E\| + \|V\|) \log \|V\|)$
Fibonacci Heap | $O(\|E\| + \|V\| \log \|V\|)$
Bucket Queue | $O(\|E\| + \|V\|C)$
Radix Queue | $O(\|E\| + \|V\| \log C$
