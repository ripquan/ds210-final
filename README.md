# ds210-final
Overview:
This project analyzes the network of subreddits to identify influential communities using graph centrality measures. The primary focus is on:

Closeness Centrality: Measures how close a subreddit is to all other subreddits in the network, highlighting generalist or hub-like communities.

Betweenness Centrality: Quantifies the role of a subreddit as a bridge between other communities, showcasing its ability to connect different parts of the network.

The project uses a directed graph representation of subreddits and their connections, based on hyperlink data provided in a TSV file.

Code Explanation:
1. Data Loading

The program begins by reading the subreddit data from the TSV file (soc-redditHyperlinks-body.tsv). Each row in the file contains information about a hyperlink between two subreddits, including the source subreddit, target subreddit, and other attributes like timestamp and sentiment.

The csv crate is used to parse the file, and the data is stored in a directed graph using the petgraph crate. Each subreddit becomes a node, and each hyperlink becomes a directed edge between two nodes. Additionally, a mapping is maintained to keep track of the nodes corresponding to each subreddit.

2. Graph Representation

The graph is represented using DiGraph from the petgraph crate. This structure allows efficient traversal and manipulation of the subreddit network. The graph's nodes represent subreddits, while the edges represent directed hyperlinks, with optional attributes like weights.

3. Closeness Centrality

The closeness centrality is calculated for each node by measuring the sum of the shortest distances to all other nodes in the graph. This involves:

Running Dijkstra's algorithm from each node to compute shortest paths.

Summing the distances to reachable nodes.

Calculating the centrality score as the inverse of the average distance.

This highlights subreddits that can reach many others with minimal "steps," indicating their centrality in the network.

4. Betweenness Centrality

The betweenness centrality measures the frequency with which a node lies on the shortest path between other pairs of nodes. The process involves:

Using Brandes' algorithm, which efficiently calculates betweenness by:

Performing a breadth-first search (BFS) to find shortest paths from a source node.

Accumulating contributions to centrality scores as the paths are backtracked.

Normalizing the centrality scores based on the size of the graph.

This identifies subreddits that act as "bridges" connecting different clusters of the network.

5. Timing and Performance

The program includes timing functions to measure the performance of key steps like graph construction and centrality calculations. These metrics help identify bottlenecks and optimize the process for large datasets.

6. Output

The program outputs ranked lists of subreddits based on their centrality scores:

Closeness Centrality: Highlights subreddits that are well-connected and can interact with many others quickly.

Betweenness Centrality: Highlights subreddits that act as key connectors or gatekeepers in the network.

More specifically, the program outputs the top 10 subreddits by closeness and betweenness

DATASET DOWNLOAD:

Make sure you download soc-redditHyperlinks-body.tsv from https://snap.stanford.edu/data/soc-RedditHyperlinks.html. The file is too big to include in the repo directly. 