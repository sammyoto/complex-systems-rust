# Complex Systems Exercises
Going through some exercises in Introduction to the Modeling and Analysis of Complex Systems by Hiroki Sayama. You can find the textbook on [Open Texbook Library](https://open.umn.edu/opentextbooks/textbooks/233). All code is written in Rust mainly for my own practice with the language. More detailed descriptions of each of the models are in their respective folders' READMEs, however brief summaries are provided below.

<h3>The Schelling Model</h3>
<p> This model is as simple as it gets for complex systems. The model aims to understand how groups tend to separate automatically. It uses simple rules to determine whether an agent (member of the population) will move locations or not. The end result is often the self organization of neighborhoods of the same group type.</p>

![Schelling Simulation](/assets/schelling-1000-organized.png)

<h3>Diffusion Limited Aggregation</h3>
<p> This is another simple model for complex systems. With minimal code changes to Schelling's Model, we can create a system where agents aggregate over time and create complex patterns from simple rules.</p>

![Diffusion Limited Aggregation](/assets/dla-100-organized.png)

<h3>Boids</h3>
<p> Boids are more complex than the previous two models. Boids (Bird-oids) aim to mimic the motion of flocks of birds or schools of fish.</p>