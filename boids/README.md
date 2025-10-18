<h1>Boids</h1>
<p>Boids were first introduced by a computer scientist Craig Reynolds in the late 1980's. Reynolds came up with a set of simple behavioral rules for agents moving in a continuous space that can reproduce an amazingly natural looking flock behavior for birds. His model was called bird-oids or "Boids" for short. Boids are also used to model how schools of fish move or how swarms of insects move.</p>

<p>You can run the program by running:</p>

```
cargo run
```

<p>From the boids directory. A GUI will run on your machine that looks like this:</p>

![Boids](/assets/boids-empty.png)

<p>The default values on each of the sliders should demonstrate the algorithm well enough. However if you do play with the sliders you can get some very interesting (buggy) results. Here are a few visuals of the program at different states:</p>

<p>100 Agents</p>

![Boids](/assets/boids-100.png)

<p>1000 Agents, Weird Pattern</p>

![Boids](/assets/boids-weird-pattern.png)

<p>The more agents you add the stranger some of the patterns that emerge become. However, be warned this program is not optimized for TOO many agents. After 1000 agents the program can begin to run quite slow.</p>

<h1>Algorithm</h1>
<p> The Boids algorithm is a bit more complex than the other two: </p>
<ol>
    <li>Find all agent's neighbors</li>
    <li>Find the neighbors center of mass (compute a velocity from agent to center)</li>
    <li>Find the neighbors average velocity</li>
    <li>Find a repulsion velocity away from neighbors (closer neighbors repulse more)</li>
    <li>Add all velocities to current velocity</li>
    <li>Update all agents</li>
</ol>
<p>From this algorithm we can observe that patterns similar to that of flocks of birds or schools of fish form. The agents stay near each other and move as a group. There are DEFINITELY some optimizations needed and perhaps some more rules to keep the group from straying from the world boundaries, however just from these rules we can mimic to a certain degree how boids behave.</p>