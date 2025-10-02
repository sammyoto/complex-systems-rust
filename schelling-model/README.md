<h1>The Schelling Model</h1>
<p>The Schelling Model was created by Thomas Schelling in the early 70's. This model tries to simulate how people of different ethnic backgrounds tend to self-segregate geographically. While this model ignores MANY of the other factors as to why ethnic groups tend to self organize into groups geographically, the algorithm is very simple and provides a good introduction to modeling Agentic Systems.</p>

<p>You can run the program by running:</p>

```
cargo run
```

<p>From the schelling-model directory. A GUI will run on your machine that looks like this:</p>

![Schelling Simulation](/assets/schelling-empty.png)

<p>The default values on each of the sliders should demonstrate the self-organizing nature of the algorithm well enough, but feel free to play with the values and see what happens! Here are a few visuals of what the program looks like at different states:</p>

<p>100 Agents, not yet organized</p>

![Schelling Simulation](/assets/schelling-unorganized.png)

<p>100 Agents, organized</p>

![Schelling Simulation](/assets/schelling-organized.png)

<p>1000 Agents, organized</p>

![Schelling Simulation](/assets/schelling-1000-organized.png)

<h1>Algorithm</h1>
<p> The Algorithm is simple: </p>
<ol>
    <li>Choose a random agent</li>
    <li>Find the agent's neighbors</li>
    <li>Find the ratio of neighbors in the same group as the agent</li>
    <li>If the ratio is less than the moving threshold, move to a random location</li>
    <li>Repeat</li>
</ol>
<p>From this simple algorithm, we can see that self organizing behaviors start to occur. With the correct hyper-parameters, you can easily get results like the image above where 1000 agents self organize into 3 large "neighborhoods" of their own groups.</p>
