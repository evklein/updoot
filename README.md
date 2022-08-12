# :arrow_up: updoot
A tool that displays information from popular web sources like HN, Lobste.rs, Reddit, etc... This code was developed by myself with the express purpose of teaching myself `Rust`.

Updoot is broken into several smaller appliations.

### :lobster: lobster-tree

A tool which scrapes a shortlist of the most recent submissions from <a href="https://lobste.rs/">lobste.rs</a>, then uses the submission data to generate a shortened user tree.
Uses [id-tree](https://github.com/iwburns/id-tree) and [id-tree-layout](https://github.com/jsinger67/id-tree-layout) for SVG generation.

![image](https://user-images.githubusercontent.com/5017975/184385341-ba7bd121-bb21-437e-a24e-7f8845c2a604.png)

**:warning: Note:** There is no publicly available lobste.rs API for a purpose like this, so making even a limited number of requests comes with some difficulties. If you're adapting any code in this repository to do something similar, I don't recommend doing it this way.

