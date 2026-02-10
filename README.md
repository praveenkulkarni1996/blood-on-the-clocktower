Blood on the clocktower solver.
-------------------------------

[x] Take a log of events.
[x] The log of events should be feedable to solvers.


What is a Event?
----------------

1. Player `Foo` fortune-teller picked players {Bar, Baz} and got YES.


We can combine that two player's logs, and then realize that these are not
compatible. Every player's logs are then marked as trusted or untrusted by
solver programs.
