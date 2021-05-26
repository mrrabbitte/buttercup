# buttercup - small yet brave behavior tree engine written in Rust

**NOTE:** this is very much a WIP project, not accepting contributions yet - although happy to talk if you think it's something worth doing.

## Concept

Behavior Tree is an abstraction widely used in gaming and robotics industry, it provides an easy to comprehend and extendable interface for designing complex behavior. 

Buttercup aims to provide a platform for designing and running agents executing complex behavior in the Web environment. 

Quite awesome book on BTs:
- [Colledanchise, Michele, and Petter Ögren. Behavior trees in robotics and AI: An introduction. CRC Press, 2018.](https://books.google.de/books?hl=pl&lr=&id=YVOWDwAAQBAJ&oi=fnd&pg=PP1&dq=behavior+trees+in+robotics+and+ai&ots=hyCuh4L8lO&sig=HKHCu1tWhEhtf9xo4NfStu-qt1c&redir_esc=y#v=onepage&q=behavior%20trees%20in%20robotics%20and%20ai&f=false)

Handful of papers about Behavior Trees: 
- [Marzinotto, Alejandro, et al. "Towards a unified behavior trees framework for robot control." 2014 IEEE International Conference on Robotics and Automation (ICRA). IEEE, 2014.](https://ieeexplore.ieee.org/abstract/document/6907656)
- [Parameterizing Behavior Trees](https://link.springer.com/chapter/10.1007/978-3-642-25090-3_13)
- [Behavior Trees with Stateful Tasks](https://link.springer.com/chapter/10.1007/978-3-319-17518-8_29)

## State of the project

The project is in first draft stages, so you will see greatly undertested code that will change frequently. 

Please track [the POC project](https://github.com/pgliniecki/buttercup/projects/1) for more details. 

## Behavior Trees Features

Here are some well known features that BTs implementations may have:

- [x] Subtrees
- [x] Blackboards
- [x] Reactive Nodes
- [x] Condition Decorator Nodes
- [ ] Parametrized Trees
- [ ] Stateful Nodes  


## Roadmap

After [the POC](https://github.com/pgliniecki/buttercup/projects/1) we're going to have some more fun, I promise. ;)

The most important part of 1.0 version (https://github.com/pgliniecki/buttercup/projects/2), just after getting the coverage to 80%, is to implement a distributed mode, where each execution can be repeated - with at-least-once execution guarantees - even if a node running a certain agent fails. At the moment looking at https://github.com/async-raft/async-raft, but it may change in the future. 


## Why buttercup?

![Buttercup-pic](https://user-images.githubusercontent.com/7830639/119693062-3ae7e500-be4c-11eb-8347-cd161f877578.png)

C'mon, small yet brave! :)

