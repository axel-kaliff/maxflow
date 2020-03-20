use std::collections::LinkedList;
use std::cmp;
fn main() {


}

fn ford_fulkerson(graph: Vec<Vec<isize>>, V: isize, s: isize, t: isize) -> isize {
    let mut u = 0;
    let mut v = 0;
    let mut rGraph: Vec<Vec<isize>> = Vec::new();

    for i in 0..V {
        u += 1;
        for k in 0..V {
            rGraph[i][k] = graph[i][k];
            v += 1;
        }
    }

    let mut parent: Vec<isize> = Vec::new();
    let mut max_flow = 0;

    while bfs(rGraph, s, t, parent) {
        let mut path_flow = usize::MAX;

        v = t.copy();
        while v != s {


        u = parent[v].copy();
        path_flow = cmp::min(path_flow, rGraph[u][v]);
        
        v = parent[v].copy();
        }


        v = t.copy();
        while v != s {

            u = parent[v].copy();
            rGraph[u][v] -= path_flow;
            rGraph[v][u] += path_flow;

            v = parent[v].copy();
        }

        max_flow += path_flow;
    }

    return max_flow;
}

fn bfs(rGraph: Vec<Vec<isize>>, s: isize, t: isize, parent: Vec<isize>, V: isize) {

    let mut visited: Vec<bool> = Vec::new();

    for 0..V {
        visited.push(false);
    } 

    let mut queue: Vec<isize> = Vec::new();
    queue.push(s);
    visited[s] = true;
//    parent[s] = -1;

    while queue.len() != 0 {
        let u = queue.remove(0);
        for v in 0..V {
            if visited[v] == false && rGraph[u][v] > 0 {
                queue.push(v);
                parent[v] = u;
                visited[v] = true;
            }
        }
    }

    return visited[t] == true;

}
