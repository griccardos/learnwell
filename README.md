# learnwell
Easy reinforcement learning framework, allowing you to quickly create Environments and test them.  
Aims to be simple  
Minimal external dependencies
Framework to create your own implementations

Implementation examples
- Q-Learning
- Deep Q Learning (DQN)

The state of this project is in alpha.
Use at your own risk. 

## Getting started

See the taxi example and walk through the comments
`cargo run --release --example taxi`
you can also run the following examples:

- `hike` - runs with display
- `taxi` 
- `mouse` 
- `mouseimage` - DQN 
- `taxiimage`  - DQN, runs with display

Imports:
```rust
use learnwell::{
    runner::Runner, 
    agent::qlearning::QLearning, 
    environment::{Environment, EnvironmentDisplay}
    strategy::decliningrandom::DecliningRandom, 
    };
```

We then ask the `Runner` to run the agent for `x` number of epochs


Allows 2 modes:
 - `Runner::run` for normal operation
 - `Runner::run_with_display` to create a window and display image which gets updated as it runs

For example:
```rust
    Runner::run(
        QLearning::new(0.1, 0.98, DecliningRandom::new(epochs, 0.01)), //Agent
        TaxiEnvironment::default(), //Environment
        400, //epochs
    );
```
or 

```rust
Runner::run_with_display(
        QLearning::new(0.2, 0.99,DecliningRandom::new(epochs, 0.005) ), //Agent
        Hike::new(), //Environment
        700_000, //epochs
        10 //frames per second to refresh image
    );
```

## We need:
- Environment - this is the game/scenario we want to learn
- Agent - this is what interacts with the environment
 
 
## We implement a few things to run
### Environment
 1. `State` Struct - this is what we base our actions on
 2. `Action` (normally enum) - these are the actions we perform
 3. Environment Struct that implements the `Environment<S,A>` trait and depends on the `State` and `Action`. The Environment struct should hold the state, because we will refer to it later
### Agent
 4. the Agent algorithm (e.g. QLearning),
 

## Implementation:
Note we derive Hash, Eq, PartialEq and Clone for both `State` and `Action`
### State
```rust
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct TaxiState {
    taxi: Point,
    dropoff: Point,
    passenger: Point, 
    in_taxi: bool,
}
```

### Action
```rust
#[derive(Clone, Hash, PartialEq, Eq)]
pub enum TaxiAction {
    Up,
    Down,
    Left,
    Right,
    Dropoff,
    Pickup,
}
```

### Environment
```rust
pub struct TaxiEnvironment {
    state: TaxiState, //this is the actual state that gets saved in the qtable
    found: usize, //just a helper. there could be a few other items you want to track in the environment
}
```


## Status
- [X] implement Qlearning
- [X] implement deep qlearning
- [ ] move optional functionality to features (e.g. display, fxhasher)
