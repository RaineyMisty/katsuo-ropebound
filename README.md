# Sherpa
CS1666 game design game: Katsuo

First manager is Ket-Hwa Cheng (MorningDuringNight KEC344)

Advanced topic group AI: Ket-Hwa (github: MorningDuringNight Pitt: KEC344), 
                         Tingxu (github: RaineyMisty Pitt: ), 
                         Xiaotin (github: XtWang299 Pitt: ), 
                         Zhuoyan (github: qhynSamuel Pitt: )

Advanced topic group multiplayer: Jagger (github: Jagger-Hershey Pitt: ),
                                  Alli (github: AlliBatell Pitt: ),
                                  Sean (github: Sean-Shmulevich Pitt: szs23)

## game premise:

It is a 2D pixel-art platformer where cooperation is necessary. Players control several characters bound by a rope, to conquer the majestic and treacherous slopes of a mysterious mountain, you must master the art of collaborative climbing to reach the top.

The game is a 2d platformer where 2 players are chained together and must make it to the top of the mountain, and must collaborate to make it to the top. The player can at most move left, right, jump, jump left, and jump right and try their best to jump on platforms to ascend the mountain. But be aware there are obstacle that might or might not hinder your progress towards your goal.

Such things that you may encounter are moving platforms, spikes, large distances which require more effort, and even trampolines that may aid your ascent. 

## Advanced topic AI: 
main implementation: AI created should be a deep learning model focused as a cooperative player for platforming, using a behavior tree to solve jumping with 2 or more players. 
### check with team which learning we are doing

## Advanced topic Multiplayer:
main implementation: multiplayer should be a client server model

server should have the same codebase which would be running/ simmulating the game. 

information that will be sent should be player positions, and getting that information flowing for communication between the clients and the server. The server should communicate with client to give other player position and be able to display on the screens where the other player has been moved.

Lag compensation: will be nessicary with a possible 100ms for client to compute values as true.
## midterm goal:

AI - AI can do inputs

Multiplayer - connect to a host clien (client and server can communicate)

maps - 1st map platforms and sidewall collision

rope implementation - rope physics slack

rope implementation - rope physics momentum (stop character from moving)

camera movement - movement up and down follows players

player movement - jump left, right, up

collision detection - AABB  

## Finals goal: 

AI - 2 AIs can climb together - 15%

Multiplayer - at least 2 players can connect to the game and be able to play together. - 15%

completed map - with moving tracking platforms, hazards (spikes) - 15%

Item- collectable coins - 10%

movement - able to jump and walk on platforms - 15%

finish - players finishing the first map achieve a completetion screen - 10%

### Stretch goal

1. Leaderboard across different instance of game

2. wall jumping and/ or fish head collision

3. possible enemines if above 1 or 2 are not accepted
