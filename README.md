# buttercup - small and brave behavior tree engine written in Rust

**NOTE:** this is very much a WIP project, not accepting contributions yet - although happy to talk if you think it's something worth doing.

## Concept

Behavior Tree is an abstraction widely used in gaming and robotics industry, it provides an easy to comprehend and extendable interface for designing complex behavior. 

Buttercup aims to provide a platform for designing and running agents executing complex behavior in the Web environment. It has to work, right? ;) 

Handful of links about Behavior Trees: 
- https://ieeexplore.ieee.org/abstract/document/6907656
- https://link.springer.com/chapter/10.1007/978-3-642-25090-3_13
- https://link.springer.com/chapter/10.1007/978-3-319-17518-8_29

## State of the project

The project is in first draft stages, so you will see greatly undertested code that will change frequently. 

Please track: https://github.com/pgliniecki/buttercup/projects/1 for more details. 

## Roadmap

After the POC (https://github.com/pgliniecki/buttercup/projects/1) we're going to have some more fun, I promise. ;)

The most important part of 1.0 version (https://github.com/pgliniecki/buttercup/projects/2), apart from testing, is to implement a distributed mode, where each execution can be repeated (at-least-once) even if a node running a certain agent fails, at the moment looking at https://github.com/async-raft/async-raft, but it may change in the future. 
