# ¿Quien es el MechaBurro?

An entry for the [first Bevy game jam][jam] following the theme of "Unfair Advantage." It was made in a week using the wonderful [Bevy game engine][bevy]. You can play the game [here][itch], rate it in the jam [here][rate] and check out the other submissions [here][submissions].

¿Quien es el MechaBurro? is a local multiplayer game with bots inspired by twin-stick shooters, aspects of mario kart and a card game I played with my family growing up called "Burro." 

Players choose to play as one of 8 different burro piñatas and then attempt to be the last burro standing in each level of the game. Burros can shoot candy in just the cardinal directions but are able to move independently from where they're aiming. When hit, burros will flash momentairly and be invulnurable to damage but will also be unable to shoot. Shot candies have a limited range before disappearing and burros have a cooldown period before being able to shoot again.

At the start of each round, one burro is chosen randomly to be upgraded to the Mechaburro. The Mechaburro shoots lasers which travel faster, have a much larger range and also experience a shorter cooldown between shots.

The round ends when only one burro remains or 5 seconds after all human players have lost. Points are based on order of elimination with the first eliminated getting 0 points and each burro after getting 1 more than the previously eliminiated burro. Points are accumulated across rounds and the burro with the most points at the end of the game is declared the winner.

Check out my other games [here][othergames]. Also, I'm always hanging out in the [bevy discord][bevy-discord], definitely feel free to @ramirezmike me and ask questions or criticize me :)


https://user-images.githubusercontent.com/1421719/156888152-78e98f83-8fd9-477f-be48-6857a1cfe1ec.mp4


# Running the Game

To run the game locally

```
cargo run --release 
```

Font:
Mexican Tequila - Vladimir Nikolic

Music:
Baila Mi Cumbia - Jimmy Fontanez/Media Right Productions


[jam]: https://itch.io/jam/bevy-jam-1
[bevy]: https://bevyengine.org/
[itch]: https://ramirezmike2.itch.io/quien-es-el-mechaburro 
[rate]: https://itch.io/jam/bevy-jam-1/rate/1423622
[submissions]: https://itch.io/jam/bevy-jam-1/entries
[othergames]: https://ramirezmike2.itch.io/
[bevy-discord]: https://discord.gg/bevy
