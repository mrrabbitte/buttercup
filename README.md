# buttercup - small yet brave behavior tree engine written in Rust

**NOTE:** this is very much a WIP project, not accepting contributions yet - although happy to talk if you think it's something worth doing.

## Concept

Behavior Tree is an abstraction widely used in gaming and robotics industry, it provides an easy to comprehend and extendable interface for designing complex behavior. 

Buttercup aims to provide a platform for designing and running agents which can execute a complex behavior in the Web environment. 

So instead of using actuators or performing actions in a game you can send emails, scrape the web, perform http requests, etc. 

Quite awesome book on BTs - basis for the ongoing implementation:
- [Colledanchise, Michele, and Petter Ögren. Behavior trees in robotics and AI: An introduction. CRC Press, 2018.](https://books.google.de/books?hl=pl&lr=&id=YVOWDwAAQBAJ&oi=fnd&pg=PP1&dq=behavior+trees+in+robotics+and+ai&ots=hyCuh4L8lO&sig=HKHCu1tWhEhtf9xo4NfStu-qt1c&redir_esc=y#v=onepage&q=behavior%20trees%20in%20robotics%20and%20ai&f=false)

Handful of papers about Behavior Trees: 
- [Marzinotto, Alejandro, et al. "Towards a unified behavior trees framework for robot control." 2014 IEEE International Conference on Robotics and Automation (ICRA). IEEE, 2014.](https://ieeexplore.ieee.org/abstract/document/6907656)
- [Shoulson, Alexander, et al. "Parameterizing behavior trees." International conference on motion in games. Springer, Berlin, Heidelberg, 2011.](https://link.springer.com/chapter/10.1007/978-3-642-25090-3_13)
- [Klöckner, Andreas. "Behavior trees with stateful tasks." Advances in Aerospace Guidance, Navigation and Control. Springer, Cham, 2015. 509-519.](https://link.springer.com/chapter/10.1007/978-3-319-17518-8_29)

## State of the project

The project is in first draft stages, so you will see greatly undertested code that will change frequently. 

Please track [the POC project](https://github.com/pgliniecki/buttercup/projects/1) for more details. 

## Behavior Trees Features

Here are some well known features that BTs implementations may have:

- [x] Subtrees
- [x] Blackboards
- [x] Reactive Nodes
- [x] Condition Decorator Nodes
- [ ] Parametrized Trees and Subtrees
- [ ] Stateful Nodes  


## Roadmap

After [the POC](https://github.com/pgliniecki/buttercup/projects/1) we're going to have some more fun.

The most important part of [the 1.0 version](https://github.com/pgliniecki/buttercup/projects/2) is getting the test coverage to 80% and a stable API.

After that, it would be grand if there was a distributed mode, where each execution can be repeated, e.g. with at-least-once execution guarantees, so even if a node running a certain agent fails, another can pick up almost where the other ended and continue the execution. 

This would allow to run a swarm of agents which could be orchestrated with or without a centralised source of truth with quite decent guarantees.  

At the moment looking at (Raft)[https://github.com/async-raft/async-raft] in a multi-raft variant, where agents would be partitioned and each partition would have 1 node selected as the leader but it's an early concept. 

## Potential use cases

Some visions on how this project can be used and specialize in the future, mentioned in order from most probable to most adventurous:

- **Pentesting**: BTs allows for designing clear and auditable scenarios, where groups of agents can coordinate parts of the attack, e.g. on group hammers a given service for others to abuse the downtime in other parts of the system;
- **IoT**: the engine is written in Rust and will run on whatever can execute `async-await` so potentially you could design BTs on the server and send those to connected devices;
- **Web3**: fast and lean implementation could provide desired latencies for crypto use cases.

## Why buttercup?

![Buttercup-pic](https://user-images.githubusercontent.com/7830639/119693062-3ae7e500-be4c-11eb-8347-cd161f877578.png)

C'mon, small yet brave! :)

