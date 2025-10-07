<h1>Diffusion Limited Aggregation</h1>
<p>Very similar to Schelling's Model, Diffusion Limited Aggregation allows only two types of particles, Stationary and Movable particles.
The stationary particles are stationary, they can't move. The movable particles move around randomly until a stationary particle gets close enough, then the movable particle
becomes stationary.</p>

<p>You can run the program by running:</p>

```
cargo run
```

<p>From the diffusion-limited-aggregation directory. A GUI will run on your machine that looks like this:</p>

![Diffusion Limited Aggregation](/assets/dla-empty.png)

<p>The default values on each of the sliders should demonstrate the self-organizing nature of the algorithm well enough, but feel free to play with the values and see what happens! Here are a few visuals of what the program looks like at different states:</p>

<p>100 Agents, not yet organized</p>

![Diffusion Limited Aggregation](/assets/dla-100-unorganized.png)

<p>100 Agents, organized</p>

![Diffusion Limited Aggregation](/assets/dla-100-organized.png)

<p>1000 Agents, organized</p>

![Diffusion Limited Aggregation](/assets/dla-1000-organized.png)

<p>The results can be vastly different for different neighborhood radius, however if you want a small neighborhood radius be prepared to wait for a long time until the system is organized.</p>

<h1>Algorithm</h1>
<p> The Algorithm is simple: </p>
<ol>
    <li>Choose a random agent</li>
    <li>Find the agent's neighbors</li>
    <li>Find out if any of the Agent's neighbors is stationary</li>
    <li>If there is a stationary neighbor, become stationary, if not move to a random location</li>
    <li>Repeat</li>
</ol>
<p>This simple algorithm can be very sensitive to hyperparameters. If you have enough agents and small enough neighborhood size you can get veru interesting patterns to appear.</p>