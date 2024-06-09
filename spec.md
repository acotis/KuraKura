## Model
```rs
struct Player {
  id: String, // A random GUID
  name: String, // Something they type in
}

struct Room {
  id: String, // A random GUID, part of the invite link
  host: Player,
  guest: Option<Player>,  // If None, we're waiting for them to join
  game: Option<Game>,  // If None, we're waiting to start a new game
}

struct ServerState {
  rooms: HashMap<String, Room>,  // maps Room IDs to Rooms
}
```

## Initialization
```mermaid
sequenceDiagram
Note over Host: Let user pick name NH
Host --> Server: Open WebSocket
Host ->> Server: makeUser()
Note over Server: Create a user U
Server ->> Host: U
Host ->> Server: [auth: U] setName(NH)
Note over Server: Set player's name
Server ->> Host: OK
Host ->> Server: [auth: U] makeRoom()
Note over Server: Create a room R
Server ->> Host: R
Host ->> Server: [auth: U] joinRoom(R)
Server ->> Host: roomState(R)
Note over Host: Render a link to copy
Host -->> Guest: "Let's play! https://kurakura.io/join?room=R"
Note over Guest: Let user pick name NG
Guest --> Server: Open WebSocket
Guest ->> Server: makeUser()
Note over Server: Create a user U
Server ->> Guest: U
Guest ->> Server: [auth: U] setName(NG)
Note over Server: Set player's name
Server ->> Guest: OK
Guest ->> Server: [auth: U] joinRoom(R)
Server ->> Guest: roomState(R)
```

## Play loop
```mermaid
sequenceDiagram
Player ->> Server: play(PlayerID, R, Move)
Note over Server: Check that PlayerID is<br>a player in room R
Note over Server: Apply Move
Server ->> Player: roomState(R)
Server ->> Opponent: roomState(R)
```

## Thoughts
* Make it resilient to closing the browser window:
  * **Client:** Persist player GUID to LocalStorage. Reuse instead of generating a new one
  * **Server:** Make joinRoom work if the ID matches a player already in the room
  * **Server:** Always respond with the entire room/game state (it's tiny anyway)

