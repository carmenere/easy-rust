# Send vs. Sync
`Send` vs. `Sync` are both **trait markers**:
- types that implement `Sync` **can** be passed between threads **by shared references**.<br>
- types that implement `Send` **can** be passed between threads **by values**, i.e., can be **moved** across threads.<br>
- types are **neither** `Send` **nor** `Sync` **can't** be passed between threads.<br>

<br>

#### Examples
- Most types are `Send` and `Sync`.<br>
- Some types are `Send`, **but not** `Sync`: 
  - `Cell`
  - `RefCell`
  - `mpsc::Receiver`
- Some types are **neither** `Send` **nor** `Sync`:
  - `JoinHandle`
  - `std::rc::Rc`.<br>
