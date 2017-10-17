# tbd (To be delivered)

tbd is a network simulator, with the ambitious but likely impractical goal of being able to simulate every part of a classic network, sending a packet from host to host across an imaginary internet.

Planned features include (but are not limited to):
- DHCP and DNS
- TCP and UDP implementations
- Simulated hosts
- Support for user defined hosts to send data into the network
- Support for user defined protocols (maybe?)
- L3 routing (link state, DV, PV, etc)
- L2 routing (STP with learning switches, maybe some new-er protocols)
- ARP
- and more! (I probably forgot something important)

tbd does its best to ensure that your packets will be delivered at some time after when they were sent, and at some time before or at the death of the universe. If for some reason tbd doesn't deliver your packet before the universe dies, it doesn't matter, because you'll be gone too! Fate sharing!

###### These efforts are rendered void if the speed of light is exceeded or if you survive the death of the universe.

tbd is not intended for use as anything but a toy to play around with. It is a pet project which has the primary purpose of helping me learn Rust, just because I want to. Due to its role as a learning tool for myself, I will not be accepting major pull requests to add features at this time (this may change as the project becomes more complete), but bug fixes and/or minor changes will be considered.
