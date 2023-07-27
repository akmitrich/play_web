# Play Web
Build Simple Play Server with Rust and Actix Web.
Test it with simple render clients.

## Roadmap
All Data is JSON serde_json::Value. Decline from the classic DOP, the Data are mutable.

- [ ] Play Server
  - [ ] World
    - [ ] Ground
      - [ ] Points
        - [ ] Signs
        - [ ] Lights
      - [ ] Sections
      - [ ] Stations
    - [ ] Rolling
    - [ ] Schedule
  - [ ] Service
  - [ ] Generic Utils
    - [X] get
    - [X] get_mut
    - [X] map
    - [X] set
    - [ ] has