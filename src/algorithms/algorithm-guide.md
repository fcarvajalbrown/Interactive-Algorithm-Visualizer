# The Pathfinding Lab: Algorithm Guide

This project implements four core pathfinding algorithms in Rust. While they all aim to find a path, they "think" differently and use different data structures to get the job done.

---

## 1. Breadth-First Search (BFS) 
**File:** `bfs.rs` | **Logic:** The "Wave"

Imagine dropping a bucket of water on the start node. The water ripples out in every direction, one square at a time. It only hits the goal when it has touched every closer square first.

* **Data Structure:** `VecDeque` (Queue - FIFO).
* **How it works:** It visits all neighbors at distance 1, then all neighbors at distance 2, and so on.
* **Guarantees Shortest Path?** Yes, but only for **unweighted** grids (where every step costs exactly 1).
* **Downside:** It is "blind." It will explore the complete opposite direction of the goal just as much as the correct direction.



---

## 2. Depth-First Search (DFS)
**File:** `dfs.rs` | **Logic:** The "Stubborn Explorer"

Imagine a mouse in a maze. It picks a path and runs as deep as it can until it hits a wall. Then it backtracks just enough to try a different turn.

* **Data Structure:** `Vec` (Stack - LIFO).
* **How it works:** It dives deep into one branch before checking other neighbors at the same level.
* **Guarantees Shortest Path?** **No.** It finds *any* path, not the best one.
* **Best For:** Maze generation or checking if two points are connected at all.
* **Downside:** It might travel 1,000 steps to reach a goal that was only 2 steps away in the other direction.

---

## 3. Dijkstra's Algorithm
**File:** `dijkstra.rs` | **Logic:** The "Smart Spreader"

A sophisticated BFS that understands "effort." Imagine a map where some roads are highways (cost 1) and others are muddy swamps (cost 10). Dijkstra always chooses the "cheapest" path explored so far.

* **Data Structure:** `BinaryHeap` (Min-Priority Queue).
* **How it works:** It always extracts the node with the lowest cumulative cost from the start.
* **Guarantees Shortest Path?** **Yes**, even on weighted grids with different terrain costs.
* **Downside:** Like BFS, it is still blind. It explores in a perfect circle around the start, even if the goal is clearly to the East.



---

## 4. A* Search (A-Star)
**File:** `astar.rs` | **Logic:** The "Guided Missile"

Dijkstra with a compass. A* calculates the cost already spent ($g$) **plus** an educated guess (heuristic, $h$) of the remaining distance to the goal.

* **Data Structure:** `BinaryHeap` (Min-Priority Queue).
* **The Formula:** $f(n) = g(n) + h(n)$
    * $g(n)$: Distance from start to current node.
    * $h(n)$: Estimated distance (Manhattan or Euclidean) to the goal.
* **Guarantees Shortest Path?** **Yes**, provided the heuristic "guess" never overestimates the actual distance.
* **Best For:** Most modern games and maps. It is significantly faster because it "beams" toward the goal.