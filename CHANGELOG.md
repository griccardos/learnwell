# 0.2 
- Change from only returning valid actions, to returning ALL actions. This makes listing all actions simpler. Instead we now have to filter out invalid actions in the take_action step by awarding them negatively, and not performing the action.
- moved `get_image` into Environment trait. Can be used either to display learning, or for deep learning based on image. Made `&self` mutable so that we can save the pixels to environment struct as it has the same lifetime. 
- move epoch loop to runner, change agent to do only `step`
- added a deep q learning (DQN) example agent. It does not have all the optimisations, but serves to illustrate how it may be done.
- added `DisplayConfig` for display options
- added examples for DQN, including using a backend for Runnt and Tch-rs
- added nnbackend trait to be able to use other neural networks for DQN
