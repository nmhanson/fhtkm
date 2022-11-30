# fhtkm
Ain't it funny how the knight moves?

Based on https://www.funnyhowtheknightmoves.com/ which is based on a chess puzzle which was shared by GM Ben Finegold.

TODO:
1. Finish move rules.
2. Add game ending parameters.
3. Display move counter.
4. Time
5. Add ncurses interface (🤮)

# Controls:
uiopjkl; each of the 8 possible knight moves (left-up, up-left, up-right, right-up, etc.)
remember the horsey moves in an L shape.

# Goal of the game:
Ben Finegold can explain it better: https://www.youtube.com/watch?v=SrQlpY_eGYU
Reach every square (in order) by moving the knight as fast as possible with as few moves as possible. The order is right-left, top-bottom.
You may not move to a square if it is
1. Attacked by the Queen in the center.
2. Occupied by the Queen in the center.



# It's pretty ugly
<img width="434" alt="image" src="https://user-images.githubusercontent.com/22331869/201496388-9353d402-9884-4fc8-a6de-9644f3b03c50.png">
